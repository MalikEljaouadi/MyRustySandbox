# my_rusty_sandbox

## Build

```sh
cargo build
```

## Run

```sh
# terminal 1
RUST_LOG=debug cargo run

# terminal 2
curl 'http://localhost:8080/health'
```

## Tests

```sh
cargo test
```

## TODO

- [ ] Work on the dependancies.
- [ ] Work on the axum-testing.
- [ ] Work on the CI.