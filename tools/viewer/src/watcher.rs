use std::{sync::mpsc::channel, time::Duration};

use bevy::ecs::{event::EventWriter, system::ResMut};

use notify::{RecursiveMode, Watcher};

use crate::{processor::ProcessorRequest, state::State};

/// Whether a kind of watch event is relevant for compilation.
fn is_relevant_event_kind(kind: &notify::EventKind) -> bool {
    match kind {
        notify::EventKind::Any => false,
        notify::EventKind::Access(_) => false,
        notify::EventKind::Create(_) => true,
        notify::EventKind::Modify(kind) => match kind {
            notify::event::ModifyKind::Any => true,
            notify::event::ModifyKind::Data(_) => true,
            notify::event::ModifyKind::Metadata(_) => true,
            notify::event::ModifyKind::Name(_) => true,
            notify::event::ModifyKind::Other => false,
        },
        notify::EventKind::Remove(_) => true,
        notify::EventKind::Other => false,
    }
}

pub fn start_file_watcher(state: ResMut<State>) {
    let flag_clone = state.last_modified.clone();
    let path = state.input.clone();

    std::thread::spawn(move || -> ! {
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(tx).unwrap();
        watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

        println!("Watching external file: {}", path.display());

        loop {
            if let Ok(Ok(event)) = rx.recv_timeout(Duration::from_millis(500))
                && is_relevant_event_kind(&event.kind)
                && let Ok(meta) = std::fs::metadata(&path)
                && let Ok(modified) = meta.modified()
            {
                log::info!("Modified");
                *flag_clone.lock().unwrap() = Some(modified);
                watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();
            }
        }
    });
}

pub fn handle_external_reload(
    mut event_writer: EventWriter<ProcessorRequest>,
    state: ResMut<crate::state::State>,
) {
    let mut last_modified_lock = state.last_modified.lock().unwrap();

    if let Some(last_modified) = *last_modified_lock
        && let Ok(elapsed) = last_modified.elapsed()
        && elapsed > state.settings.reload_delay
    {
        event_writer.write(ProcessorRequest::Render);
        log::info!("Changed file");

        // Reset so we don’t reload again
        *last_modified_lock = None;
    }
}
