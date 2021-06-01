# start
```bash
$ docker-compose up -d
$ cat ./schema/*.sql | docker-compose exec -T db mysql -u dev -pdev user
$ RUST_LOG=sql=info,actix=info cargo run -- --workers 1 --web-port 8080 --mysql dev:dev@localhost:8888/user
```

# service
```
curl -X POST -H "Content-Type: application/json" -d '{"name":"taro yamada", "mailadress":"mail@mail", "password":"12345"}' localhost:8080/create  
curl -X POST -H "Content-Type: application/json" -d '{"name":"jiro suzuki", "mailadress":"mail2@mail2", "password":"12345"}' localhost:8080/create  
curl -X GET localhost:8080/users 
curl -X PUT -H "Content-Type: application/json" -d '{ "password":"abcde" }' localhost:8080/update/1
curl -X PUT localhost:8080/delete/1
curl -X GET localhost:8080/user/1 
curl -X DELETE localhost:8080/delete/physics/1
```

# doc
```
https://github.com/launchbadge/sqlx
https://crates.io/crates/sqlx
https://docs.rs/crate/sqlx
```