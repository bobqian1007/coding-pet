# Coding Pet

面向 macOS 本地开发者的 AI Coding Desktop Pet。当前 MVP 只接收 Codex CLI 的单轮完成通知，并用桌宠动画和系统通知反馈。

## 开发环境

- Node.js 20.19 或更新版本
- Rust 1.90（由 `rust-toolchain.toml` 固定）
- macOS 上的 Tauri 2 系统依赖

## 本地启动

```bash
npm install
npm run tauri dev
```

当前已完成 Tauri 2 + React 应用壳、透明置顶窗口、Codex 完成事件 Hook、回环 HTTP Listener、内存状态、完成动画和系统通知。

## Codex notify 配置

先启动桌宠：

```bash
npm run tauri dev
```

确认应用正常运行后，在 `~/.codex/config.toml` 中配置：

```toml
notify = ["/Users/weigeqian/code/coding-pet/repositories/coding-pet/scripts/ai-pet-hook"]
```

Codex 会把 `agent-turn-complete` JSON 作为 Hook 的第一个参数传入。Hook 最多等待 1 秒；桌宠未运行或转发失败时仍返回成功，不阻塞 Codex。

## 当前验证状态

- 合成 payload：HTTP 返回 `202`，完成状态事件与通知调用成功。
- 真实 Codex CLI `0.167.0`：单轮完成后 Hook 已被调用，桌宠确认接收完成事件。
- Hook 容错：桌宠关闭时约 `0.015s` 内退出，退出码仍为 `0`。
- macOS 通知和动画的最终视觉效果仍建议在目标发布包上人工确认。
