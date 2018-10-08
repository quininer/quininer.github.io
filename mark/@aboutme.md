About Me
========

* quininer
	<div style='float: right;'>![](https://avatars2.githubusercontent.com/u/6286761?s=200)</div>
* Blog: [https://quininer.github.io/](https://quininer.github.io/)
* Github: [https://github.com/quininer](https://github.com/quininer)
* Email: [quininer@live.com](mailto:quininer@live.com)


Skills
------

* 编程语言
	+ 自 2016 年以来，长期使用 [Rust]
	+ 曾使用 Python、JavaScript
	+ 曾接触 D、OCaml
* 密码学
	+ 一定的密码学基础
	+ 一定的密码协议分析能力
	+ 实现过一些较新穎的密码算法

| Name | Description
|------|-------------
| [NewHope](https://github.com/quininer/newhope) | 基于 RLWE 的密钥交换算法
| [Dilithium](https://github.com/quininer/dilithium) | 基于 MLWE 的签名算法
| [Kyber](https://github.com/quininer/kyber) | 基于 MLWE 的、CCA 安全的密钥封裝算法
| [Gimli](https://github.com/quininer/gimli) | 轻量级、高安全性的置换算法
| [COLM](https://github.com/quininer/colm) | 在线、防滥用的 AEAD 算法
| [NORX](https://github.com/quininer/norx) | 基于海绵结构、灵活的 AEAD 算法
| [MEM-AEAD MRS](https://github.com/quininer/mem-aead-mrs) | 基于海绵结构、防滥用的 AEAD 算法
| [BFenc](https://github.com/noisat-labs/bfenc) | 布隆过滤器加密，允许实现前向保密的 0-RTT 通信

* 基础 Web 编程
	+ 有 Web 安全相关经验
	+ 包括一些前端知识，例如 WebAssembly
* 日常使用 Arch Linux、Neovim，较熟悉 Linux 环境


Projects
--------

### sarkara - 2016 ~ 2018 - <div style='float: right;'>https://github.com/quininer/sarkara</div>

+ 实验性 [后量子](https://en.wikipedia.org/wiki/Post-quantum_cryptography) 密码库
+ <ruby>密钥交换<rt>Key Exchange</rt></ruby>
	使用 [kyber](https://eprint.iacr.org/2017/634.pdf)
+ <ruby>签名方案<rt>Signature Schemes</rt></ruby>
	使用 [dilithium](https://eprint.iacr.org/2017/633.pdf)
+ <ruby>认证加密<rt>Authenticated Encryption</rt></ruby>
	使用 [norx](https://norx.io/)

### ENE - 2018 - <div style='float: right;'>https://github.com/quininer/ene</div>

+ 为邮件设计的端到端加密工具
+ <ruby>认证密钥交换<rt>Authenticated Key Exchange</rt></ruby>
+ <ruby>可否认的身份认证<rt>Deniable authentication</rt></ruby>
+ 具有 Nonce 滥用抗性的 AEAD
+ 实验性的 Post-quantum 算法支持


More Projects
-------------

| Project	| Description
|-----------|--------------
| [memsec](https://github.com/quininer/memsec) / [seckey](https://github.com/quininer/seckey) | 《[Securing memory allocations]》的实现以及安全封装
| [tokio-rustls](https://github.com/quininer/tokio-rustls) | <ruby>纯 Rust TLS 库<rt>[rustls]</rt></ruby>的<ruby>异步<rt>[tokio]</rt></ruby>支持
| [x11-clipboard](https://github.com/quininer/x11-clipboard) | x11 剪切板库
| [webdir](https://github.com/Tyzzer/webdir) | 一个简单的异步的静态文件 HTTP 服务器
| [rust-hacl-star](https://github.com/quininer/rust-hacl-star) | [HACL\*] 的安全薄包装
| [ktls](https://github.com/quininer/ktls) / [tokio-linux-zio](https://github.com/quininer/tokio-linux-zio) | Linux [Kernel TLS] 和零复制技术的 Rust 支持


[Rust]: https://www.rust-lang.org/
[Securing memory allocations]: https://download.libsodium.org/doc/helpers/memory_management.html
[tokio]: https://tokio.rs/
[rustls]: https://github.com/ctz/rustls
[HACL\*]: https://github.com/mitls/hacl-star
[Kernel TLS]: https://github.com/torvalds/linux/blob/master/Documentation/networking/tls.txt
