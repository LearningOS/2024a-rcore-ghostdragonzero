//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task info
    pub task_info: TaskInfo,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
/// The info of a task
#[derive(Copy, Clone, PartialEq)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// task_start time
    pub start_time: usize,
    /// Total running time of task
    pub time: usize,
}
impl TaskInfo{
    ///init taskinfo
    pub fn init_zero() -> Self{
        Self{
            status:TaskStatus::UnInit, 
            syscall_times:[0;MAX_SYSCALL_NUM], 
            start_time:0,
            time:0,
        }
    }
}
