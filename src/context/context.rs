use crate::storage::Storage;
use crate::Term;

#[derive(Default)]
pub struct Context{
    pub term: Term,
    pub env: Storage
}