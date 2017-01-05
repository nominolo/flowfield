default: run

run:
	rustup run nightly cargo run

build:
	rustup run nightly cargo build

test:
	rustup run nightly cargo test	

bench:
	rustup run nightly cargo bench