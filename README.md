# Rust (actix-web) sample
* redis(ping, set, get)の確認

# 参考
1. doc https://doc.rust-jp.rs
1. playground https://play.rust-lang.org/
1. actix-web https://actix.rs/
             https://docs.rs/actix-web/


# start
```bash
# redis
$ docker-compose up -d
# localhost:8080
$ cargo run
```

# service
```
# redis ping check
localhost:8080/watcher

# redis set
localhost:8080/set?id=1

# redis get
localhost:8080/get?id=1
localhost:8080/get?id=1&text=xyz
```