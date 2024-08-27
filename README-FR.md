<p align="center">
  [<a href="README-DE.md">Deutsch</a>] | [<a href="README-ES.md">Español</a>] | [<a href="README-FR.md">Français</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-PL.md">Polski</a>] | [<a href="README-PT.md">Português</a>]<br>
</p>

# Une démonstration pratique de l’implémentation du serveur RustDesk

Il s’agit d’une implémentation de démonstration très simple avec une seule connexion de relais autorisée, sans traversée de NAT, persistance, cryptage et autres fonctionnalités avancées. Mais cela peut être un bon point de départ pour écrire votre propre programme de serveur RustDesk.

## Comment exécuter ?

```bash
# installer rustup d'abord, https://rustup.rs/
IP=<adresse ip publique de cette machine> cargo run
```

https://rustdesk.com/blog/id-relay-set/

https://github.com/rustdesk/rustdesk/issues/115
