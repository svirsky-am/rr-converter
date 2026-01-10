
Run monitor:
```sh 
	cargo run -p streaming_quotes_project --bin monitor --features 'sqlite random logging'
```

Run sender:
```sh 
	cargo run -p streaming_quotes_project --bin sensor_simulator --features 'sqlite random logging'
```