use crate::thread::scheduler::scheduler;

pub extern "C" fn sys_thread_exit() {
    scheduler().exit();
}