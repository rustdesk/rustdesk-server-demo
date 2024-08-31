<p align="center">
  [<a href="README-DE.md">Deutsch</a>] | [<a href="README-ES.md">Español</a>] | [<a href="README-FR.md">Français</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-PL.md">Polski</a>] | [<a href="README-PTBR.md">Português (Brasil)</a>] ｜ [<a href="README-ZH.md">简体中文</a>] <br>
</p>

# Een werkende demo van de RustDesk serverimplementatie
Dit is een super eenvoudige werkende demo-implementatie met slechts een toegestane relay-verbinding, zonder NAT traversal, persistentie, encryptie en andere geavanceerde functies. Maar het kan een goed uitgangspunt zijn om uw eigen RustDesk serverprogramma te schrijven. 

## Hoe uitvoeren
```bash
# install rustup first, https://rustup.rs/
IP=<public ip of this machine> cargo run
```

https://rustdesk.com/blog/id-relay-set/

https://github.com/rustdesk/rustdesk/issues/115
