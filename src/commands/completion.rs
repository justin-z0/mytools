use crate::Cli;
use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::io;

/// 命令补全工具
#[derive(clap::Parser)]
pub struct CompletionCommand {
    /// 指定shell类型
    shell: Shell,
}

impl super::Runable for CompletionCommand {
    fn run(&self) {
        let mut app = Cli::command();
        generate(self.shell, &mut app, "mt", &mut io::stdout());
    }
}
