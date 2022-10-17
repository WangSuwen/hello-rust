RUST 学习

## 一、Cargo
* `cargo fmt` 格式化代码
* `cargo build` 编译项目并生成可执行文件到 /target/debug/xxx 中.、安装依赖包
* `cargo run` 编译项目并直接运行
* `cargo check` 此命令快速检查您的代码以确保它可以编译但不会生成可执行文件
* `cargo build --release` 此命令构建生产环境
* `cargo doc --open` 构建依赖包的本地说明文档  // TODO: 这个很重要

## 二、本地断点调试

#### 1、Mac 上安装： `CodeLLDB`插件； Windows上安装： `C/C++`插件
#### 2、添加`.vscode/launch.json`文件
```
{
    // 使用 IntelliSense 了解相关属性。 
    // 悬停以查看现有属性的描述。
    // 欲了解更多信息，请访问: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "name": "本地调试",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceRoot}/target/debug/hello-rust", // 这里的 hello-rust 需要根据自己的项目详情来替换
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```