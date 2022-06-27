# RS-DEMO-1

- toml : `better toml` -> vscode's plugin 
- rust-analyzer : rust 即時編譯和解析 rust 源碼，提示程式碼中的錯誤，並且對類型進行標注。
- Tabnine : 基於 AI 的自動補全，可以幫助你更快撰寫程式碼


```bash
# 建立一個項目(lib or bin ...)
cargo new rs-demo-1 --bin
cargo new rs-demo-1 --lib
```


```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
html2md = "0.2"
```

- Rust 使用 cargo 管理項目
    - like:
	- nodejs's npm
	- go's go module
	- php's composer
	- ... etc ...

- Rust 整體語法偏向 C/C++ 風格。
- Rust 運算符 `::` 為訪問命名空間 (namespace) 或是對象的靜態函數。
如果要簡化命名空間對內部函數或是數據類型引用，可以使用 `use` 關鍵字
like

```rust
use std::fs;
```



