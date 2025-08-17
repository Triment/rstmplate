```
sqlx migrate add initalize -r
```
```
cargo run test --workspace
```


```bash
cargo run -p plugin -- gen #生成签名证书，保存好，私钥给签名插件，公钥校验签名
cargo run -p plugin -- sign target_plugin.{ dylib | dll | so } #签名，默认使用ed25519_sk.bin文件签名
cargo run -p plugin -- verify target_plugin.{ dylib | dll | so } #校验签名
```