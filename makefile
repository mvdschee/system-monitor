dev:
	watchexec -q -c -w src --exts rs --restart "cargo run"

build:
	cargo build --release

start:
	cargo run --release

test:
	cargo test -- --nocapture

prod:
	cd ./target/release/ && ./system-monitor
