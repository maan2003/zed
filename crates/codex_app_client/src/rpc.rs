use std::{
    any::Any,
    borrow::Cow,
    collections::HashMap,
    fmt,
    future::Future,
    rc::Rc,
    sync::{
        Arc, Mutex,
        atomic::{AtomicI64, Ordering},
    },
};

use codex_app_protocol as protocol;
use futures::{
    AsyncBufReadExt as _, AsyncRead, AsyncWrite, AsyncWriteExt as _, FutureExt as _,
    StreamExt as _,
    channel::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    future::LocalBoxFuture,
    io::BufReader,
    select_biased,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, value::RawValue};

pub type RequestId = protocol::RequestId;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Error {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Error {
    pub fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn data(mut self, data: impl Into<Value>) -> Self {
        self.data = Some(data.into());
        self
    }

    pub fn parse_error() -> Self {
        Self::new(-32700, "Parse error")
    }

    pub fn invalid_request() -> Self {
        Self::new(-32600, "Invalid request")
    }

    pub fn method_not_found() -> Self {
        Self::new(-32601, "Method not found")
    }

    pub fn invalid_params() -> Self {
        Self::new(-32602, "Invalid params")
    }

    pub fn internal_error() -> Self {
        Self::new(-32603, "Internal error")
    }

    pub fn into_internal_error(error: impl std::error::Error) -> Self {
        Self::internal_error().data(error.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Request<Params> {
    pub id: RequestId,
    pub method: Arc<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Response<ResponseValue> {
    Result { id: RequestId, result: ResponseValue },
    Error { id: RequestId, error: Error },
}

impl<ResponseValue> Response<ResponseValue> {
    fn new(id: RequestId, result: Result<ResponseValue>) -> Self {
        match result {
            Ok(result) => Self::Result { id, result },
            Err(error) => Self::Error { id, error },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Notification<Params> {
    pub method: Arc<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum OutgoingMessage<Local: Side, Remote: Side> {
    Request(Request<Remote::InRequest>),
    Response(Response<Local::OutResponse>),
    Notification(Notification<Remote::InNotification>),
}

#[derive(Debug, Serialize)]
struct JsonRpcMessage<M> {
    jsonrpc: &'static str,
    #[serde(flatten)]
    message: M,
}

impl<M> JsonRpcMessage<M> {
    fn wrap(message: M) -> Self {
        Self {
            jsonrpc: "2.0",
            message,
        }
    }
}

#[derive(Debug)]
pub struct RpcConnection<Local: Side, Remote: Side> {
    outgoing_tx: UnboundedSender<OutgoingMessage<Local, Remote>>,
    pending_responses: Arc<Mutex<HashMap<RequestKey, PendingResponse>>>,
    next_id: AtomicI64,
}

#[derive(Debug)]
struct PendingResponse {
    deserialize: fn(&RawValue) -> Result<Box<dyn Any + Send>>,
    respond: oneshot::Sender<Result<Box<dyn Any + Send>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum RequestKey {
    String(String),
    Int(i64),
}

impl RequestKey {
    fn from_request_id(request_id: &RequestId) -> Self {
        match request_id {
            RequestId::String(value) => Self::String(value.clone()),
            RequestId::Int64(value) => Self::Int(*value),
        }
    }
}

impl<Local, Remote> RpcConnection<Local, Remote>
where
    Local: Side + 'static,
    Remote: Side + 'static,
{
    pub fn new<Handler>(
        handler: Handler,
        outgoing_bytes: impl Unpin + AsyncWrite,
        incoming_bytes: impl Unpin + AsyncRead,
        spawn: impl Fn(LocalBoxFuture<'static, ()>) + 'static,
    ) -> (Self, impl Future<Output = Result<()>>)
    where
        Handler: MessageHandler<Local> + 'static,
    {
        let (incoming_tx, incoming_rx) = mpsc::unbounded();
        let (outgoing_tx, outgoing_rx) = mpsc::unbounded();

        let pending_responses = Arc::new(Mutex::new(HashMap::default()));

        let io_task = {
            let pending_responses = pending_responses.clone();
            async move {
                let result = Self::handle_io(
                    incoming_tx,
                    outgoing_rx,
                    outgoing_bytes,
                    incoming_bytes,
                    pending_responses.clone(),
                )
                .await;

                if let Ok(mut pending_responses) = pending_responses.lock() {
                    pending_responses.clear();
                } else {
                    log::error!("failed to clear pending responses: lock poisoned");
                }

                result
            }
        };

        Self::handle_incoming(outgoing_tx.clone(), incoming_rx, handler, spawn);

        let this = Self {
            outgoing_tx,
            pending_responses,
            next_id: AtomicI64::new(0),
        };

        (this, io_task)
    }

    pub fn notify(
        &self,
        method: impl Into<Arc<str>>,
        params: Option<Remote::InNotification>,
    ) -> Result<()> {
        self.outgoing_tx
            .unbounded_send(OutgoingMessage::Notification(Notification {
                method: method.into(),
                params,
            }))
            .map_err(|_| Error::internal_error().data("failed to send notification"))
    }

    pub async fn request<Out: DeserializeOwned + Send + 'static>(
        &self,
        method: impl Into<Arc<str>>,
        params: Option<Remote::InRequest>,
    ) -> Result<Out> {
        let method = method.into();
        let (tx, rx) = oneshot::channel();
        let id = RequestId::Int64(self.next_id.fetch_add(1, Ordering::SeqCst));
        let key = RequestKey::from_request_id(&id);

        {
            let mut pending_responses = self.pending_responses.lock().map_err(|error| {
                Error::internal_error().data(format!("failed to lock pending responses: {error}"))
            })?;

            pending_responses.insert(
                key.clone(),
                PendingResponse {
                    deserialize: |value| {
                        serde_json::from_str::<Out>(value.get())
                            .map(|out| Box::new(out) as _)
                            .map_err(Error::into_internal_error)
                    },
                    respond: tx,
                },
            );
        }

        if self
            .outgoing_tx
            .unbounded_send(OutgoingMessage::Request(Request { id: id.clone(), method, params }))
            .is_err()
        {
            if let Ok(mut pending_responses) = self.pending_responses.lock() {
                pending_responses.remove(&key);
            } else {
                log::error!("failed to clean pending response after send failure: lock poisoned");
            }
            return Err(Error::internal_error().data("failed to send request"));
        }

        let response = rx
            .await
            .map_err(|_| Error::internal_error().data("server shut down unexpectedly"))??;

        let value = response
            .downcast::<Out>()
            .map_err(|_| Error::internal_error().data("failed to deserialize response"))?;
        Ok(*value)
    }

    async fn handle_io(
        incoming_tx: UnboundedSender<IncomingMessage<Local>>,
        mut outgoing_rx: UnboundedReceiver<OutgoingMessage<Local, Remote>>,
        mut outgoing_bytes: impl Unpin + AsyncWrite,
        incoming_bytes: impl Unpin + AsyncRead,
        pending_responses: Arc<Mutex<HashMap<RequestKey, PendingResponse>>>,
    ) -> Result<()> {
        let mut input_reader = BufReader::new(incoming_bytes);
        let mut outgoing_line = Vec::new();
        let mut incoming_line = String::new();

        loop {
            select_biased! {
                message = outgoing_rx.next() => {
                    if let Some(message) = message {
                        outgoing_line.clear();
                        serde_json::to_writer(&mut outgoing_line, &JsonRpcMessage::wrap(message.clone()))
                            .map_err(Error::into_internal_error)?;
                        log::trace!("send: {}", String::from_utf8_lossy(&outgoing_line));
                        outgoing_line.push(b'\n');
                        outgoing_bytes
                            .write_all(&outgoing_line)
                            .await
                            .map_err(Error::into_internal_error)?;
                    } else {
                        break;
                    }
                }
                bytes_read = input_reader.read_line(&mut incoming_line).fuse() => {
                    if bytes_read.map_err(Error::into_internal_error)? == 0 {
                        break;
                    }
                    log::trace!("recv: {}", incoming_line.trim_end());

                    match serde_json::from_str::<RawIncomingMessage<'_>>(&incoming_line) {
                        Ok(message) => {
                            if let Some(id) = message.id {
                                if let Some(method) = message.method {
                                    match Local::decode_request(&method, message.params) {
                                        Ok(request) => {
                                            if incoming_tx.unbounded_send(IncomingMessage::Request { id, request }).is_err() {
                                                return Err(Error::internal_error().data("request channel closed"));
                                            }
                                        }
                                        Err(error) => {
                                            outgoing_line.clear();
                                            let error_response = OutgoingMessage::<Local, Remote>::Response(
                                                Response::Error { id, error },
                                            );
                                            serde_json::to_writer(
                                                &mut outgoing_line,
                                                &JsonRpcMessage::wrap(error_response),
                                            )
                                            .map_err(Error::into_internal_error)?;
                                            outgoing_line.push(b'\n');
                                            outgoing_bytes
                                                .write_all(&outgoing_line)
                                                .await
                                                .map_err(Error::into_internal_error)?;
                                        }
                                    }
                                } else {
                                    let key = RequestKey::from_request_id(&id);
                                    let pending_response = match pending_responses.lock() {
                                        Ok(mut pending_responses) => pending_responses.remove(&key),
                                        Err(error) => {
                                            log::error!("failed to lock pending responses: {error}");
                                            None
                                        }
                                    };

                                    if let Some(pending_response) = pending_response {
                                        if let Some(result_value) = message.result {
                                            let result = (pending_response.deserialize)(result_value);
                                            if pending_response.respond.send(result).is_err() {
                                                log::warn!("failed to deliver successful response");
                                            }
                                        } else if let Some(error) = message.error {
                                            if pending_response.respond.send(Err(error)).is_err() {
                                                log::warn!("failed to deliver error response");
                                            }
                                        } else {
                                            let null_raw_value = RawValue::from_string("null".into())
                                                .map_err(Error::into_internal_error)?;
                                            let result = (pending_response.deserialize)(&null_raw_value);
                                            if pending_response.respond.send(result).is_err() {
                                                log::warn!("failed to deliver null response");
                                            }
                                        }
                                    } else {
                                        log::error!("received response for unknown request id: {id}");
                                    }
                                }
                            } else if let Some(method) = message.method {
                                match Local::decode_notification(&method, message.params) {
                                    Ok(notification) => {
                                        if incoming_tx
                                            .unbounded_send(IncomingMessage::Notification { notification })
                                            .is_err()
                                        {
                                            return Err(Error::internal_error().data("notification channel closed"));
                                        }
                                    }
                                    Err(error) => {
                                        log::error!("failed to decode notification: {error}");
                                    }
                                }
                            } else {
                                log::error!("received message with neither id nor method");
                            }
                        }
                        Err(error) => {
                            log::error!("failed to parse incoming message: {error}. Raw: {}", incoming_line.trim_end());
                        }
                    }

                    incoming_line.clear();
                }
            }
        }

        Ok(())
    }

    fn handle_incoming<Handler: MessageHandler<Local> + 'static>(
        outgoing_tx: UnboundedSender<OutgoingMessage<Local, Remote>>,
        mut incoming_rx: UnboundedReceiver<IncomingMessage<Local>>,
        handler: Handler,
        spawn: impl Fn(LocalBoxFuture<'static, ()>) + 'static,
    ) {
        let spawn = Rc::new(spawn);
        let handler = Rc::new(handler);

        spawn({
            let spawn = spawn.clone();
            async move {
                while let Some(message) = incoming_rx.next().await {
                    match message {
                        IncomingMessage::Request { id, request } => {
                            let outgoing_tx = outgoing_tx.clone();
                            let handler = handler.clone();
                            spawn(
                                async move {
                                    let result = handler.handle_request(request).await;
                                    if outgoing_tx
                                        .unbounded_send(OutgoingMessage::Response(Response::new(
                                            id, result,
                                        )))
                                        .is_err()
                                    {
                                        log::error!("failed to send request response");
                                    }
                                }
                                .boxed_local(),
                            );
                        }
                        IncomingMessage::Notification { notification } => {
                            let handler = handler.clone();
                            spawn(
                                async move {
                                    if let Err(error) =
                                        handler.handle_notification(notification).await
                                    {
                                        log::error!("failed to handle notification: {error}");
                                    }
                                }
                                .boxed_local(),
                            );
                        }
                    }
                }
            }
            .boxed_local()
        });
    }
}

#[derive(Debug, Deserialize)]
struct RawIncomingMessage<'a> {
    id: Option<RequestId>,
    #[serde(borrow)]
    method: Option<Cow<'a, str>>,
    #[serde(borrow)]
    params: Option<&'a RawValue>,
    #[serde(borrow)]
    result: Option<&'a RawValue>,
    error: Option<Error>,
}

#[derive(Debug)]
enum IncomingMessage<Local: Side> {
    Request {
        id: RequestId,
        request: Local::InRequest,
    },
    Notification {
        notification: Local::InNotification,
    },
}

pub trait Side: Clone {
    type InRequest: Clone + Serialize + DeserializeOwned + 'static;
    type InNotification: Clone + Serialize + DeserializeOwned + 'static;
    type OutResponse: Clone + Serialize + DeserializeOwned + 'static;

    fn decode_request(method: &str, params: Option<&RawValue>) -> Result<Self::InRequest>;

    fn decode_notification(method: &str, params: Option<&RawValue>) -> Result<Self::InNotification>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerRequest {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Clone, Default, Debug)]
pub struct ClientSide;

impl Side for ClientSide {
    type InRequest = ServerRequest;
    type InNotification = protocol::ServerNotification;
    type OutResponse = Value;

    fn decode_request(method: &str, params: Option<&RawValue>) -> Result<Self::InRequest> {
        let parsed_params = match params {
            Some(params) => Some(
                serde_json::from_str(params.get())
                    .map_err(|error| Error::invalid_params().data(error.to_string()))?,
            ),
            None => None,
        };

        Ok(ServerRequest {
            method: method.to_string(),
            params: parsed_params,
        })
    }

    fn decode_notification(method: &str, params: Option<&RawValue>) -> Result<Self::InNotification> {
        let mut value = serde_json::Map::new();
        value.insert("method".to_string(), Value::String(method.to_string()));
        if let Some(params) = params {
            let params = serde_json::from_str(params.get())
                .map_err(|error| Error::invalid_params().data(error.to_string()))?;
            value.insert("params".to_string(), params);
        }

        serde_json::from_value(Value::Object(value))
            .map_err(|error| Error::invalid_params().data(error.to_string()))
    }
}

#[derive(Clone, Default, Debug)]
pub struct ServerSide;

impl Side for ServerSide {
    type InRequest = Value;
    type InNotification = Value;
    type OutResponse = Value;

    fn decode_request(_method: &str, params: Option<&RawValue>) -> Result<Self::InRequest> {
        let Some(params) = params else {
            return Ok(Value::Null);
        };

        serde_json::from_str(params.get()).map_err(|error| Error::invalid_params().data(error.to_string()))
    }

    fn decode_notification(_method: &str, params: Option<&RawValue>) -> Result<Self::InNotification> {
        let Some(params) = params else {
            return Ok(Value::Null);
        };

        serde_json::from_str(params.get()).map_err(|error| Error::invalid_params().data(error.to_string()))
    }
}

pub trait MessageHandler<Local: Side> {
    fn handle_request(
        &self,
        request: Local::InRequest,
    ) -> impl Future<Output = Result<Local::OutResponse>>;

    fn handle_notification(
        &self,
        notification: Local::InNotification,
    ) -> impl Future<Output = Result<()>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_escaped_method_name() {
        let json = r#"{"jsonrpc":"2.0","id":1,"method":"thread\/status\/changed","params":{}}"#;
        let parsed: RawIncomingMessage<'_> =
            serde_json::from_str(json).expect("raw incoming message should parse");
        assert_eq!(
            parsed.method.expect("method should exist"),
            "thread/status/changed"
        );
    }

    #[test]
    fn parses_unescaped_method_name() {
        let json = r#"{"jsonrpc":"2.0","id":2,"method":"thread/status/changed","params":{}}"#;
        let parsed: RawIncomingMessage<'_> =
            serde_json::from_str(json).expect("raw incoming message should parse");
        assert_eq!(
            parsed.method.expect("method should exist"),
            "thread/status/changed"
        );
    }
}
