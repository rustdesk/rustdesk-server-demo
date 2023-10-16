<p align="center">
  [<a href="README.md">English</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-PL.md">Polski</a>]<br>
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
