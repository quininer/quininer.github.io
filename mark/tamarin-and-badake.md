# 如何設計一個壞協議
## *Wed Jun 13 19:44:31 2018*

近幾日看 [The 8th BIU Winter School on Cryptography](https://cyber.biu.ac.il/event/8th-biu-winter-school/) 的 slide，
這期冬令營主要內容是協議設計。

冬令營第二講 [Implicitly Authenticated KE Protocols](http://cyber.biu.ac.il/wp-content/uploads/2017/08/KE2_Hugo_BIU_Feb2018.pdf) 拋磚引玉，
提出了一個相當簡潔的兩消息<ruby>隱式帶認證密鑰交換<rt>Implicitly Authenticated Key Exchange</rt></ruby>協議，

後文將稱之爲 <ruby>BADAKE<rt>Basic A-round Diffie-hellman Authenticated Key Exchange</rt></ruby>。

![BADAKE](./upload/badake.png)

這是一個相當乾淨的協議，甚至可能大家自己就在實踐中設計過類似的協議。
但這不是一個安全的協議。

slide 中已經舉出了一些該協議的攻擊，這些攻擊並不那麼直觀。

Define
------

在描述這些攻擊之前，我們先澄清一些定義。

* 隱式帶認證密鑰交換:
	與顯式認證密鑰交換不同，隱式認證協議不會在協議執行中顯式的失敗，
	但只有雙方身份正確時會生成相同的密鑰。
* 認證協議安全要求: 一個合格的認證協議應該在協議完成之後保證
	+ <ruby>基本僞裝抗性<rt>Basic Impersonation Resistance</rt></ruby>: 若 A 正確執行協議，當敵手不知道 A 的長期密鑰時，敵手不可以僞裝成 A。
* 密鑰交換協議安全要求: 一個合格的密鑰交換協議應該保證，
	+ <ruby>完美前向保密<rt>Perfect Forward Secrecy</rt></ruby>: 長期密鑰泄漏時，不會影響之前會話密鑰的安全性。
	+ <ruby>弱前向保密<rt>weak Perfect Forward Secrecy</rt></ruby>: 由於兩消息的認證密鑰交換協議無可能滿足 PFS，所以對於這些協議，退而求其次，我們只要求 wPFS。
		若敵手在長期密鑰泄漏之前沒有參與協議，那長期密鑰泄漏時，不會影響之前會話密鑰的安全性。
	+ <ruby>已知密鑰安全<rt>Known-Key Security</rt></ruby>: 會話密鑰泄漏時，不會影響長期密鑰及其他會話密鑰的安全性。
	+ <ruby>臨時祕密揭露抗性<rt>Ephemeral Secrets Reveal Resistance</rt></ruby>: 臨時密鑰泄漏時，不會影響長期密鑰及其他會話密鑰的安全性。
	+ <ruby>未知密鑰共享<rt>Unknown Key-Share</rt></ruby>抗性: 當 A 和 C 協商出會話密鑰時，A 不會認爲該密鑰是與 B 協商得到的。

Signature + DH
--------------

在分析 BADAKE 前，slide 先給出了一個簡單的簽名與密鑰交換的組合協議。
這個協議在簽名中加入了目標身份，以避免交錯攻擊。

但這個協議仍然是不安全的。

當 A 的臨時密鑰 `x` 泄漏時，
攻擊者可以重放消息 `g^x, SIG{A}(g^x, B)`，與 B 進行握手。
從而達成僞裝 A 身份的目的。

一般情況下防止重放攻擊的最好方案是添加一個 challenge，
但這需要增加一輪通信。

這個例子說明了使用 Signature + DH 的簡單組合無法實現安全的兩消息帶認證密鑰交換。

Implicitly Authenticated DH
---------------------------

slide 給出了 BADAKE 的兩種簡單組合方式。

第一種組合方式相當簡單，
```
K = H(g^ab, g^xy)
```

slide 中描述它受到 `Open to known key and interleaving attacks` 的威脅。
一開始我不太明白這個攻擊如何實現。

[藉助 tamarin](https://gist.github.com/quininer/5e29437c3c6e1d66942c8fddb39a8536)，
我們可以輕鬆找到針對這個協議的多個攻擊方案，這裏我們舉出兩個

* 反射攻擊

![reflection attack](./upload/reflection_attack.png)

A 向 B 發起兩個握手請求，敵手將這兩個請求作爲響應發回給 A。

A 不會知道他正在和自己通信。（呆萌

* 會話密鑰揭露時的交錯攻擊

![interleaving attack](./upload/open_key_interleaving_attack.png)

這圖片中描述的攻擊與 slide 可能有所不同。

此攻擊需要 B 在握手結束之後泄漏會話密鑰。

B 向 A 發起兩個握手請求，敵手將一個請求作爲響應發回給 B。
兩個會話產生相同的密鑰，結束其中一個會話，B 泄漏會話密鑰 `K`。
敵手使用該密鑰繼續與 B 通信。

最終達成僞造 A 身份的目的。

KCI Attack
----------

第二種組合方式添加了角色因素，
可以對抗反射、並行會話等攻擊方式。
```
K = H(g^ab, g^xy, g^x, g^y)
```

這相比第一種組合方式要好一點，但仍不是一個理想的 AKE 方案。

slide 中提到了<ruby>密鑰泄漏僞裝攻擊<rt>Key-Compromise Impersonation</rt></ruby>。
簡單來講就是，在 A 的長期密鑰泄漏之後，敵手不但可以僞裝成 A 與 B 通信，也可以僞裝成 B 與 A 通信。

這個漏洞在使用密鑰交換作認證的協議裏很常見，例如 [Tox](https://github.com/TokTok/c-toxcore/issues/426) 就有這個問題。

很多人會覺得泄漏長期密鑰之後，協議可以不保證認證安全性。但一個好的認證協議應該要避免這一點。

-----

上文的內容在冬令營的演講中只佔不到半分鐘時間，而後面關於 MQV、HMQV 的各種變種佔了將近兩個小時。

設計一個好的密碼協議並不是一件容易的事情。

而想要設計一個壞協議？只需要拍腦袋式的想一個簡潔的協議就可以了。
