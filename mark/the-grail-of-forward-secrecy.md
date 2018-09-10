# 0-RTT and Forward Secrecy
## *Tue Sep 11 00:43:19 2018*


前向保密是一個已經被人熟知的安全特性。
基於密鑰交換的前向安全廣泛存在於各種密碼協議之中，例如 TLS、Signal Protocol、WireGuard。

Session based
-------------

一般我們所說的前向保密方案是基於會話的，每次會話通過握手產生一個獨立的會話密鑰。
會話結束之後銷毀該會話密鑰便可保證前向保密。

但基於會話的前向保密並不適用於所有場景，
很多情況下我們沒有握手的條件、或者通信開銷非常之大。
例如 Email、<ruby>IoT<rt>Internet of Things</rt></ruby>。

“如何在這些情況下保證前向保密”成爲了一個聖盃級的公開問題，
數十年來衆多密碼學家爲此付出心力。

幸運的是，這並非是無可能的。


Interval based
--------------

基於時間段的前向保密方案聽起來不太優雅，但非常實用。

基本思路是將密鑰分成不同的時間段。
在某個時間段過去之後，銷毀該時間段對應的私鑰，從而達到前向保密。

Weekend scheme
==============

我相信這個方案在歷史上被設計過不止一次，最後一次見到它是在 openpgp 討論中 <sup>1</sup>。
我不清楚它的名字，本文將之稱爲週末方案。

用戶每逢週末更換一次自己的密鑰，並將新的公鑰公佈在網絡上。
其他用戶加密時必須檢索對方的最新公鑰。

這個方案很簡單，但缺點也很明顯

* 要求其他用戶加密時有檢索最新公鑰的能力
* 多設備不友好

每次都要更新公鑰實在是太麻煩了，那可不可以再給力點？

HIBE scheme
===========

有的！我們首先通俗的介紹一下 <ruby>HIBE<rt>Hierarchical Identity Based Encryption</rt></ruby><sup>2</sup>。

HIBE 有一個中心的 <ruby>PKG<rt>Private Key Generator</rt></ruby>，它可以通過 ID 產生所有用戶的私鑰。
每個用戶也能通過子用戶的 ID 產生子用戶的私鑰，自然的，子用戶不能恢復父用戶的私鑰。

其他用戶加密時只需要知道 <ruby>MPK<rt>Master Public Key</rt></ruby> 和子用戶的 ID。

我們可以基於 HIBE 構造出一個不需要更換公鑰的前向保密方案。

1. 用戶加密時根據當時的時間產生用戶 ID，並用此進行加密。例如

	```
	CT = enc(MPK, ["2018", "9", "10"], MSG)
	```

2. 用戶解密時，根據 PKG 或父用戶的私鑰產生對應 ID 的私鑰，並用其解密，例如

	```
	SUB_SK = keygen(SK, ["2018", "9", "10"])
	MSG = dec(SUB_SK, CT)
	```

3. 當一個時間段過去時，用戶先用 PKG 或父用戶的私鑰產生剩餘所有子用戶的私鑰，並銷毀 PKG 和父用戶的私鑰。例如

	```
	(_, FEB, MAR, APR, MAY, JUN, JUL, AUG, SEP, OCT, NOV, DEC) = keygen_year(YEAR)
	delete(YEAR)
	```

完美！這個方案完全解決了週末方案的兩個缺點，
其他用戶不需要時時檢索對方的最新公鑰，用戶自己也不需要在多個設備之間同步自己的新私鑰。

但是基於時間段的方案有一個共同的缺點，

試想一個情景，alice 在 2018年9月30號 執行加密，但由於各種原因到達 bob 手中時遲了一日。
可惜當時 bob 已經將 9月 所對應的私鑰銷毀了，所以他無法解密 alice 的密文。

這顯然不是我們想要的效果，那可不可以再給力點？


Puncture based
--------------

除了時間段這個思路之外，我們還有沒有其他實現前向保密的辦法呢？

可穿刺加密是另一種思路，
原始狀態下的私鑰包含所有信息，可以解密所有密文。
當用戶成功解密一個密文後，用戶可以從私鑰中消除（我們將之稱爲<ruby>穿刺<rt>Puncture</rt></ruby>）一部分信息，
使得新私鑰不能解密該密文。

Binary Tree Encryption scheme
=============================

<ruby>BTE<rt>Binary Tree Encryption</rt></ruby><sup>3,4</sup> 是一個有趣的設計，它同樣基於 HIBE。

