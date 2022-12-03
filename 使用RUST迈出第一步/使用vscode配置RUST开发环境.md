# 安装rustup
  
  ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

# 检查RUST安装
控制台输入以下命令
```rust
    rustc --version
    cargo --version
```
  
# 创建一个新的目录来保存代码
```rust
    mkdir ~/rust-learning-path
    cd ~/rust-learning-path
    makir hello-world
    cd hello-world
```

# 编写第一个RUST程序并运行
创建一个名为 main.rs 的新文件，并使用编辑器将以下代码写入其中：
```rust
    fn main() {
        println!("Hello World!");
    }
```