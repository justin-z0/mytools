```text
                          _              _
 _   _  ___  _   _ _ __  | |_ ___   ___ | |___
| | | |/ _ \| | | | '__| | __/ _ \ / _ \| / __|
| |_| | (_) | |_| | |    | || (_) | (_) | \__ \
 \__, |\___/ \__,_|_|     \__\___/ \___/|_|___/
 |___/
```

常用工具集合，通过 `yt` 入口掌控一切

## 总体规划

- 通过 `clap` 实现命令行交互方式
- 通过 `tauri` 实现 UI 交互
- 提供 Web 在线服务

## 使用方式

所有的子命令调用都由 `yt` 发起，例如：`yt timestamp 1752915718`

## 工具列表

- timestamp 时间戳格式化工具 _( ✅ 2025/07/19)_
- password 密码管理工具 _(✅ 2025/07/24)_
- completion 生成命令补全脚本 _(✅ 2025/07/25)_

## 工具详细说明

### completion

生成命令补全脚本，支持 bash、zsh、fish 等多种 shell。

使用示例：

```bash
# 生成 zsh 补全脚本并添加到 ~/.zshrc
yt completion zsh > ~/.zfunc/_yt
echo 'fpath=(~/.zfunc $fpath)' >> ~/.zshrc
echo 'autoload -Uz compinit && compinit' >> ~/.zshrc

# 生成 bash 补全脚本并添加到 ~/.bashrc
yt completion bash > ~/.bash_completion.d/yt
 echo 'source ~/.bash_completion.d/yt' >> ~/.bashrc

# 生成 fish 补全脚本并添加到 ~/.config/fish/completions/
yt completion fish > ~/.config/fish/completions/yt.fish
```
