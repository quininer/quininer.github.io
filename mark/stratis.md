# Stratis
## *Sun Apr 15 18:14:39 2018*


[Stratis] 是 redhat 放棄 Btrfs 之後推出的代替品，
使用類似 LVM 的方案來提供有如 btrfs、zfs 般強大的功能。

其實 [Stratis] 釋出 0.5 有一段時間了，今次抽出時間來嚐下鮮。
[Stratis] 目前沒有面向用戶的文檔，故久違的寫一篇博文用以記錄。


準備工作
--------

從 AUR 安裝 [stratisd](https://aur.archlinux.org/packages/stratisd/)
同 [stratis-cli](https://aur.archlinux.org/packages/stratis-cli/)，
然後啟動 stratisd

```
$ systemctl start stratisd
```

開始
----

首先我們清理要用的塊設備

```
$ sudo wipefs -a /dev/sda
```

但當前版本創建 pool 的時候會碰到錯誤，stratisd 認爲這個塊設備已經被使用

```
$ stratis pool create stratis /dev/sda
Execution failed: 1: Device /dev/sda appears to belong to another application
```

根據 [issue](https://github.com/stratis-storage/stratisd/issues/822#issuecomment-371585235)，
如果塊設備的前 `4`/`8`k 字節未置零，stratisd 不會使用它。

知道原因就簡單了，我們將它置零

```
$ dd if=/dev/zero of=/dev/sda bs=1024 count=8
```

然後就可以正常創建 pool

```
$ stratis pool create stratis /dev/sda
$ stratis pool list
Name       Total Physical Size  Total Physical Used
stratis           > 238.47 GiB               52 MiB
```

隨後創建 fs

```
$ stratis fs create stratis storage
$ stratis fs list stratis
Name
storage
$ la /dev/stratis/stratis/storage
Permissions Size User Date Modified Name
lrwxrwxrwx     9 root 15 4月  18:21 /dev/stratis/stratis/storage -> /dev/dm-6
```

完成
----

創建完成，直接掛載就可以使用啦

```
$ sudo mount /dev/stratis/stratis/storage /run/media/quininer/storage
$ sudo chown quininer /run/media/quininer/storage
```

用得開心。 ;-)


[Stratis]: https://stratis-storage.github.io/
