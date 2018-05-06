# WebAssembly StackOverflow
## *Mon Apr 30 16:50:47 2018*


[WebAssembly] 是一個設計運行在瀏覽器上的類彙編語言，
藉助它可以令 C / C++ 之類的語言運行在瀏覽器上，以期得到性能提升。

新功能自然會引入新攻擊面，可以預見 C / C++ 上經常出現的溢出攻擊並不會在 wasm 上消失。

WebAssembly 同常見的彙編有很多不一樣的地方，例如

* 它是一個 [Stack Machine]
* 它使用 [Linear Memory]
* 指令與線性內存隔離
* …

因爲第三點，WebAssembly 在一定程度上是 [Harvard architecture]，這爲 wasm 帶來了很大的安全優勢。

指令與數據隔離，使得 ROP、return2libc 之類的技巧不可能，但棧溢出本身仍是可能的。

我們舉個虛構的例子來證明 wasm 棧溢出的可行性

Rust 代碼:

```rust
// ...

#[wasm_bindgen]
pub fn hash_with_key(key: &str, data: &str) -> u32 {
    let mut val = [0; 32];
    let key = key.as_bytes();
    let data = data.as_bytes();

    val[..3].copy_from_slice(&key[..3]);
    val[3] = 0x33;
    val[4..][..data.len()].copy_from_slice(&data);

    js_hash(&val)
}

#[wasm_bindgen]
pub fn table_index(i: usize) -> u8 {
    let a = [b'a', b'd', b'c', b'b'];
    unsafe { *a.get_unchecked(i) }
}
```

WebAssembly 的 wat 格式基於 sexp，指令精簡，與傳統彙編相比，相對容易閱讀。

鑑於編譯出的 wasm 較長，下面我們只看我們關心的內容，
完整的內容可以看[這裏](https://gist.github.com/quininer/53820dbf2bcadfca807aee1e9adb4865#file-wasm_stack_overflow_zero_bg-wat)。

```wasm
(global (;0;) (mut i32) (i32.const 1050544))
```

`global` 定義了一塊全局內存。其中一部分是數據段，剩下的 `1`MB 是棧空間。

```wasm
  (func $hash_with_key (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    get_global 0
    i32.const 32
    i32.sub

	;; ...

        get_local 2
        get_local 0
        i32.load16_u align=1
        i32.store16
        get_local 2
        get_local 0
        i32.const 2
        i32.add
        i32.load8_u
        i32.store8 offset=2

	;; ...
  )
```

函數 `hash_with_key` 在棧上開闢 `32`bytes，將 key 寫入棧上，退棧時未將其清零。

```wasm
  (func $table_index (type 4) (param i32) (result i32)
    (local i32)
    get_global 0
    i32.const 16
    i32.sub

	;; ...

    i32.const 12
    i32.add
    get_local 0
    i32.add
    i32.load8_u)
```

函數 `table_index` 在棧上開闢 `16`bytes，偏移 `12`bytes 讀取。

在用戶正常調用 `hash_with_key(..)` 之後攻擊者調用 `table_index(16-12-32+x)` 便可獲得 key。

![Read Stack Overflow](/upload/wasm-read-stack.png)


-----

作者不熟悉二進制安全、計算機原理。胡言亂語不知所云，有錯勿怪。

[WebAssembly]: http://webassembly.org/
[Stack Machine]: https://docs.google.com/document/d/1CieRxPy3Fp62LQdtWfhymikb_veZI7S9MnuCZw7biII/edit
[Register machine]: https://en.wikipedia.org/wiki/Register_machine
[Linear Memory]: http://webassembly.org/docs/semantics/#linear-memory
[Instructions]: https://webassembly.github.io/spec/core/valid/instructions.html
[Harvard architecture]: https://en.wikipedia.org/wiki/Harvard_architecture
