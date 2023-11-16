## Learn Axum

### Lancer la commande pour le hot reload et les tests

```sh
cargo watch -q -c -w tests/ -x "test -q test_hello -- --nocapture"
```

### Lancer la commande pour compiler et exécuter le code

```sh
cargo watch -q -c -w src/ -x run
```
