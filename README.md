# How to run the benchmark

Make sure to download the notes from genesis

```shell
cargo run --release --bin download-state
```

And to build the WebAssembly module:

```shell
cd wasm-bench
cargo b --release
```

Use a tool to serve the contents of the `wasm-bench` directory, such as

https://github.com/thecoshman/http

Navigate to `index.html` and open the console, to eventually find the
benchmark written to the console log. Should take around 2mins, and blocking
the main thread. This could mean the browser warns to stop the script. Don't.

