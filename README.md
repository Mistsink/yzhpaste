# yzhpaste - `云蒸 Paste`

旨在实现全平台剪切板增强工具

> 仿 MacOS 的 `Paste`

## 命令

```sh
# 建议使用 pnpm 
# npm install --global pnpm

pnpm i

pnpm tauri dev

# 如 tauri 运行出现问题请参照 tauri 官网说明
# tauri 前置环境: https://tauri.app/v1/guides/getting-started/prerequisites
```



## 贡献代码

请遵循常规多人协作规范，创建新的分支，保证测试无误后合入`main`支，或`fork`下来提交`pr`。

同时使用`git`提交代码时请使用`pnpm cz`命令规范化提交信息。需要您熟悉安装`git-cz`工具，请自行搜索下载。

```bash
# 如完成代码编写后，需要 git add . && git commit -m'msg' 时
# 直接使用 pnpm cz
pnpm cz
```



