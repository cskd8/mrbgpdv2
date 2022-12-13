use crate::packets::open::OpenMessage;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Event {
    ManualStart,
    TcpConnectionEstablished,
    TcpConnectionConfirmed,
    BgpOpen(OpenMessage),
}
