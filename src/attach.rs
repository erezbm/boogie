use crate::args::Args;
use anyhow::Context;
use log::debug;
use nix::{
    sys::{ptrace::traceme, wait::waitpid},
    unistd::{execv, fork, ForkResult, Pid},
};
use std::{ffi::CString, path::Path};

pub fn attach(args: &Args) -> anyhow::Result<Pid> {
    spawn(&args.executable)
}

pub fn spawn(executable: &Path) -> anyhow::Result<Pid> {
    let executable = &CString::new(
        executable
            .to_str()
            .context(format!("Could not determine {executable:#?} path"))?,
    )?;

    match unsafe { fork() }? {
        ForkResult::Parent { child } => {
            debug!("Child pid: {}", child);
            waitpid(child, None)?;
            Ok(child)
        }
        ForkResult::Child => {
            traceme()?;
            execv(executable, &[executable])?;
            unreachable!();
        }
    }
}
