# WebAssembly StackOverflow
## *Mon Apr 30 16:50:47 2018*


[WebAssembly] 是一個設計運行在瀏覽器上的類彙編語言，
藉助它可以令 C / C++ 之類的語言運行在瀏覽器上，以期得到性能提升。

新功能自然會引入新攻擊面，可以預見 C / C++ 上經常出現的溢出攻擊並不會在 WebAssembly 上消失。


有什麼不同
----------

WebAssembly 同常見的彙編有很多不同的地方，例如

* 它是一個 [Stack Machine]
* 它使用 [Linear Memory]
* 指令與線性內存隔離
* …

因爲第三點，WebAssembly 在一定程度上是 [Harvard architecture]，這爲 WebAssembly 帶來了很大的安全優勢，但棧溢出仍是可能的。


一個例子
--------

我們舉個虛構的例子來證明 WebAssembly 棧溢出的可行性。

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

WebAssembly 的 wat 格式基於 [S-expression]，指令精簡，與傳統彙編相比，相對容易閱讀。

鑑於編譯出的 wasm 較長，下面我們只看我們關心的內容，
完整的內容可以看[這裏](https://gist.github.com/quininer/53820dbf2bcadfca807aee1e9adb4865#file-wasm_stack_overflow_zero_bg-wat)。

```wasm
(global (;0;) (mut i32) (i32.const 1050544))
```

`global` 定義了一塊可變的全局內存。
其中 `1`MB 是棧空間，剩下的是數據段。

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

函數 `hash_with_key` 在棧上開闢 `32`bytes，將 key 寫入棧上，而退棧時未將其清零。

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

函數 `table_index` 在棧上開闢 `16`bytes，偏移 `12`bytes 讀取一個 byte。

簡單分析可以知道，在用戶正常調用 `hash_with_key(..)` 之後，
攻擊者只需要調用 `table_index(16-12-32+x)` 便可獲得 key。

![Read Stack Overflow](/upload/wasm-read-stack.png)


柳暗花明
--------

指令與數據的分離，使得我們只能讀寫棧內容而不能改變控制流。
這意味著 ROP、return2libc 之類的技巧不再有效，
棧溢出將只能在邏輯方面造成一些影響，危險度下降不止一個等級。

但是 WebAssembly 想要支持 函數指針、虛函數、trait 對象，必須引入一些動態方案。

WebAssembly 的方案是將所有需要動態使用的函數放入 [Table]，
在運行時通過引索指定，並且在調用時會檢查函數簽名。

這個設計試圖在最大程度上降低風險，但無可避免的爲攻擊者開了一扇窗。

// TODO


-----

作者不熟悉二進制安全、計算機原理。胡言亂語不知所云，有錯勿怪。

[WebAssembly]: http://webassembly.org/
[Stack Machine]: https://docs.google.com/document/d/1CieRxPy3Fp62LQdtWfhymikb_veZI7S9MnuCZw7biII/edit
[Register machine]: https://en.wikipedia.org/wiki/Register_machine
[Linear Memory]: http://webassembly.org/docs/semantics/#linear-memory
[Instructions]: https://webassembly.github.io/spec/core/valid/instructions.html
[Harvard architecture]: https://en.wikipedia.org/wiki/Harvard_architecture
[S-expression]: https://en.wikipedia.org/wiki/S-expression
[Table]: https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format#WebAssembly_tables
