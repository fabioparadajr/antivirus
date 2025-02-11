use std::path::PathBuf;
use notify::{Config, RecursiveMode, RecommendedWatcher, Watcher, EventKind};
use tokio::sync::mpsc;
use inotify::{Event, WatchMask};
use tokio::{task, select};

pub async fn monitor_directory(path: &str, x: mpsc::Sender<Option<PathBuf>>) -> Result<(), Box<dyn std::error::Error>> {

    let mut inotify = inotify::Inotify::init()?;

    inotify.watches().add(path, WatchMask::MODIFY | WatchMask::CREATE | WatchMask::MOVE_SELF)?;

    task::spawn(async move {
        let mut buffer = [0; 4096];
        loop {
            let result = inotify.read_events(&mut buffer);
            match result {
                Ok(events) => {
                    for event in events {
                        if event.mask.contains(inotify::EventMask::MODIFY) || event.mask.contains(inotify::EventMask::CREATE) || event.mask.contains(inotify::EventMask::MOVE_SELF) {
                            if let Some(name) = event.name {
                                let _ = x.send(Some(name.to_owned().into())).await;

                            }
                        }
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        continue;
                    } else {
                        println!("Error inotify:{:?}", e);
                        break
                    }

                }
            }
        }
    });

    Ok(())

}