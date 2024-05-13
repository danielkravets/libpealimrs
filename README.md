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