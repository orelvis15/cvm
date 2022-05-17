use clap::ArgMatches;
use crate::{Error, Success};
use crate::commands::config::Args;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::config::config::get_config;
use crate::task::task_impl::check_update_task::{CheckUpdateData, CheckUpdateTask};
use crate::task::task_impl::use_version_task::UserVersionTask;
use crate::task::task_manager;

pub fn start(command: &ArgMatches, current_version: String) -> Result<Success, Error> {

    let config = get_config();
    if let Err(error) = config {
        return Result::Err(error);
    }

    task_manager::start(vec![
        Box::new(CheckUpdateTask { input_data: CheckUpdateData { version: current_version } }),
    ])
}