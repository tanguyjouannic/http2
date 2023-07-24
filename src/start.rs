/// HTTP/2 connection preface sequence.
///
/// In HTTP/2, each endpoint is required to send a connection preface as
/// a final confirmation of the protocol in use and to establish the
/// initial settings for the HTTP/2 connection. The client and server
/// each send a different connection preface. The client connection preface
/// starts with the string "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n".
pub const HTTP2_CONNECTION_PREFACE_SEQUENCE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
