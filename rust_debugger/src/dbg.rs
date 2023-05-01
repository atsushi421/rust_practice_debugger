use crate::helper::DynError;
use nix::{
    libc::user_regs_struct,
    sys::{
        personality::{self, Persona},
        ptrace,
        wait::{waitpid, WaitStatus},
    },
    unistd::{execvp, fork, ForkResult, Pid},
};
use std::ffi::{c_void, CString};

todo!();
