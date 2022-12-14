# 简介  

*** 

Rust 是一门内存安全的语言，在嵌入式，加密货币交易，操作系统等领域有广阔的发展前景。
  
## 使用RUST模块系统管理代码
  
Rust 提供了一系列功能，可帮助你管理和组织代码。 这些功能称为“Rust 模块系统”。系统由箱、模块、路径和工具组成，以与那些项结合使用。

箱：Rust 箱是一个编译单元。 它是 Rust 编译器可以运行的最小代码段。 箱中的代码一起编译以创建二进制可执行文件或库。 在 Rust 中，仅将箱编译为可重复使用的单元。 箱包含具有隐式未命名顶级模块的 Rust 模块的层次结构。

模块：Rust 模块通过让你管理箱内单个代码项的范围来帮助你组织程序。 结合使用的相关代码项或项可以分组到相同模块中。 递归代码定义可以跨越其他模块。

路径：在 Rust 中，可以使用路径来命名代码中的项。 例如，路径可以是一个数据定义（例如，矢量、代码函数，甚至是模块）。 模块功能还可帮助你控制路径的隐私。 可以指定可公开访问的代码部分和私有部分。 通过该功能可以隐藏实现详细信息。
  
## 使用 Rust 箱和库
  
Rust 标准库 std 包含 Rust 程序中的基本定义和操作的可重复使用代码。 该库拥有核心数据类型（例如 String 和 Vec<T>）的定义、Rust 基元的操作、常用宏函数的代码、对输入和输出操作的支持以及许多其他功能区域。

有数以万计的库和第三方箱可用于 Rust 程序，其中大部分可以通过 Rust 的第三方箱存储库 crates.io 进行访问。 稍后你将了解如何从项目访问这些箱，但以下是编程练习中使用的一些箱：

- Rust 标准库。 在 Rust 练习中，你将会注意到以下模块：
```rust
    std::collections - 集合类型的定义，如 HashMap。
    std::env - 用于处理环境的函数。
    std::fmt - 控制输出格式的功能。
    std::fs - 用于处理文件系统的功能。
    std::io - 用于处理输入/输出的定义和功能。
    std::path - 支持处理文件系统路径数据的定义和功能。
    structopt - 用于轻松分析命令行参数的第三方箱。
    chrono - 用于处理日期和时间数据的第三方箱。
    regex - 用于处理正则表达式的第三方箱。
    serde - 适用于 Rust 数据结构的序列化和反序列化操作的第三方箱。
```

默认情况下，std 库适用于所有 Rust 箱。 若要访问箱或库中的可重复使用代码，请使用 use 关键字。 通过 use 关键字，箱或库中的代码就会“进入范围”，这样你就可以访问程序中的定义和功能。 标准库是在路径 std 的 use 语句中访问的，如 use std::fmt 所示。 其他箱或库是通过其名称访问的，例如 use regex::Regex。
  
## 使用 Cargo 创建和管理项目
虽然可以直接使用 Rust 编译器 (rustc) 来生成箱，但大多数项目都使用 Rust 生成工具和名为 Cargo 的依赖项管理器。
```rust
    Cargo 可以为你做许多事情，包括：

    使用 cargo new 命令创建新的项目模板。
    使用 cargo build 编译项目。
    使用 cargo run 命令编译并运行项目。
    使用 cargo test 命令测试项目。
    使用 cargo check 命令检查项目类型。
    使用 cargo doc 命令编译项目的文档。
    使用 cargo publish 命令将库发布到 crates.io。
    通过将箱的名称添加到 Cargo.toml 文件来将依赖箱添加到项目。
```