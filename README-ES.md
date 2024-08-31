<p align="center">
   [<a href="README-DE.md">Deutsch</a>] | [<a href="README-ES.md">Español</a>] | [<a href="README-FR.md">Français</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-PL.md">Polski</a>] | [<a href="README-PTBR.md">Português (Brasil)</a>] ｜ [<a href="README-ZH.md">简体中文</a>] <br>
</p>

# Una demostración funcional de la implementación del servidor RustDesk
Esta es una demostración super simple funcionando de la implementación con sólo una conexión de retransmisión permitida, sin NAT transversal, persistencia, encriptación ni ninguna otra característica avanzada. Pero puede ser un buen punto de partida para escribir tu propio programa del servidor RustDesk.

## Como correr
```bash
# primero instalar rustup, https://rustup.rs/
IP=<ip pública de esta máquina> cargo run
```

https://rustdesk.com/blog/id-relay-set/

https://github.com/rustdesk/rustdesk/issues/115
