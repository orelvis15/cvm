use crate::config::remote_config::RemoteConfig;
use crate::context::storage::Storage;
use crate::Term;

#[derive(Default)]
pub struct Context{
    pub term: Term,
    pub storage: Storage,
    pub remote_config: RemoteConfig
}