# Talking n Algos


## How to run stuff?

You can run tests via the commands below


```sh
cargo test --all --lib -- --nocapture
```


```sh
cargo test --all --release --lib -- --nocapture
```


```sh
cargo test --all --verbose --release --lib -- --nocapture
```


### Target a specific test

You can also pass the `--release` flag here also

```sh
cargo test test_base_binary_search --lib -- --nocapture
```
