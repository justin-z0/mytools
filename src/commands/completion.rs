use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;
use crate::Cli;

/// 命令补全工具
#[derive(clap::Parser)]
pub struct CompletionCommand {
    /// 指定shell类型
    shell: Shell,
}

impl super::Runable for CompletionCommand {
    fn run(&self) {
        let mut app = Cli::command();
        generate(self.shell, &mut app, "yt", &mut io::stdout());
    }
}