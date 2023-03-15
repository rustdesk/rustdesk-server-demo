# Een werkende demo van de RustDesk serverimplementatie
Dit is een super eenvoudige werkende demo-implementatie met slechts een toegestane relay-verbinding, zonder NAT traversal, persistentie, encryptie en andere geavanceerde functies. Maar het kan een goed uitgangspunt zijn om uw eigen RustDesk serverprogramma te schrijven. 

## Hoe uitvoeren
```bash
# install rustup first, https://rustup.rs/
IP=<public ip of this machine> cargo run
```

https://rustdesk.com/blog/id-relay-set/

https://github.com/rustdesk/rustdesk/issues/115
