<p align="center">
  [<a href="README-DE.md">Deutsch</a>] | [<a href="README-ES.md">Español</a>] | [<a href="README-FR.md">Français</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-PL.md">Polski</a>] | [<a href="README-PTBR.md">Português (Brasil)</a>] | [<a href="README-ZH.md">简体中文</a>] <br>
</p>

# Działające demo implementacji serwera RustDesk
Jest to bardzo prosta implementacja demonstracyjna z dozwolonym tylko jednym połączeniem, bez tłumaczenia NAT, trwałości, szyfrowania i innych zaawansowanych funkcji. Może to być dobry punkt wyjścia do napisania własnego serwera RustDesk.

## Jak uruchomić
```bash
# zainstaluj najpierw rustup, https://rustup.rs/
IP=<publiczny adres IP tej maszyny> cargo run
```

https://rustdesk.com/blog/id-relay-set/

https://github.com/rustdesk/rustdesk/issues/115
