/*
 * Contains functions to create, start, switch and end threads.
 *
 * Author: Michael Schoettner, Heinrich Heine University Duesseldorf, 2023-05-15
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-15
 * License: GPLv3
 */

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::{fmt, ptr};
use core::arch::naked_asm;
use core::fmt::Display;
use core::sync::atomic::AtomicUsize;
use crate::consts::STACK_SIZE;
use crate::device::cpu;
use crate::thread::scheduler::scheduler;

static THREAD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn next_id() -> usize {
    THREAD_ID_COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst)
}

/// Low-level routine for starting a thread.
#[unsafe(naked)]
unsafe extern "C" fn thread_start(stack_ptr: usize) {
    naked_asm!(
        //switch to stack addr ("stack_ptr" in rdi)
        "mov rsp, rdi",

        //unlock the scheduler
        "call unlock_scheduler",

        //write stack into registers
        "popfq", //status reg rflags
        "pop rbp", //base pointer for stack (stays the same for stackframe; rsp is the moving one)
        "pop rdi", //string dest -> parametr for kickoff !!!
        "pop rsi", //string src
        "pop rdx", //data (extends 'a' reg)
        "pop rcx", //counter
        "pop rbx", //base
        "pop rax", //accumulator
        "pop r15", //general purpose regs
        "pop r14", //...
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",

        //return to kickoff
        "ret"
    )
}

/// Low-level routine for switching to the next thread.
/// `current_stack_ptr` is a pointer to `stack_ptr` of the next coroutine (where the rsp is saved).
/// `next_stack` is the value of `stack_ptr` of the next thread (the new rsp value).
/// `next_stack_end` points to the top of the next thread's stack (used to set the rsp0 entry of the TSS)
#[unsafe(naked)]
unsafe extern "C" fn thread_switch(current_stack_ptr: *mut usize, next_stack: usize, next_stack_end: usize) {
    naked_asm!(
        //save registers on stack
        "push r8", //general purpose regs
        "push r9", //...
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "push rax", //accumulator
        "push rbx", //base
        "push rcx", //counter
        "push rdx", //data (extends 'a' reg)
        "push rsi", //string src
        "push rdi", //string dest -> parametr for kickoff !!!
        "push rbp", //base pointer for stack (stays the same for stackframe; rsp is the moving one)
        "pushfq", //status reg rflags

        //save rsp ("*current_stack_ptr" in rdi)
        "mov [rdi], rsp",

        "mov rdi, rdx", //move parameter from rdx to rdi
        "call tss_set_rsp0", //set kernel stack in tss

        
        //go to next rsp ("next_stack" in rsi)
        "mov rsp, rsi",

        //unlock the scheduler
        "call unlock_scheduler",

        //load other stack
        "popfq", //status reg rflags
        "pop rbp", //base pointer for stack (stays the same for stackframe; rsp is the moving one)
        "pop rdi", //string dest -> parametr for kickoff !!!
        "pop rsi", //string src
        "pop rdx", //data (extends 'a' reg)
        "pop rcx", //counter
        "pop rbx", //base
        "pop rax", //accumulator
        "pop r15", //general purpose regs
        "pop r14", //...
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        
        //return to next coroutine
        "ret"
    )
}

/*
 * CAUTION: This is not meant as a replacement for your existing thread.rs
 *          It just contains additional code that you should copy into your own file.
 */

// Low-level routine for storing `rsp0` in the current TSS
unsafe extern "C" {
    fn tss_set_rsp0(rsp0: usize);
}

#[unsafe(naked)]
/// Called by `switch_to_usermode()`.
/// Switches to the prepared user stack and uses `iret` to return to ring 3 code.
unsafe extern "C" fn thread_user_start(stack_ptr: usize) {
    naked_asm!(
        "mov rsp, rdi", // Switch stack
        "pop rdi",
        "iretq" // Return to user mode
    )
}

