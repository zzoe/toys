# 环境准备
1. 安装 nvm-windows:
   https://github.com/coreybutler/nvm-windows/releases
2. 安装 nodejs
```shell
nvm install latest
nvm use 版本号
```
3. 安装 pnpm：
```shell
iwr https://get.pnpm.io/install.ps1 -useb | iex
```
4. 修改环境变量
   PNPM_HOME
   PATH
5. 修改镜像
```shell
pnpm config set registry https://registry.npm.taobao.org/
```
6. 安装rust：https://www.rust-lang.org/zh-CN/learn/get-started
7. 安装dioxus-cli
```shell
cargo install dioxus-cli
```
# 初始化项目
```bash
dx create --template=gh:dioxuslabs/dioxus-template
pnpm init
pnpm i -D tailwindcss
```
vim package.json
```json
{
  "scripts": {
    "watch": "tailwindcss -i ./input.css -o ./public/tailwind.css --watch"
  }
}
```
# 编译调试
```shell
pnpm watch
dx serve --release --platform desktop
```
