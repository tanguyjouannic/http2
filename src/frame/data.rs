pub struct Data {
    pub payload: Vec<u8>,
}

impl From<Frame> for Data {
    fn from(frame: Frame) -> Self {
        Data {
            payload: frame.payload,
        }
    }
}