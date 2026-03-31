build:
	cargo build --workspace

test:
	cargo test --workspace

run-shell:
	cargo run -p librery-shell

generate-schema:
	cargo run -p librery-shell -- generate-schema
