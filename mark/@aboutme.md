About Me
========

* quininer
	<div style='float: right;'>![](https://avatars2.githubusercontent.com/u/6286761?s=200)</div>
* Blog: [quininer.github.io](https://quininer.github.io/)
* Github: [quininer](https://github.com/quininer)
* Email: [quininer@live.com](mailto:quininer@live.com)


Skills
------

* 编程语言
	+ 自 2016 年以来，长期使用 [Rust]
	+ 曾使用 Python、JavaScript
	+ 曾接触 D、OCaml
* 密码学
	+ 一定的密码学基础
	+ 一定的协议分析能力
	+ 实现过一些较新穎的密码算法
		- [newhope](https://github.com/quininer/newhope)
		- [dilithium](https://github.com/quininer/dilithium)
		- [kyber](https://github.com/quininer/kyber)
		- [gimli](https://github.com/quininer/gimli)
		- [colm](https://github.com/quininer/colm)
		- [norx](https://github.com/quininer/norx)
		- [mrs](https://github.com/quininer/mem-aead-mrs)
* 基础 Web 编程
	+ 有一些 Web 安全相关经验
	+ 包括一些前端知识，例如 WebAssembly
* 日常使用 Arch Linux，较熟悉 Linux 环境


Projects
--------

### isperdal - 2015 - [source](https://github.com/quininer/isperdal)

+ 简单的 Python web 框架
+ 基于 asyncio / aiohttp
+ 树状 Router

### sarkara - 2016 ~ 2018 - [source](https://github.com/quininer/sarkara)

+ 实验性 [后量子](https://en.wikipedia.org/wiki/Post-quantum_cryptography) 密码库
+ <ruby>密钥交换<rt>Key Exchange</rt></ruby>
	使用 [kyber](https://eprint.iacr.org/2017/634.pdf)
+ <ruby>签名方案<rt>Signature Schemes</rt></ruby>
	使用 [dilithium](https://eprint.iacr.org/2017/633.pdf)
+ <ruby>认证加密<rt>Authenticated Encryption</rt></ruby>
	使用 [norx](https://norx.io/)

### ENE - 2018 - [source](https://github.com/quininer/ene)

+ 設計用於郵件的端到端加密工具
+ <ruby>認證密鑰交換<rt>Authenticated Key Exchange</rt></ruby>
+ <ruby>可否認的郵件認證<rt>Deniable authentication</rt></ruby>
+ 具有 Nonce-misuse Resistant 的 AEAD
+ 實驗性的 Post-quantum 密碼套件


More Projects
-------------

| Project	| Description
|-----------|--------------
| [memsec](https://github.com/quininer/memsec) / [seckey](https://github.com/quininer/seckey) | 《[Securing memory allocations]》的实现以及安全封装
| [tokio-rustls](https://github.com/quininer/tokio-rustls) | 纯 Rust TLS 库 [rustls] 的 [tokio] 异步支持
| [x11-clipboard](https://github.com/quininer/x11-clipboard) | x11 剪切板库
| [webdir](https://github.com/Tyzzer/webdir) | 一个简单的异步的静态文件服务器
| [rust-hacl-star](https://github.com/quininer/rust-hacl-star) | [HACL\*] 的 Rust 安全的薄包装


[Rust]: https://www.rust-lang.org/
[Securing memory allocations]: https://download.libsodium.org/doc/helpers/memory_management.html
[tokio]: https://tokio.rs/
[rustls]: https://github.com/ctz/rustls
[HACL\*]: https://github.com/mitls/hacl-star
