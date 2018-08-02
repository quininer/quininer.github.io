# **E**ncrypt o**n** **E**mail

什麼是 ENE？
------------

ENE 是一個實驗性質的、設計用於郵件的、<ruby>端到端<rt>End to End</rt></ruby>加密工具。
它基本上是我設想中的 OpenPGP 2.0，針對 PGP 的弱點作出了一些改進。

郵件是一個單次、無狀態的通信方案，這意味著爲它設計加密方案無法達到很多常見的安全要求。
例如經常被提到的 <ruby>完美前向保密<rt>Perfect Forward Secrecy</rt></ruby>。
事實上放寬其中一個限制，我們就可以做到很多，
例如 OpenPGP 實際上有一個有狀態的前向保密方案<sup>1</sup>。

但在這些條件下，我們可以做的事情並不多。

在早期，我們滿足於簡單的 <ruby>KEM<rt>Key encapsulation mechanisms</rt></ruby> + Encrypt 加密方案，
這些方案缺失了身份認證、完整性驗證。

而在 2018 年，我們可以做得更好。

ENE 將會提供：

* <ruby>認證密鑰交換<rt>Authenticated Key Exchange</rt></ruby>
* <ruby>可否認的郵件認證<rt>Deniable authentication</rt></ruby>
* 郵件<ruby>完整性<rt>Integrity</rt></ruby>
* 具有 Nonce-misuse Resistant 的加密方案
* 實驗性的 Post-quantum 密碼套件


我們需要認證？
=============

試想一個場景，Alice 向 Bob 發送一個沒有認證的加密消息，Mallory 可以輕鬆的攔截並轉發這個消息給 Bob。
Bob 會認爲這個消息是由 Mallory 發送的。

這是由於缺乏身份認證導致的。

可能很多人會覺得加簡單的增加一個簽名會解決所有問題，但並不是。簡單的簽名方案有兩個缺點

1. 可否認性<sup>2</sup>的丟失

	* 當你使用私鑰簽名一段消息時，實際上你背負了一定的法律責任。你將無法否認那段消息出自於你的手筆。

2. Surreptitious Forwarding Attack<sup>3,4</sup>

	* Mallory 可以將 Alice 的簽名剝離，使用自己的簽名代替。
	* 可能有人認爲 Sign-then-Encrypt 將會解決這個問題，但若加密方案達不到 CCA<sup>5</sup>，那它仍無法防止此攻擊。

ENE 有四種認證方案，用戶可以靈活選擇自己想要的安全性。

1. One-Pass Authenticated Key Exchange<sup>6,7</sup>

	* 一次通信的認證密鑰交換有良好的性能，因爲它不涉及複雜的簽名算法。
	* 由於只能通信一次，我們仍無法做到完美前向保密。但我們可以做到發送者前向保密，這至少和公鑰加密一樣好。
	* one-pass ake 只提供隱式認證，
		這意味著它無法抵抗 <ruby>KCI<rt>Key Compromise Impersonation</rt></ruby>攻擊<sup>8</sup>。
	* one-pass ake 是 ENE 中最弱的認證方案，但它仍能防禦大多數攻擊。
		你應該用它代替早期郵件加密中無認證的加密方案。

2. Signature-based Authenticated Key Exchange<sup>9,10</sup>

	* 在密鑰交換中加入簽名可以做到顯式認證。這個增強將可以抵抗 KCI 攻擊。
	* 簽名內容不包括消息，這意味著該方案仍能保持消息的可否認性。

3. Signature Key Exchange and Message

	* 在 方案2 的基礎上，簽名內容包括明文消息。這將保證消息的<ruby>不可否認性<rt>Non-Repudiation</rt></ruby>。

4. Signature Only

	* 只簽名明文消息，不作任何加密。

我們需要 AEAD？
===============

我相信我不需要在 8102 年贅述完整性驗證的重要性。

