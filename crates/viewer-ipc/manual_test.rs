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
    let search_paths = microcad_builtin::dirs::default_search_paths();
    let viewer = ViewerProcessInterface::run(&search_paths, false); // Start hidden

    let mut cycle = 0;

    loop {
        viewer.send_request(ViewerRequest::Show)?;

        if !prompt_for_confirmation("Is the window visible?")? {
            return Err(anyhow::anyhow!("Window did not appear as expected."));
        }

        viewer.send_request(ViewerRequest::ShowSourceCode {
            path: None,
            name: Some("Test".to_string()),
            code: format!(
                r#" 
                use std::geo2d::Text;
                Text("{cycle}", height = 10mm);
            "#
            ),
        })?;

        if !prompt_for_confirmation("Is there a number?")? {
            return Err(anyhow::anyhow!("Invalid source code"));
        }

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
