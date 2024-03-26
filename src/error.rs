use std::result::Result as Result_;

pub type Result<T> = Result_<T, Error>;

#[derive(Debug)]
pub struct Error(pub &'static str);