/// Represents a thread in the system.
/// It contains the stack and the entry function.
/// Threads must be registered in the scheduler and are run automatically once the scheduler is started.
#[repr(C)]
pub struct Thread {
    id: usize,
    is_kernel_thread: bool,
    kernel_stack: Vec<u64>, // Memory for the kernel stack
    user_stack: Vec<u64>, // Memory for the user stack
    stack_ptr: usize, // Pointer on the stack to the saved context
    entry: fn(),
}

impl Thread {
    /// Create a new user thread with the given entry function.
    pub fn new_kernel_thread(entry: fn()) -> Box<Thread> {
        // Allocate memory for the kernel stack and initialize it to zero
        let mut kernel_stack = Vec::<u64>::with_capacity(STACK_SIZE / 8);
        for _ in 0..kernel_stack.capacity() {
            kernel_stack.push(0);
        }

        // Allocate memory for the user stack and initialize it to zero
        let mut user_stack = Vec::<u64>::with_capacity(STACK_SIZE / 8);
        for _ in 0..user_stack.capacity() {
            user_stack.push(0);
        }

        // Set the stack pointer to the top of the kernel stack
        let stack_ptr = ptr::from_ref(&kernel_stack[kernel_stack.capacity() - 1]) as usize;

        let is_kernel_thread = true;

        // Create a new thread object
        let mut thread = Box::new(
            Thread { id: next_id(), is_kernel_thread, kernel_stack, user_stack, stack_ptr, entry }
        );

        // Prepare the stack for the thread so it can be started via `thread_start()`
        thread.prepare_kernel_stack();
        thread
    }

    /// Create a new user thread with the given entry function.
    pub fn new_user_thread(entry: fn()) -> Box<Thread> {
        // Allocate memory for the kernel stack and initialize it to zero
        let mut kernel_stack = Vec::<u64>::with_capacity(STACK_SIZE / 8);
        for _ in 0..kernel_stack.capacity() {
            kernel_stack.push(0);
        }

        // Allocate memory for the user stack and initialize it to zero
        let mut user_stack = Vec::<u64>::with_capacity(STACK_SIZE / 8);
        for _ in 0..user_stack.capacity() {
            user_stack.push(0);
        }

        // Set the stack pointer to the top of the kernel stack (begnins as kernel stack)
        let stack_ptr = ptr::from_ref(&kernel_stack[user_stack.capacity() - 1]) as usize;

        let is_kernel_thread = false;

        // Create a new thread object
        let mut thread = Box::new(
            Thread { id: next_id(), is_kernel_thread, kernel_stack, user_stack, stack_ptr, entry }
        );

        // Prepare the stack for the thread so it can be started via `thread_start()`
        thread.prepare_kernel_stack();
        thread
    }

    /// Start the thread.
    /// This function is only once by the scheduler.
    /// The scheduler does further thread switching via `switch()`.
    pub fn start(&mut self) {
        unsafe { thread_start(self.stack_ptr); }
    }

    /// Switch from the `current` thread to the `next` thread.
    /// This function is called by the scheduler to switch between threads.
    pub unsafe fn switch(current: *mut Thread, next: *mut Thread) {
        unsafe { thread_switch(
            &mut (*current).stack_ptr as *mut usize,
            (*next).stack_ptr,
             Self::get_top_of_stack(&(*next).kernel_stack) as usize
            );
        }
    }

