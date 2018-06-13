# 基于serviceWorker的缓存投毒
## *Sat May  2 20:52:57 2015*

前几天才察觉到 Chromium 42 已经支持 Fetch API 了，
鉴于我早就想用 Fetch 代替 Ajax，
所以今天把 ice.js 的`$.http`部分改用 Fetch [实现](https://github.com/quininer/quininer.github.io/commit/df932e8111e716a0813f6c3fa856d196626352a6)了。

没想到的是 Firefox 目前还是 37 版本，不支持 Fetch。所以不得不引入一个 fetch.js 做兼容。

-------------------------------

以上都不是重点
--------------

和 Fetch 搭配使用的还有一个有趣的东西，`serviceWorker`。
其实早就在某牛的[博客](http://www.web-tinker.com/article/20879.html)中看到过，
当时没有太深想，也因为没有实际上支持 Fetch 的浏览器，所以并没在意。

因为上文的事件，想起了这东西。
简单来说，这个API能拦截 HTTP Request，并返回预定的 Response。

自然，Response 里也包括 HTTP Headers，
所以只要简单的在 HTTP Header 里控制一下缓存时间即可，比如

    Cache-control: max-age=99999999999999999999999999999999999999

下面是一些截图

![1](/upload/cache-tests-1.png)
![2](/upload/cache-tests-2.png)
![3](/upload/cache-tests-3.png)

POC: [cache tests](https://quininer.github.io/tests/cache-tests/serviceworker/index.html)

实际上也并非所有 HTTP Header 都能使用，这里有一个[禁表](https://fetch.spec.whatwg.org/#concept-header-list)。

-------------------------------

~~总之这套东西不管从哪方面来说都好方便。 :)~~

实际上
------

收回上一句话。
和[FD](https://twitter.com/filedescriptor)测试了一会，
上面的POC看上去很美，实际上`serviceWorker`限制多多。

比如

* 只能拦截**同源**且子路径的 Requests
* scriptURL 必须**同源**，且 [原则](https://github.com/slightlyoff/ServiceWorker/issues/262) 上必须是**实际**存在网络的资源
* `worker.js` 的处理和一般 script 处理不一样，必须**符合**规范，否则将会静默的失败
* `serviceWorker`只能工作在 <ruby>开发环境<rt>localhost</rt></ruby> 和 [HTTPS](https://github.com/slightlyoff/ServiceWorker/issues/385) 下

且不说要求 HTTPS 这样涉及协议的奇葩规定，
要求 scriptURL 必须实际存在这一点就让利用变得几无可能。

实际上使用也会变得极不方便，难以和各种完全由 JavaScript 生成的框架整合。
也没有实际上解决安全问题。

我倒是更希望 scriptURL 能允许使用 Blob，而`Cache`被加入 HTTP Header 禁表。
