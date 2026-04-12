use std::{future::Future, sync::Arc};

use codex_app_protocol as protocol;
use futures::{AsyncRead, AsyncWrite, future::LocalBoxFuture};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

mod rpc;

pub use rpc::{ClientSide, Error, MessageHandler, Result, ServerRequest, ServerSide};
use rpc::RpcConnection;

#[async_trait::async_trait(?Send)]
pub trait Client {
    async fn server_request(&self, _request: ServerRequest) -> Result<Value> {
        Err(Error::method_not_found())
    }

    async fn server_notification(&self, notification: protocol::ServerNotification) -> Result<()>;
}

impl<C: Client> MessageHandler<ClientSide> for C {
    fn handle_request(
        &self,
        request: ServerRequest,
    ) -> impl Future<Output = Result<Value>> {
        async move { self.server_request(request).await }
    }

    fn handle_notification(
        &self,
        notification: protocol::ServerNotification,
    ) -> impl Future<Output = Result<()>> {
        async move { self.server_notification(notification).await }
    }
}

#[derive(Debug)]
pub struct CodexAppClient {
    conn: RpcConnection<ClientSide, ServerSide>,
}

impl CodexAppClient {
    pub fn new(
        client: impl MessageHandler<ClientSide> + 'static,
        outgoing_bytes: impl Unpin + AsyncWrite,
        incoming_bytes: impl Unpin + AsyncRead,
        spawn: impl Fn(LocalBoxFuture<'static, ()>) + 'static,
    ) -> (Self, impl Future<Output = Result<()>>) {
        let (conn, io_task) = RpcConnection::new(client, outgoing_bytes, incoming_bytes, spawn);
        (Self { conn }, io_task)
    }

    pub async fn request_json(
        &self,
        method: impl Into<Arc<str>>,
        params: Option<Value>,
    ) -> Result<Value> {
        self.conn.request(method, params).await
    }

    pub fn notify_json(&self, method: impl Into<Arc<str>>, params: Option<Value>) -> Result<()> {
        self.conn.notify(method, params)
    }

    pub async fn thread_start(
        &self,
        params: protocol::ThreadStartParams,
    ) -> Result<protocol::ThreadStartResponse> {
        self.request_typed("thread/start", params).await
    }

    pub async fn thread_read(
        &self,
        params: protocol::ThreadReadParams,
    ) -> Result<protocol::ThreadReadResponse> {
        self.request_typed("thread/read", params).await
    }

    pub async fn thread_resume(
        &self,
        params: protocol::ThreadResumeParams,
    ) -> Result<protocol::ThreadResumeResponse> {
        self.request_typed("thread/resume", params).await
    }

    pub async fn turn_start(
        &self,
        params: protocol::TurnStartParams,
    ) -> Result<protocol::TurnStartResponse> {
        self.request_typed("turn/start", params).await
    }

    pub async fn turn_interrupt(
        &self,
        params: protocol::TurnInterruptParams,
    ) -> Result<protocol::TurnInterruptResponse> {
        self.request_typed("turn/interrupt", params).await
    }

    pub async fn thread_set_name(
        &self,
        params: protocol::ThreadSetNameParams,
    ) -> Result<protocol::ThreadSetNameResponse> {
        self.request_typed("thread/name/set", params).await
    }

    async fn request_typed<TParams, TResult>(
        &self,
        method: &str,
        params: TParams,
    ) -> Result<TResult>
    where
        TParams: Serialize,
        TResult: DeserializeOwned + Send + 'static,
    {
        let params = serde_json::to_value(params)
            .map_err(|error| Error::invalid_params().data(error.to_string()))?;
        self.conn.request(method.to_string(), Some(params)).await
    }
}
