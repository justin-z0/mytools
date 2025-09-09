```text
  __  ____   __  _____ ___   ___  _     ____
 |  \/  \ \ / / |_   _/ _ \ / _ \| |   / ___|
 | |\/| |\ V /    | || | | | | | | |   \___ \
 | |  | | | |     | || |_| | |_| | |___ ___) |
 |_|  |_| |_|     |_| \___/ \___/|_____|____/

```

常用工具集合，通过 `mt` 入口掌控一切

## 总体规划

- 通过 `clap` 实现命令行交互方式
- 通过 `tauri` 实现 UI 交互
- 提供 Web 在线服务

## 使用方式

所有的子命令调用都由 `mt` 发起，例如：`mt timestamp 1752915718`

## 工具列表

- timestamp 时间戳格式化工具 _( ✅ 2025/07/19)_
- password 密码管理工具 _(✅ 2025/07/24)_
- completion 生成命令补全脚本 _(✅ 2025/07/25)_
- lottery 模拟机选彩票 _(✅ 2025/07/26)_

### todo
- 完善 password 工具，能够模糊匹配，提示，过滤

## 工具详细说明

### timestamp

用于获取时间戳，或者将指定时间戳格式化为中国时间，支持毫秒操作
![timestamp](./help/timestamp.svg)

### lottery

模拟机选彩票，支持同时生成多注，默认为1注。输入：
- 回车键：确认
- 空格键：重选

![lottery](./help/lottery.svg)

### completion

生成命令补全脚本，支持 bash、zsh、fish 等多种 shell。

使用示例：

```bash
# 生成 zsh 补全脚本并添加到 ~/.zshrc
mt completion zsh > ~/.zfunc/_mt
echo 'fpath=(~/.zfunc $fpath)' >> ~/.zshrc
echo 'autoload -Uz compinit && compinit' >> ~/.zshrc

# 生成 bash 补全脚本并添加到 ~/.bashrc
mt completion bash > ~/.bash_completion.d/mt
 echo 'source ~/.bash_completion.d/mt' >> ~/.bashrc

# 生成 fish 补全脚本并添加到 ~/.config/fish/completions/
mt completion fish > ~/.config/fish/completions/mt.fish
```

## 致谢
- termtosvg 终端命令录制工具