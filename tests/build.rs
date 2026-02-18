// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad markdown test

use anyhow::anyhow;

/// markdown test main
fn main() {
    // ignore pre-build steps in rust-analyzer or clippy
    if std::env::var("RUST_ANALYZER_INTERNALS_DO_NOT_USE").is_ok()
        || std::env::var("CLIPPY_ARGS").is_ok()
        || std::env::var("SKIP_BUILD_RS").is_ok()
    {
        println!("cargo:warning=skipping build.rs because of environment var!");
        return;
    }

    // run copyright update/check depending on environment variables `COPYRIGHT_CHECK` and `COPYRIGHT_UPDATE`
    let check_only = std::env::var("COPYRIGHT_CHECK").is_ok();
    let update = std::env::var("COPYRIGHT_UPDATE").is_ok();
    if update || check_only {
        println!("cargo:warning=updating copyrights...");
        let check_failed = check_copyright(check_only).expect("copyright check failed");
        if check_failed {
            panic!("copyrights changed - please run: COPYRIGHT_UPDATE=1 cargo test")
        }
    }

    // update test banners in markdown books
    use update_md_banner::*;
    println!("cargo:warning=updating test banners...");
    update_md_banner("../books").expect("banner update failed");

    // generate rust tests from µcad code in markdown books
    println!("cargo:warning=generating mdbooks...");
    update_book("tests").expect("test generation failed");
    update_book("language").expect("test generation failed");
    update_book("tutorials").expect("test generation failed");
    update_book("examples").expect("test generation failed");
}

fn check_copyright(check_only: bool) -> anyhow::Result<bool> {
    Ok(update_copyright::update_copyrights(
        "../",
        &[
            ("#", &["toml"]),
            ("//", &["rs", "pest", "slint", "wgsl", "µcad"]),
        ],
        &[
            "../target/*",
            "../tests/*.µcad",
            "../crates/cli/examples/*.µcad",
            "../thirdparty/*",
        ],
        check_only,
    )?)
}

fn update_book(name: &str) -> anyhow::Result<()> {
    match microcad_markdown_test::generate(
        format!("../books/{name}/src"),
        format!("md_test_book_{name}.rs"),
        format!("../books/{name}/src/test_list.md"),
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!(
            "error generating rust test code from markdown book '{name}': {err}"
        )),
    }
}
