# libpealimrs
libpealimrs

# Build wasm
```wasm-pack build --features wasm-support```

# Compile protobuf bindings
```cargo run --bin protoc```

# run profiler on example app (MACOS, from example dir)
## allocations
```cargo instruments --template "Allocations" --open```
## time profiler
```cargo instruments --template "Time Profiler" --open```


# release for ios
```cargo build --release --lib --target aarch64-apple-ios```
```cargo build --release --lib --target aarch64-apple-ios```