
run: build run-only

run-only:
	deno run --allow-read run.js

build:
	wasm-pack build --release -t web
