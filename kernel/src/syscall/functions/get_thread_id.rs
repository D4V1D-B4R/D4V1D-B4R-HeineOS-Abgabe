use crate::thread::scheduler::scheduler;

pub extern "C" fn sys_thread_get_id() -> usize {
    scheduler().get_active_tid()
}