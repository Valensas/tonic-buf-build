# tonic-buf-build

A build helper that allows you to integrate [buf.build](https://buf.build) with [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).
Using buf.build and tonic, you can easily manage third party dependencies for proto files and generate code for your proto files in Rust.
Works with both [buf.yaml](https://buf.build/docs/configuration/v1/buf-yaml) and [buf.work.yaml](https://buf.build/docs/configuration/v1/buf-work-yaml).

## Usage

Add the following to your Cargo.toml:

```toml
tonic-buf-build = "*"
tonic-build = "*"
```

Then, in your build.rs:

```rust
fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
   tonic_buf_build::compile_from_buf(tonic_build::configure(), None)?;
   Ok(())
}
```

To use buf workspaces, simply call `tonic_buf_build::compile_from_buf_workspace` instead.

For complete and working examples, take a look at the examples folder.
