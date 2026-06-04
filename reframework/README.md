# Rust bindings for reframework
This library provides "safe" rust bindings for [reframework](https://github.com/praydog/REFramework). Currently mainly just for `sdk`, but also has unsafe `imgui` bindings.

It should be pretty similar to using lua or c++ (I haven't really messed with ref in c++ though).

Most functions bindings also end up returning an `Option<T>`, so you can use the `?` operator to send it up out of a function.

I also provide an implementation for `Logger` from the `log` crate, so you can use `log::info!` etc. These functions output to a specified log file and reframework's log.

A template plugin is available [here](https://github.com/kvasszn/ree-lib-rs/tree/main/ref-template-plugin).

## Building
Copy the template and the build.
```sh
cargo build -p ref-plugin-template --target x86_64-pc-windows-msvc --release 
```

On linux, i use cargo-xwin to build.
```sh
cargo xwin build -p ref-plugin-template --target x86_64-pc-windows-msvc --release
```

## TODO
- [ ] there's a decent amount of stuff missing, i.e some structs
- [ ] Add `Result<T, REFrameworkError>` for some returns
- [ ] Add il2cpp codegen with procmacros
    - to make this useful, requires casting pointers to objects to their real types ig or something like that
- [x] Add imgui bindings
- [ ] Add egui/other rust ui stuff (would need to make a renderer for dx12, or maybe wgpu witchcraft?? to make it kinda generic?)
- [ ] maybe add lua stuff? not sure what the use case would be.
- [x] impl traits like `Display` for `TypeDefinition`, `Method`, etc
- [ ] nice tooling things and helpers idk
