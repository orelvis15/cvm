#![allow(dead_code, unused_variables)]

use std::fmt;
use std::fmt::Formatter;
use crate::task::task_impl::install_dependences_task::InstallDependenciesOutputData;
use crate::task::task_impl::install_haskell_ghc_task::{InstallHanskellGhcOutputData};
use crate::task::task_impl::run_command_task::RunCommandOutputData;

pub enum Env{
    Empty(),
    RunCommnad(RunCommandOutputData),
    InstallDependences(InstallDependenciesOutputData),
    InstallHaskellGhc(InstallHanskellGhcOutputData)
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Env::Empty() => write!(f, "empty"),
            Env::RunCommnad(..) => write!(f, "run_command"),
            Env::InstallDependences(..) => write!(f, "install_dependences"),
            Env::InstallHaskellGhc(..) => write!(f, "install_hanskell_ghc"),
        }
    }
}