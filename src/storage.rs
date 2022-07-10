#![allow(dead_code, unused_variables)]

#[derive(Default)]
pub struct Storage {
    pub data: Vec<Data>
}

pub enum Data{
    Empty(),
}