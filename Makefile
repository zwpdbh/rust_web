# Ignore warnings for now
dev:
	RUSTFLAGS="-A unused -A dead_code " cargo build

run: 
	cargo build && RUST_LOG=info cargo run

get_questions:
	curl localhost:3030/questions?start=1&end=200