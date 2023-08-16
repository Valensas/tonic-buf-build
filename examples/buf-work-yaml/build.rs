fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    tonic_buf_build::compile_from_buf_workspace(tonic_build::configure(), None)?;
    Ok(())
}
