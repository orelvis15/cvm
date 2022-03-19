use clap::ArgMatches;
use crate::{Error, Success};
use crate::commands::config::Args;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::config::config::get_config;
use crate::task::task_impl::use_version_task::UserVersionTask;
use crate::task::task_manager;

pub fn start(command: &ArgMatches) -> Result<Success, Error> {

    let config = get_config();
    if let Err(error) = config {
        return Result::Err(error);
    }

    let mut version: String = LATEST.to_string();

    match command.value_of(Args::VERSION.to_string()) {
        Some(value) => {
            if !value.is_empty() {
                if verify_version(value) || version == LATEST {
                    version = value.to_string()
                } else {
                    return Result::Err(Error {
                        code: 0,
                        message: "The version is not well formed".to_string(),
                        task: "".to_string(),
                        stack: vec![],
                    });
                }
            }
        }
        None => {}
    };

    if version == LATEST {
        let last_tag = get_last_tag(config.unwrap().build_cardano_node.cnode_release);
        match last_tag {
            Ok(tag) => version = tag,
            Err(error) => return Result::Err(error)
        }
    };

    task_manager::start(vec![
        Box::new(UserVersionTask { version }),
    ])
}