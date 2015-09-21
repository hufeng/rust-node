# rust-node
使用rust去写node的addons，通过将rust编译为dynamic lib, node通过ffi调用。


探索将Rust和Node结合起来使用，不用去学习C或者C++去写扩展。


# Getting Started
```sh
  brew update
brew install rust
```


# build rust
```sh
cargo build --release
```


# npm dependencies
```sh
npm install
```

# test
node test.js

```javascript
λ rust-node git:(master) ✗ node test.js
fib 10: 144
add 10,10 20

```
