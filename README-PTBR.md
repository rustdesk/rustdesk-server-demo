<p align="center">
  [<a href="README-DE.md">Deutsch</a>] | [<a href="README-ES.md">Español</a>] | [<a href="README-FR.md">Français</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-PL.md">Polski</a>] | [<a href="README-PTBR.md">Português (Brasil)</a>]<br>
</p>

# Uma demonstração funcional da implementação do servidor RustDesk
Esta é uma demonstração super simples funcionando da implementação com apenas uma conexão de retransmissão permitida, sem NAT traversal, persistência, criptografia nem outros recursos avançados. Mas pode ser um bom ponto de partida para escrever seu próprio programa de servidor RustDesk.

## Como rodar
```bash
# primeiro instalar o rustup, https://rustup.rs/
IP=<ip pública desta máquina> cargo run
```

https://rustdesk.com/blog/id-relay-set/

https://github.com/rustdesk/rustdesk/issues/115
