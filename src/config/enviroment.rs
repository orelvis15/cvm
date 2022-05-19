use std::fs;
use std::path::Path;

use serde::{Serialize, Deserialize};
use crate::config::config::{get_project_dir};
use crate::task::message_type::MessageType;

use crate::task::task::{Message, Success};

pub fn get_env() -> Result<Enviroment, Message> {
    const FILE_NAME: &str = "env.tom";

    let project_dir = get_project_dir();
    if let Err(error) = project_dir {
        return Err(error);
    }

    let config_dir = project_dir.unwrap().config_dir().to_str().unwrap().to_string();

    // obtenemos el path del directorio como string y lo concatenamos con  el archivo
    let mut path_str: String = config_dir.clone();
    path_str.push_str("/");
    path_str.push_str(FILE_NAME);

    //comprobamos que existe el archivo de configuracion
    let exist_config = Path::new(&path_str).exists();

    if !exist_config {
        let file = Enviroment { ..Default::default() };
        let tom_string = toml::to_string(&file).unwrap();
        let creation = fs::write(path_str, tom_string);

        if let Err(_) = creation {
            return Err(Message {
                code: 0,
                message: "Error creating enviroment file".to_string(),
                kind: MessageType::Error,
                task: "".to_string(),
                stack: vec![],
            });
        }
    }

    // leemos el archivo de configuracion
    let env_file = fs::read_to_string(
        format!("{}/{}", config_dir.clone(), FILE_NAME)
    );

    // si fue correcta la lectura se devuelve el objeto Config parseado del archivo
    return match env_file {
        Ok(file) => {
            if let Ok(env) = toml::from_str(&file) {
                Ok(env)
            } else {
                Err(Message {
                    code: 0,
                    message: "Error try parsing enviroment file".to_string(),
                    kind: MessageType::Error,
                    task: "".to_string(),
                    stack: vec![],
                })
            }
        }
        Err(_) => Err(Message {
            code: 0,
            message: "Error try reading enviroment file".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![],
        }),
    };
}

pub fn set_env(env: Enviroment) -> Result<Success, Message> {
    const FILE_NAME: &str = "env.tom";

    let current_env = get_env();
    if let Err(error) = current_env {
        return Err(error);
    };

    let update = current_env.unwrap().update(env);

    let project_dir = get_project_dir();
    if let Err(error) = project_dir {
        return Err(error);
    };

    let config_dir = project_dir.unwrap().config_dir().to_str().unwrap().to_string();

    // obtenemos el path del directorio como string y lo concatenamos con  el archivo
    let mut path_str: String = config_dir;
    path_str.push_str("/");
    path_str.push_str(FILE_NAME);

    let toml_string = toml::to_string(&update);
    let creation = fs::write(path_str, toml_string.unwrap());

    if let Err(error) = creation {
        return Err(Message {
            code: 0,
            message: "Error creating enviroment file".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![error.to_string()],
        })
    }else {
        Ok(Success{})
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Enviroment {
    pub cnode_home: String,
    pub ghcup_home: String,
    pub cabal_home: String,
    pub ghc_home: String,
    pub libsodium_path: String,
    pub pkg_path: String,
    pub active_version: String,
}

impl Enviroment {
    fn update (mut self, env: Enviroment) -> Enviroment{
        if !env.cnode_home.is_empty() {self.cnode_home = env.cnode_home};
        if !env.cabal_home.is_empty() {self.cabal_home = env.cabal_home};
        if !env.ghc_home.is_empty() {self.ghc_home = env.ghc_home};
        if !env.ghcup_home.is_empty() {self.ghcup_home = env.ghcup_home};
        if !env.active_version.is_empty() {self.active_version = env.active_version};
        self
    }
    fn to_string(self: Self){
        println!("cnode: {}\ncabal: {}\nghcup: {}\nghc: {}\n", self.cnode_home, self.cabal_home, self.ghcup_home, self.ghc_home)
    }
}