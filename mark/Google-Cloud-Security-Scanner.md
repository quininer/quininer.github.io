# Google-Cloud-Security-Scanner
## *Thu Mar  5 14:12:39 2015*

一回来上推就看到个有趣的东西——[Google Cloud Security Scanner](https://twitter.com/paradoxengine/status/568436963960877056)，
虽然目前只是Beta，主要只有XSS检测和分析功能，不过怎么说也是**Google**出品。

[![](/upload/gcsscan.png)](https://cloud.google.com/tools/security-scanner/)

亲测一番效果一般，可以选择前端测试环境。
有一点比较麻烦，目前只允许扫描自己有权限的Gae应用。
不过只要用Gae做个Proxy应用就能随意扫了。


没有太大亮点。不过比上不足比下有余，比起市面上大部份云扫描器算是好得多了。

--------------------

其实用`asyncio`写了半个Proxy应用来着，才反应过来Gae没有`Python 3.4`。
在Github里找了个[成品](https://github.com/kitek/GAE-http-proxy)，效果不错。
