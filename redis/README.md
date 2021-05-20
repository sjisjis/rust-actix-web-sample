# start
```bash
$ docker-compose up -d
$ cargo run -- --workers 1 --web-port 8080 --redis localhost:6379/0
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