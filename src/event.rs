use crate::packets::{
    open::OpenMessage,
    keepalive::KeepaliveMessage,
};

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Event {
    ManualStart,
    TcpConnectionEstablished,
    TcpConnectionConfirmed,
    BgpOpen(OpenMessage),
    KeepAliveMsg(KeepaliveMessage),
}
