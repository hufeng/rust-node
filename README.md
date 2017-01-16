# 探索使用rust去构建node的addon

# Why Rust?

Rust is awesome. So we can hack without fear!

# rust-node
使用rust去写node的addons，通过将rust编译为dynamic lib, node通过ffi调用。

# FFI

```sh
#cd ffi/native/fib
#cargo build --release

# test it
# node __test__.js
```

# Neon
```sh
cd neon/
#npm run install or yarn run install
# test it.
#node lib/__test__.js
```
