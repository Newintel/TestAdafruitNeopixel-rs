use std::path::PathBuf;

fn build_my_lib() -> anyhow::Result<PathBuf> {
    let builder = embuild::bindgen::Factory::new()
    .cpp_builder()?
    .header("src/include/wrapper.h")
    .header("src/include/first.h")
    .allowlist_function("test_gen");
    
    embuild::bindgen::run(builder)
}

// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> anyhow::Result<()> {
    build_my_lib()?;

    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")
}
