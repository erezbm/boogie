mod args;
mod attach;

use std::{
    ffi::c_void,
    io::{stdin, stdout, Write}, thread::sleep, time::Duration,
};

use args::Args;
use attach::attach;
use clap::Parser;
use nix::sys::ptrace::{self, AddressType};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let pid = attach(&args)?;

    print!("> ");
    stdout().flush()?;
    for line in stdin().lines() {
        println!("{}", line?);

        print!("> ");
        stdout().flush()?;
        // let mask : usize = (0xCC << usize::) & usize::MAX;
        //println!("{}", ptrace::read(pid, 0x0000555555555149 as AddressType)?);
        let regs = ptrace::getregs(pid)?;
        println!("{:#?}",regs.rip);
        let res = ptrace::read(pid, regs.rip as AddressType)?;
        println!("{:x}", res);
        ptrace::step(pid, None)?;
        sleep(Duration::from_secs(5));
        println!("{:#?}",ptrace::getregs(pid)?);
        // let cc = &mut (0xCC as usize);
        // unsafe { ptrace::write(pid, 0x0000555555555149 as AddressType, (cc as *mut _) as *mut c_void) }?;
        // ptrace::cont(pid, None)?;
    }

    Ok(())
}
