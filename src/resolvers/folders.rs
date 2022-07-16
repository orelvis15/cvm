
pub enum FolderResolvers {
    TMP
}

impl FolderResolvers {
    pub fn get_string(&self) -> String {
        match &self {
            FolderResolvers::TMP => {
                String::from(std::env::temp_dir().to_str().unwrap())
            }
        }
    }
}