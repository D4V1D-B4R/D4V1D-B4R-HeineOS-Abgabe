/*
 * All system calls are routed here via the IDT syscall handler (interrupt 0x80).
 * The system call number is passed in the rax register and is used as an index into
 * the syscall table to call the corresponding function.
 *
 * Author: Stefan Lankes, RWTH Aachen University
 *         Licensed under the Apache License, Version 2.0 or MIT license, at your option.
 *
 *         Michael Schoettner, Heinrich Heine University Duesseldorf, 2024-10-23
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2025-10-15
 */

use core::arch::{naked_asm};
use crate::syscall::functions::hello::sys_hello_world;
use crate::syscall::user_api::SyscallFunction;

/// Global syscall function table.
static SYSCALL_TABLE: SyscallFunctionTable = SyscallFunctionTable::new();

/// Struct to hold the syscall function pointers.
#[repr(align(64))]
#[repr(C)]
struct SyscallFunctionTable {
    table: [*const u64; SyscallFunction::NumSyscalls as usize],
}

impl SyscallFunctionTable {
    pub const fn new() -> SyscallFunctionTable {
        SyscallFunctionTable {
            table: [
                sys_hello_world as *const u64
            ],
        }
    }
}

unsafe impl Send for SyscallFunctionTable {}
unsafe impl Sync for SyscallFunctionTable {}

/// System call dispatcher.
/// This function is called from the IDT syscall handler (interrupt 0x80).
/// The syscall number is passed in the rax register and is used as an index into
/// the syscall table to call the corresponding function.
/// All registers (except rax) are saved and restored.
/// The syscall function's return value is passed back in rax.
#[unsafe(naked)]
pub extern "C" fn dispatch_syscall() {
    naked_asm!(
        // TODO: Save all registers (except rax, which contains the syscall number)

        //save registers on stack (copy from thread.rs)
        "push r8", //general purpose regs
        "push r9", //...
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        //"push rax", //accumulator
        "push rbx", //base
        "push rcx", //counter
        "push rdx", //data (extends 'a' reg)
        "push rsi", //string src
        "push rdi", //string dest -> parametr for kickoff !!!
        "push rbp", //base pointer for stack (stays the same for stackframe; rsp is the moving one)
        //"pushfq", //status reg rflags

        // Call syscall handler (or syscall_abort for an invalid syscall number)
        "cmp rax, {NUM_SYSCALLS}",
        "jge syscall_abort",
        "call [{SYSCALL_TABLE} + rax * 8]",

        // TODO: Restore all registers (except rax)

        //load the same stack, since rsp was untouched (copy from thread.rs)
        //"popfq", //status reg rflags
        "pop rbp", //base pointer for stack (stays the same for stackframe; rsp is the moving one)
        "pop rdi", //string dest -> parametr for kickoff !!!
        "pop rsi", //string src
        "pop rdx", //data (extends 'a' reg)
        "pop rcx", //counter
        "pop rbx", //base
        //"pop rax", //accumulator
        "pop r15", //general purpose regs
        "pop r14", //...
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",

        // Return from interrupt
        "iretq",

        NUM_SYSCALLS = const SyscallFunction::NumSyscalls as usize,
        SYSCALL_TABLE = sym SYSCALL_TABLE
    )
}

/// This function is called if an invalid syscall number is passed.
/// It panics and halts the system.
#[unsafe(no_mangle)]
extern "C" fn syscall_abort() {
    panic!("Invalid syscall number");
}