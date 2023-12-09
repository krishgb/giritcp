default:
	clear && cargo run

build:
	clear && cargo build --release

release:
	clear && cargo run --release

fmt:
	clear && cargo fmt

check:
	clear && cargo check

call:
	clear && curl localhost:9090