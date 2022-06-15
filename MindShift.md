# MindShift 心智轉換

> [從 C++ 轉向 Rust ， 兩大主題值得關注](https://mp.weixin.qq.com/s/UTncMLvU4dcjZumK2dQrqQ)

---

## 錯誤處理 (Error Handle)

1. 從 C 繼承下來的返回值風格。 所有函數必須返回整型，用錯誤碼來表示各種錯誤情況
```c
#include <stdio.h>
#include <stdlib.h>
int main() {
    printf("Hello world !");

    /**
    if (無法開啟檔案去做存取寫入的話)
    return 1;
     */

    return 0;
}
```
2. C++ 的異常則是在錯誤的位置拋出異常；然後在錯誤處理的位置捕捉異常。 Like PHP
```php
try {
    // do something
    if (! $this->apiService->checkServiceSurvival()) {
        throw new ApiServiceIsUnArrivalException();
    }
    // do something
} catch (\Exception $e) {
    // ... error handle ...
}
```

兩種方式都有人使用，各有優劣勢的地方…
- 返回值風格
    - 優點：
        - 清晰，錯誤發生的位置和處理方式都比較直白
    - 缺點：
        - 程式碼較為拖沓，錯誤處理與業務程式碼交錯在一起，`使主要邏輯不突出`。同時佔用了返回值位置，影響邏輯表達
        - 若`無強制錯誤強制錯誤檢查`，可能會引發一些代碼缺陷等問題

- 異常處理的風格：
    - 優點：
        - 業務程式碼相較起來比較容易看懂
    - 缺點：
        - 由於`異常隱晦`，使得任何地方都可能拋出異常。
        - 函數的退出點也變得隱晦，帶來`異常安全`的問題，增加程式碼的心智負擔


## Rust 
- `Result<T, E>`
Rust 沒有提供異常機制，與使用 Option 來解決可選的情況類似

```rust
pub enum Result<T, E> {
    // Contains the success value
    // 包含成功的值
    Ok(T),

    // Contains the error value
    // 包含失敗的值
    Err(E);
}
```

### 這邊使用 Result + ? 操作符
```rust 
#[derive(Debug)]
pub enum MyError {
    IoError(String),
    Inexist(String),
}

pub type Result<T> = std::result::Result<T, MyError>;

pub fn fetch_id() -> Result<u32> {
    Ok(0)
}

fn main() -> Result<()> {
    let id = fetch_id()?;
    println!("{:?}", id);
    Ok(())
}
```

### 當不使用 ? 操作符的話就是 ... ； 相當於被調函數 (fetch_id) 正常返回則 unwrap 其值；反之，則將被調函數的錯誤向上返回
```rust
let id = match fetch_id() {
    Ok(id) => id,
    Err(err) => {
        return Err(err);
    }
}
```


