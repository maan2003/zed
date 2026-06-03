use std::io::{BufReader, BufWriter};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};

use anyhow::{Context as _, Result, anyhow};
use tau_proto::{
    ClientKind, EventSelector, Frame, FrameReader, FrameWriter, Hello, Message, PROTOCOL_VERSION,
    Subscribe,
};

pub(crate) type Writer = Arc<Mutex<FrameWriter<BufWriter<UnixStream>>>>;

pub(crate) enum SocketEvent {
    Frame(Frame),
    Disconnected(String),
}

pub(crate) fn spawn(socket_path: PathBuf, tx: mpsc::Sender<SocketEvent>) -> Result<Writer> {
    let stream = UnixStream::connect(&socket_path)
        .with_context(|| format!("failed to connect to {}", socket_path.display()))?;
    let read_stream = stream.try_clone().context("failed to clone socket")?;
    let writer = Arc::new(Mutex::new(FrameWriter::new(BufWriter::new(stream))));

    send_frame(
        &writer,
        &Frame::Message(Message::Hello(Hello {
            protocol_version: PROTOCOL_VERSION,
            client_name: "tau-gui".into(),
            client_kind: ClientKind::Ui,
        })),
    )?;
    send_frame(
        &writer,
        &Frame::Message(Message::Subscribe(Subscribe {
            selectors: vec![
                EventSelector::Prefix("ui.".to_owned()),
                EventSelector::Prefix("session.".to_owned()),
                EventSelector::Prefix("provider.".to_owned()),
                EventSelector::Prefix("tool.".to_owned()),
                EventSelector::Prefix("extension.".to_owned()),
                EventSelector::Prefix("agent.".to_owned()),
                EventSelector::Prefix("harness.".to_owned()),
                EventSelector::Prefix("shell.".to_owned()),
                EventSelector::Prefix("term.".to_owned()),
            ],
        })),
    )?;

    std::thread::spawn(move || {
        let mut reader = FrameReader::new(BufReader::new(read_stream));
        loop {
            match reader.read_frame() {
                Ok(Some(frame)) => {
                    if tx.send(SocketEvent::Frame(frame)).is_err() {
                        return;
                    }
                }
                Ok(None) => {
                    if tx
                        .send(SocketEvent::Disconnected("eof".to_owned()))
                        .is_err()
                    {
                        return;
                    }
                    return;
                }
                Err(error) => {
                    if tx
                        .send(SocketEvent::Disconnected(error.to_string()))
                        .is_err()
                    {
                        return;
                    }
                    return;
                }
            }
        }
    });

    Ok(writer)
}

pub(crate) fn send_frame(writer: &Writer, frame: &Frame) -> Result<()> {
    let mut writer = writer
        .lock()
        .map_err(|_| anyhow!("socket writer mutex poisoned"))?;
    writer.write_frame(frame).map_err(|error| anyhow!(error))?;
    writer.flush().context("failed to flush socket frame")
}
