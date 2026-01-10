
Run quote_server:
```sh 
	cargo run -p streaming_quotes_project --bin quote_server --features 'sqlite random logging'
```

Run quote_client:
```sh 
	cargo run -p streaming_quotes_project --bin quote_client --features 'sqlite random logging'
```