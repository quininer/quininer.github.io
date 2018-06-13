# Surface Pro 3
## *Mon Mar 30 17:19:12 2015*

![Surface Pro 3](/upload/surface.jpg)

事情是這樣的，我買了一臺 Surface Pro 3，並給它裝了 Arch Linux。

關於Windows
-----------

用了兩天 Windows 8，其實感覺還不錯，按照之前的[想法](https://twitter.com/quininers/status/555389735594496003)，
試圖裝上
[Cmder](http://gooseberrycreative.com/cmder/) 和
[msys2](https://msys2.github.io/)
來讓 Windows 上的終端也達到可用狀態。

[效果](https://twitter.com/quininers/status/581435255137030144)不盡人意，

* cmder 不知是字體或是什麼問題，Vim 下字符會混亂，有點像不等寬字體，而且還有鍵捕獲不到以及亂碼的奇怪問題
* msys2 的包太少，完全不夠用
* mingw 不會用= =
* 一開始 msys2 用 pacman 裝的 Mingw Python 找不到，需要手動把`/mingw64/bin`加入 PATH
* 試圖安裝上 yaourt，但是依賴的依賴 yajl 編譯報錯，似乎還要做許多設置

雖然 Windows 的觸屏體驗確實很棒，但是我早有裝 Arch Linux 的想法。所以上面的問題只是藉口。

關於Arch Linux
--------------

自從 Debian 換成 Arch Linux 之後便很喜歡。

### Secure Boot
開機時`音量鍵上 + 開機鍵`進 UEFI 關掉就好了，
雖然不關也能通過[這個方法](https://wiki.archlinux.org/index.php/Unified_Extensible_Firmware_Interface#Secure_Boot)來啓動，
但是實際上安裝完成之後依然會提示錯誤。

### Install
雖然 [reddit](http://www.reddit.com/r/SurfaceLinux/) 上都說 WiFi 驅動在 3.19 內核才被合併，
但是被 #archlinux-cn 頻道的 Feb-Chip 告知親測 Arch Linux 目前 3.18 內核的鏡像也能直接使用。
livecd 裏的 tty 下不能使用 Type Cover 鍵盤(猜測 3.19 內核之後就可以了)，所以使用 SD卡 引導。
和 U盤 一樣，`音量鍵下 + 開機鍵`即可。

安裝過程完全參考[新手指南](https://wiki.archlinux.org/index.php/Beginners%27_guide)，
沒有碰上特殊問題。

### KDE
HiDPI 依然按照 [wiki](https://wiki.archlinux.org/index.php/HiDPI) 配置。

Firefox 效果不錯，
而[Chromium 效果糟糕](https://twitter.com/quininers/status/582090512603033600)，
`--force-device-scale-factor=2` 就像被直接放大，沒有任何優化。
比較好的方案是手動把網頁調到 175% 的大小。

[電磁筆](https://github.com/nuclearsandwich/surface3-archlinux/issues/8)和觸屏幾乎完美。
可惜沒有什麼對觸屏有優化的應用。
這一點 Chromium 做的比 Firefox 好。
配置一下可以讓電磁筆的[按鈕](https://github.com/nuclearsandwich/surface3-archlinux/issues/4#issuecomment-68158518)起些作用。

當然 Type Cover 也要[配置](https://github.com/nuclearsandwich/surface3-archlinux/issues/4#issuecomment-62277004)。
~~不過我碰到了一些問題，
一旦碰到 Type Cover 的觸摸板，X 便段錯誤崩潰了。
我沒有找到類似的案例，推測是個例。~~

此問題已解決，
更新 linux-surfacepro3 內核時重新配置了一下 `/etc/X11/xorg.conf.d/50-synaptics.conf`，觸摸板使用正常~

### Kernel
編譯 aur 裏的 [linux-surfacepro3](https://aur.archlinux.org/packages/linux-surfacepro3/) 內核，
最主要的是打上了電源補丁，終於可以看到電量了。

装内核時碰上一個坑，/tmp 掛載的大小只有 2G，因爲之前從 aur 打包了幾個軟件而內核又很大，
所以打包 linux-surfacepro3 時塞滿了 /tmp，導致打包失敗。
編譯了兩次之後才發覺..
默默修改 /etc/yaourtrc 中的`TMPDIR`，

    TMPDIR="$HOME/Aurs"

### Battery
在 Windows 下粗略計過一下續航，<ruby>平常使用<rt>瀏覽網頁</rt></ruby>確實能有 8h 之長。
換到 Linux 之後，用秒錶計算，提示 10% 的電量時，续航 4:29:59 。其中包括 40m- 的 FEZ 遊戲時間。

估計用 [laptop-mode](https://wiki.archlinux.org/index.php/Laptop_Mode_Tools) 之類的動態調頻能延長一些續航。

最後，編譯 YCM 只用了 2分8秒，而上一個筆記本需要 5分鐘 左右。

----------------------------------------

寫完博客第二天 linux-surfacepro3 就[更新](https://github.com/matthewwardrop/linux-surfacepro3)了，
不得不再編譯一次內核。
過程順利。

順便找到一個[自动旋转脚本](https://github.com/AykoPoel/surface3-scripts)，
雖然不夠流暢，但也可用。

剩下唯一的問題就是 KDE 缺少一個好用的屏幕鍵盤了吧。

----------------------------------------

在 Laptop-mode 下測試，正常使用至 10% 電量，續航 6:21:40。

參考
----
* [Unified Extensible Firmware Interface - Secure Boot](https://wiki.archlinux.org/index.php/Unified_Extensible_Firmware_Interface#Secure_Boot)
* [Beginners' guide](https://wiki.archlinux.org/index.php/Beginners%27_guide)
* [HiDPI](https://wiki.archlinux.org/index.php/HiDPI)
* [N-Trig Touch Pen - Works great but I have no idea how to configure it!](https://github.com/nuclearsandwich/surface3-archlinux/issues/8)
* [Type Cover 3 not working even after kernel patch](https://github.com/nuclearsandwich/surface3-archlinux/issues/4)
* [aur - linux-surfacepro3](https://aur.archlinux.org/packages/linux-surfacepro3/)
* [Laptop_Mode_Tools](https://wiki.archlinux.org/index.php/Laptop_Mode_Tools)
* [github - linux-surfacepro3](https://github.com/matthewwardrop/linux-surfacepro3)
* [Surface3-Scripts - autorotate.py](https://github.com/AykoPoel/surface3-scripts)
