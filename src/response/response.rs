use std::{
    any::{self, type_name, type_name_of_val}, collections::VecDeque, time::Instant
};

#[derive(Debug)]
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


pub enum DATATYPE {
    String(Response<String>),
    List(Response<VecDeque<String>>),    
}

pub struct RedisObject<'a, DATATYPE> {
    data_type: &'a str,
    value: DATATYPE,
}
