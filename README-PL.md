<p align="center">
   [<a href="README.md">English</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-NL.md">Nederlands</a>]<br>
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
