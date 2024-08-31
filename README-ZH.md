<p align="center">
  [<a href="README-DE.md">Deutsch</a>] | [<a href="README-ES.md">Español</a>] | [<a href="README-FR.md">Français</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-PL.md">Polski</a>] | [<a href="README-PTBR.md">Português (Brasil)</a>] ｜ [<a href="README-ZH.md">简体中文</a>] <br>
</p>

# RustDesk 服务器实现的演示
这是一个非常简易的实现，只允许一个中继连接，没有NAT穿透、持久性、加密和其他任何高级功能。但是它可以作为你编写你自己的RustDesk服务器程序很好的起点。

## 如何运行
```bash
# 首先安装 rustup, https://rustup.rs/
IP=<该机器的公网IP> cargo run
```

https://rustdesk.com/blog/id-relay-set/

https://github.com/rustdesk/rustdesk/issues/115