<p align="center">
  [<a href="README-DE.md">Deutsch</a>] | [<a href="README-ES.md">Español</a>] | [<a href="README-FR.md">Français</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-PL.md">Polski</a>] | [<a href="README-PTBR.md">Português (Brasil)</a>] ｜ [<a href="README-ZH.md">简体中文</a>] <br>
</p>

# Eine funktionierende Demo der RustDesk Server-Implementierung
Dies ist eine supereinfache, funktionierende Demo-Implementierung, die nur eine Relay-Verbindung erlaubt, ohne NAT-Traversal, Persistenz, Verschlüsselung und andere erweiterte Funktionen. Aber es kann ein guter Ausgangspunkt sein, um ein eigenes RustDesk Server-Programm zu schreiben.

## Wie es funktioniert
```bash
# Installieren Sie rustup zuerst, https://rustup.rs/
IP=<öffentliche ip dieses rechners> cargo run
```

https://rustdesk.com/blog/id-relay-set/

https://github.com/rustdesk/rustdesk/issues/115
