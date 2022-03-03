proto:
	python3 -m grpc_tools.protoc  -I . --python_out=. --grpc_python_out=. ./python/user.proto
run_python_client:
	python3 ./python/client.py
run_rust_server:
	cargo run --bin user-server