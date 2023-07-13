
### Build & Run

```shell
cargo run
```

### Docker Build & Run

```shell
docker build . -t rust-api
docker run -it -p 8080:8080 rust-api
```

### Sample URLs

- http://localhost:8080/api/v1/hello
- http://localhost:8080/api/v1/hello?name=Zaphrod