它相當於使用 IBE 產生一個巨大的私鑰集，
然後使用和密文對應的 tag 確定其中一個 ID 或密鑰（通過一個 Hash）。

穿刺時，通過 Hash 該 tag 來確定私鑰，然後銷毀它。

但是一個夠用的私鑰集相當大，假設一個私鑰大小爲 64b，我們期望有 2^32 個私鑰可以用，
那最終儲存這個密鑰集我們需要 274877.907 MB。

這是不現實的。

但我們可以通過 HIBE 來懶惰式的生成私鑰，這可以有效的將最壞情況下的密文尺寸降低一半。

Bloom Filter Encryption scheme
==============================

<ruby>BFE<rt>Bloom Filter Encryption</rt></ruby><sup>5</sup> 是 BTE 的一個有趣變種。

它使用一個 Bloom Filter 來降低 tag 的碰撞率，代價是較快的空間使用速度。

* 加密時，使用 <ruby>IBBE<rt>Identity Based Broadcast Encryption</rt></ruby> 將消息廣播加密
	給 bloom filter 選擇出的 k 個 ID。
* 解密時，使用 k 個 ID 中其中一個可用的 ID 進行解密，若均不可用則解密失敗。
* 穿刺時，將這 k 個 ID 對應的私鑰銷毀。

無論如何，基於 IBE 的可穿刺加密仍有相當大的私鑰尺寸。那，可不可以再給力點？

Non-Monotonic ABE scheme
========================

照例我們先簡單介紹一下 <ruby>NM-ABE<rt>Non-Monotonic Attribute based encryption</rt></ruby><sup>6</sup>。

ABE 就像 IBE，但它使用<ruby>屬性<rt>Attribute</rt></ruby> 而不是 ID 來進行加密。
<ruby>KP<rt>Key Policy</rt></ruby>-ABE 是指是加密時爲密文指定屬性，只有符合某個規則的私鑰可以解密。

而一般 ABE 的規則是單調的，即是說只有 And 和 Or 兩種規則。
非單調性 ABE 的有趣之處是支持 Not 規則。

有了 NM-ABE，構建可穿刺加密變得非常直觀。加密解密時和普通 ABE 一樣。
穿刺時，只需在私鑰規則中增加一條 `Not(tag)`。<sup>7</sup>


至此，該方案的私鑰大小只會隨着穿刺次數線性增長，而不會爆炸。

Binary Tree Bloom Filter Encryption?
====================================

我們可以將 BTE 和 BFE 結合，
利用 <ruby>HIBBE<rt>Hierarchical Identity Based Broadcast Encryption</rt></ruby>
構造出 <ruby>BTBFE<rt>Binary Tree Bloom Filter Encryption</rt></ruby>。

理論上這可以將 BFE 的私鑰尺寸降得更低，但我沒有找到合適的 HIBBE 方案實現它。

Perfect?
--------

我們介紹了數個在限制情況下保證前向保密的加密方案，看起來似乎足夠讓人們戴着鐐銬跳舞。

如果這些方案足夠高效，是不是意味著我們再不需要握手，無延遲的 0-RTT 通信可以全面推廣呢？

很遺憾並不是，這些方案只能保證“前向保密”。

而基於會話的方案除了前向保密之外還保證了“弱後向保密”，
即在泄漏長期私鑰時，若敵手在之後沒有參與協議，那協議仍能保證之後密文的保密性。

當然，這一切在量子敵手面前都毫無意義。

-----

1. [Clarify status of subkeys with certification use](https://mailarchive.ietf.org/arch/msg/openpgp/mk8_FSS-n4DVGfh_VwuGtEOS2xk)
2. [Hierarchical Identity Based Encryption](http://cryptowiki.net/index.php?title=Identity-based_encryption_schemes#Hierarchical_Identity_Based_Encryption)
3. [Binary Tree Encryption: Constructions and Applications](https://www.cs.umd.edu/~jkatz/papers/BTE-survey.pdf)
4. [0-RTT Key Exchange with Full Forward Secrecy](https://eprint.iacr.org/2017/223)
5. [Bloom Filter Encryption and Applications to Efficient Forward-Secret 0-RTT Key Exchange](https://eprint.iacr.org/2018/199)
6. [Encryption with non-monotone access structure](http://cryptowiki.net/index.php?title=Attribute-based_encryption#Encryption_with_non-monotone_access_structure)
7. [Forward Secure Asynchronous Messaging from Puncturable Encryption](http://cs.jhu.edu/~imiers/pdfs/forwardsec.pdf)
