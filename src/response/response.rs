use std::time::Instant;

pub struct Response<T> {
    pub value: T,
    pub expiry: Option<Instant>,
}

impl<T> Response<T> {
    pub fn new(val: T, exp: Option<Instant>) -> Self {
        Self {
            value: val,
            expiry: exp,
        }
    }
}