[EFAIL](https://efail.de/) 就是一個未能很好處理完整性驗證導致的漏洞。

實際上 OpenPGP 有自己的完整性驗證方案 MDC<sup>11</sup>，
但在大多數應用中只當作警告處理，並且存在降級攻擊<sup>12</sup>。
而在 ENE 中，我們可以使用更現代的 AEAD 方案，更好的保證完整性。

關於 AEAD<sup>13</sup> 的研究已經有很多年，更多的安全性質被提出，例如 Nonce-misuse Resistant<sup>14</sup>。

由於郵件無狀態的特點，我們只能隨機的產生 nonce。
對於某些加密算法而言，nonce reuse 的後果是毀滅性的，例如 AES-GCM<sup>15</sup>。
雖然我們可能沒有如此大量的郵件以導致生日攻擊，
但使用一個 Nonce-misuse Resistant AEAD 將可以免疫這個問題。

我們需要 Post-quantum？
=======================

衆所周知，量子機可以打破所有當前流行的公鑰算法。
屆時，所有使用 RSA/ECC 加密的郵件都將無法保證保密性。
因此，有必要在實用的量子機出現之前進行防禦。

一般有兩種方法來對抗量子機

1. 使用預共享密鑰。
2. 使用 Post-quantum 密鑰交換<sup>16</sup>。

考慮到郵件的使用場景，預共享密鑰在很多情況下不太現實。
因而使用 Post-quantum 算法是一个有意义的选择。


ENE 是完美的嗎？
----------------

不是！

ENE 未來仍有很多工作要做。例如

* 子密鑰支持
* 完全的前向保密<sup>17,18</sup>
* <ruby>郵件列表<rt>Mailing list</rt></ruby>加密支持
* <ruby>側信道攻擊<rt>Side-Channel attacks</rt></ruby>抗性
* 協議<ruby>形式化驗證<rt>Formal verification</rt></ruby>

ENE 是一個非常年輕的工具，一切在未來必將發生變化。


爲什麼不用 ENE？
----------------

ENE 不是萬能的。它被設計用於加密郵件，不是 PGP 的完全替代品。

1. Web of Trust<sup>19</sup>

	* 我不打算討論 Web of Trust 的好壞。
	* ENE 不是信任系統，不考慮如何建立信任關係。
	* 若需要，你可以使用 PGP 簽署 ENE 公鑰。

2. 爲 git commit、壓縮包以及其他公鑰簽名

	* ENE 可以做，但在這些方面不會比 PGP 有什麼優勢。
	* 當然，你也可以使用 [pbp](https://boats.gitlab.io/blog/post/signing-commits-without-gpg/)。

3. 加密巨大的文件

	* 通常郵件文本不會太大，所以 ENE 不考慮這一點。
	* 當然，這不意味著 GPG 適合做文件加密。

4. 需要可靠的、穩定的加密方案

	* ENE 非常不穩定！隨時可能發生變化。
	* “現代 PGP” 依然是最好的選擇之一，例如 [Sequoia](https://sequoia-pgp.org/)。

-----

ENE 是開源的，代碼託管在 [Github](https://github.com/quininer/ene)。

-----

1. [Forward Secrecy Extensions for OpenPGP](https://tools.ietf.org/html/draft-brown-pgp-pfs-03)
2. [Deniable authentication](https://en.wikipedia.org/wiki/Deniable_authentication)
3. [Should we sign-then-encrypt, or encrypt-then-sign?](https://crypto.stackexchange.com/questions/5458/should-we-sign-then-encrypt-or-encrypt-then-sign)
4. [Defective Sign & Encrypt in S/MIME, PKCS#7, MOSS, PEM, PGP, and XML](http://world.std.com/~dtd/sign_encrypt/sign_encrypt7.html)
5. [Chosen-ciphertext attack](https://en.wikipedia.org/wiki/Chosen-ciphertext_attack)
6. [One-Pass HMQV and Asymmetric Key-Wrapping](https://eprint.iacr.org/2010/638.pdf)
7. [OAKE: a new family of implicitly authenticated diffie-hellman protocols](https://dl.acm.org/citation.cfm?id=2508859.2516695)
8. [Key Compromise Impersonation attacks (KCI)](https://www.cryptologie.net/article/372/key-compromise-impersonation-attacks-kci/)
9. [The SIGMA Family of Key-Exchange Protocols](http://webee.technion.ac.il/~hugo/sigma.html)
10. [AEAD variant of the SIGMA-I protocol](https://tools.ietf.org/id/draft-selander-ace-cose-ecdhe-08.html#protocol)
11. [Modification Detection Code Packet](https://tools.ietf.org/html/rfc4880#page-52)
12. [OpenPGP SEIP downgrade attack](https://www.ietf.org/mail-archive/web/openpgp/current/msg08285.html<Paste>)
13. [Authenticated encryption](https://en.wikipedia.org/wiki/Authenticated_encryption)
14. [Nonce misuse resistance 101](https://www.lvh.io/posts/nonce-misuse-resistance-101.html)
15. [Nonce-Disrespecting Adversaries: Practical Forgery Attacks on GCM in TLS](https://eprint.iacr.org/2016/475.pdf)
16. [Post-quantum cryptography](https://en.wikipedia.org/wiki/Post-quantum_cryptography)
17. [Forward Secure Asynchronous Messaging from Puncturable Encryption](http://cs.jhu.edu/~imiers/pdfs/forwardsec.pdf)
18. [0-RTT Key Exchange with Full Forward Secrecy](https://eprint.iacr.org/2017/223.pdf)
19. [Web of trust](https://en.wikipedia.org/wiki/Web_of_trust)
