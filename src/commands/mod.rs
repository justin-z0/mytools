pub mod completion;
pub mod lottery;
pub mod password;
pub mod timestamp;

/// 所有命令的通用trait
pub trait Runable {
    /// 执行命令
    fn run(&self);
}
