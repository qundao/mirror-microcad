//! µcad viewer ipc manual test

use microcad_viewer_ipc::*;

fn prompt_for_confirmation(prompt: &str) -> anyhow::Result<bool> {
    loop {
        println!("{} (y/n)", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}

/// Test show hide
fn test_show_hide_window() -> anyhow::Result<()> {
    env_logger::init();
    let search_paths = vec![std::env::current_dir()?];
    let viewer = ViewerProcessInterface::run(&search_paths, false); // Start hidden

    let mut cycle = 0;

    loop {
        log::info!("Sending 'Show' request...");
        viewer.send_request(ViewerRequest::Show)?;
        if !prompt_for_confirmation("Is the window visible?")? {
            return Err(anyhow::anyhow!("Window did not appear as expected."));
        }

        log::info!("Sending 'Hide' request...");
        viewer.send_request(ViewerRequest::Hide)?;
        if !prompt_for_confirmation("Is the window hidden?")? {
            return Err(anyhow::anyhow!("Window did not hide as expected."));
        }
        cycle += 1;

        log::info!("Show/Hide Cycle #{cycle}")
    }
}

fn main() -> anyhow::Result<()> {
    //  export MICROCAD_VIEWER_BIN

    test_show_hide_window()
}
