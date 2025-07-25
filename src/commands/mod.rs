pub mod timestamp;
pub mod password;
pub mod completion;

/// 所有命令的通用trait
pub trait Runable {
    /// 执行命令
    fn run(&self);
}