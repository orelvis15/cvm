#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystemFolder {
    UnixOpt
}

impl SystemFolder {
    pub fn get_path_string(&self) -> String {
        return match &self {
            SystemFolder::UnixOpt => { "/opt".to_string() }
        };
    }
}