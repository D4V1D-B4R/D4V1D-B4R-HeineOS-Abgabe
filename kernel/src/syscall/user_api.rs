/*
 * Module: user_api
 *
 * Description: All system calls available to user programs are defined in this module.
 *
 * Author: Stefan Lankes, RWTH Aachen University
 *         Licensed under the Apache License, Version 2.0 or MIT license, at your option.
 *
 *         Michael Schoettner, Heinrich Heine University Duesseldorf, 14.09.2023
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 15.10.2025
 */

use core::arch::asm;

use crate::device::key::KeyEvent;

/// System call numbers available to user programs.
#[repr(u64)]
pub enum SyscallFunction {
    ExitThread,
    GetThreadID,
    HelloWorld,
    KeyEventSysC,
    PrintString,
    SysTime,
    YieldThread,
    NumSyscalls // Last entry to count number of syscalls
}

/// Test system call printing "Hello, World!" to the serial console.
pub fn usr_hello_world() {
    unsafe { syscall0(SyscallFunction::HelloWorld); }
}

// TODO: Implement further system calls
pub fn usr_thread_exit() {
    unsafe { syscall0(SyscallFunction::ExitThread); }
}

pub fn usr_get_thread_id() -> u64 {
    unsafe { syscall0(SyscallFunction::GetThreadID) }
}

pub fn usr_get_key_event() -> Option<KeyEvent>{
    let mut event = KeyEvent::new();
    let event_ptr = &mut event as *mut KeyEvent as u64;

    let mut was_avaible = 0;

    unsafe { was_avaible = syscall1(SyscallFunction::KeyEventSysC, event_ptr) }
    if was_avaible != 0 {
        return Some(event);
    }
    else {
        None
    }
}

pub fn usr_print(msg: &str) {
    unsafe { 
        syscall2(
            SyscallFunction::PrintString,
            msg.as_ptr() as u64,
            msg.len() as u64
        );
    }
}

pub fn usr_get_system_time() -> u64 {
    unsafe { syscall0(SyscallFunction::SysTime) }
}

pub fn usr_thread_yield() {
    unsafe { syscall0(SyscallFunction::YieldThread); }
}

/// Perform a system call with 0 parameters.
#[inline(always)]
pub unsafe fn syscall0(syscall: SyscallFunction) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
        "int 0x80",
        inlateout("rax") syscall as u64 => ret,
        options(preserves_flags, nostack)
        );
    }
    ret
}
// TODO: Implement system call wrapper functions with parameters

/// Perform a system call with 1 parameter.
#[inline(always)]
pub unsafe fn syscall1(syscall: SyscallFunction, arg1: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
        "int 0x80",
        inlateout("rax") syscall as u64 => ret,
        in("rdi") arg1,
        options(preserves_flags, nostack)
        );
    }
    ret
}

/// Perform a system call with 2 parameters.
#[inline(always)]
pub unsafe fn syscall2(syscall: SyscallFunction, arg1: u64, arg2: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
        "int 0x80",
        inlateout("rax") syscall as u64 => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        options(preserves_flags, nostack)
        );
    }
    ret
}