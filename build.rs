use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;

/// We need to have our assets moved into the build so we can access them.
/// Hence this build script.
fn main() -> Result<()> {
    // Tells Cargo to rerun if anything changes in our assets
    println!("cargo:rerun-if-changed=assets/*");

    let out_dir = env::var("OUT_DIR")?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();
    paths_to_copy.push("assets/");
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}