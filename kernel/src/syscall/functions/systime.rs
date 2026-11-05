use crate::device::pit::system_time;

pub extern "C" fn sys_get_system_time() -> usize {
    system_time()
}