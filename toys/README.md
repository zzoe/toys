# 环境准备

1. Node 版本管理

   * nvm-windows:
      https://github.com/coreybutler/nvm-windows/releases

   * fnm
   ```shell
   cargo install fnm
   ```
   
2. 安装 nodejs

   ```shell
   nvm install latest
   nvm use 版本号
   ```

3. 安装 pnpm：

   * windows
   ```shell
   iwr https://get.pnpm.io/install.ps1 -useb | iex
   ```
   * mac
   ```shell
   brew install pnpm
   ```

4. 修改环境变量
   PNPM_HOME
   PATH

5. 修改镜像

   ```shell
   pnpm config set registry https://registry.npmmirror.com
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
pnpm install tailwindcss
```

vim package.json

```json
{
  "scripts": {
    "css": "tailwindcss -i ./input.css -o ./public/tailwind.css"
  }
}
```

# 编译调试

```shell
pnpm css
dx build --release --platform web
dx bundle
```
