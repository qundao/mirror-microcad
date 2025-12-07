//! µcad viewer ipc manual test

use microcad_viewer_ipc::*;

/// Test show hide
fn test_show_hide_window() -> anyhow::Result<()> {
    env_logger::init();
    let search_paths = vec![std::env::current_dir()?];
    let viewer = ViewerProcessInterface::run(&search_paths, false); // Start hidden

    // Show window
    viewer.send_request(ViewerRequest::Show)?;

    // Manually verify the window appears.

    // Wait for user confirmation
    println!("Press Enter after verifying the window is visible...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // Hide window
    viewer.send_request(ViewerRequest::Hide)?;

    // Manually verify the window disappears.

    // Wait for user confirmation
    println!("Press Enter after verifying the window is visible...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    //  export MICROCAD_VIEWER_BIN

    test_show_hide_window()
}