    /// Get the ID of the thread.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Prepare the stack of a newly created thread in a way that it can be used
    /// to return to the 'kickoff' function with the thread itself as parameter.
    /// The prepared stack is used in 'thread_start' to start the first thread.
    /// Other threads are started by 'thread_switch' with the prepared stack.
    fn prepare_kernel_stack(&mut self) {
        let kickoff = (Thread::kickoff_kernel_thread as *const()) as u64;
        let thread = ptr::from_mut(self) as u64;
        let length = self.kernel_stack.len();

        self.kernel_stack[length - 1] = kickoff; // Address of 'kickoff'
        self.kernel_stack[length - 2] = 0; // r8
        self.kernel_stack[length - 3] = 0; // r9
        self.kernel_stack[length - 4] = 0; // r10
        self.kernel_stack[length - 5] = 0; // r11
        self.kernel_stack[length - 6] = 0; // r12
        self.kernel_stack[length - 7] = 0; // r13
        self.kernel_stack[length - 8] = 0; // r14
        self.kernel_stack[length - 9] = 0; // r15
        self.kernel_stack[length - 10] = 0; // rax
        self.kernel_stack[length - 11] = 0; // rbx
        self.kernel_stack[length - 12] = 0; // rcx
        self.kernel_stack[length - 13] = 0; // rdx
        self.kernel_stack[length - 14] = 0; // rsi
        self.kernel_stack[length - 15] = thread; // rdi -> First parameter for 'kickoff'
        self.kernel_stack[length - 16] = 0; // rbp
        self.kernel_stack[length - 17] = 0x2; // rflags (IE = 0); interrupts disabled

        self.stack_ptr = self.stack_ptr - (size_of::<u64>() * 16);
    }

    /// Switch this thread from Ring 0 to Ring 3.
    /// For this, the kernel stack is prepared in a way that an 'iretq' instruction
    /// switches to user mode (Ring 3) and the user stack is used. If this function works correctly,
    /// the thread continues in user mode in the function 'kickoff_user_thread'.
    fn switch_to_usermode(&mut self) { //KI Kram
        //todo!("Thread::switch_to_usermode() is not implemented yet");
       const SAFETY_MARGIN: usize = 1024;

        // Top des aktuellen Kernel-Stacks
        let kernel_stack_top =
            Self::get_top_of_stack(&self.kernel_stack) as usize;

        // Genügend Abstand zum aktuellen Stackframe lassen
        let frame_addr = kernel_stack_top - SAFETY_MARGIN;

        // Top des User-Stacks bestimmen.
        // Beim Eintritt in eine normale x86-64-Funktion soll RSP
        // auf 8 modulo 16 ausgerichtet sein.
        let user_stack_top =
            Self::get_top_of_stack(&self.user_stack) as usize;
        //let user_rsp = (user_stack_top & !0xF) - 8;

        // Den IRETQ-Stackframe auf dem Kernel-Stack aufbauen.
        let frame = frame_addr as *mut usize;

        unsafe {
            frame.add(0).write(ptr::from_mut(self) as usize); //THread-Zeiger (self)
            frame.add(1).write(
                Thread::kickoff_user_thread as *const () as usize
            ); // RIP Startadresse im Usermode
            frame.add(2).write(0x1B);       // User CS (Code segment)
            frame.add(3).write(0x202);      // RFLAGS: Reserved bit + IF (interrupt flags)
            frame.add(4).write(user_stack_top);   // User RSP (neuer Stackpointer)
            frame.add(5).write(0x23);       // User SS (Data segment)

            // Wechselt über thread_user_start und iretq nach Ring 3.
            thread_user_start(frame_addr);
        }
    }

    /// Called indirectly by using the prepared stack in 'thread_start' and 'thread_switch'.
    fn kickoff_kernel_thread(&mut self) {
        // Set TSS rsp0 to the top of the kernel stack of this thread
        unsafe {
            let rsp0 = Self::get_top_of_stack(&self.kernel_stack);
            tss_set_rsp0(rsp0 as usize);
        }

        if self.is_kernel_thread {
            cpu::enable_int(); // Interrupts are disabled during thread start
            ((*self).entry)();
        } else {
            self.switch_to_usermode(); // make it a user thread
        }

        scheduler().exit();
    }

    /// Called indirectly by using the prepared stack in 'switch_to_usermode'.
    fn kickoff_user_thread(&self) {
        //todo!("Thread::kickoff_user_thread() is not implemented yet");
        let entry = self.entry;
        entry();

        loop {} // User threads may currently not exit
    }

    /// Get a pointer to the top of the given stack.
    fn get_top_of_stack(stack: &Vec<u64>) -> *const u64 {
        unsafe {
            ptr::from_ref(&stack[stack.len() - 1]).offset(1)
        }
    }
}

impl Display for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "T{}", self.id)
    }
}
