use crate::header::Header;

pub enum Frame {
    Data(Vec<u8>),
    Headers(Vec<Header>),
    Priority,
    RstStream,
    Settings,
    PushPromise,
    Ping,
    GoAway,
    WindowUpdate,
    Continuation,
}