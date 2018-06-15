# digit-recognition

```
$ cargo build
$ cargo run --bin client
$ cargo run --bin server
```

[compile error](https://github.com/SergioBenitez/Rocket/issues/660)

```
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"image_base64":""}' \
  http://localhost:8080/image
```


curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"image_base64":""}' \
  http://localhost:8000/image
