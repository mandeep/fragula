use std::path::Path;
use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};
use notify::{RawEvent, RecommendedWatcher, RecursiveMode, Watcher};

/// Create channels to be used with the notify crate
///
/// The sender and receiver need to be cloned so that two separate
/// threads can rely on the same signals. This allows the expression
/// `match receiver.recv()` to be used in the spawn_watcher function
/// as well as in the render loop.
pub fn create_channels(
    )
    -> (Sender<RawEvent>, Receiver<RawEvent>, Sender<RawEvent>, Receiver<RawEvent>)
{
    let (sender, receiver) = unbounded();
    let (messenger, collector) = (sender.clone(), receiver.clone());

    (sender, receiver, messenger, collector)
}

/// Spawn a watcher thread that watches the given file for edits
pub fn spawn_watcher(file: &Path,
                     sender: Sender<RawEvent>,
                     receiver: Receiver<RawEvent>,
                     messenger: Sender<RawEvent>) {
    let fragment_dirpath = file.parent().unwrap().display().to_string();

    thread::spawn(move || {
        let mut watcher: RecommendedWatcher = Watcher::new_immediate(sender).unwrap();
        watcher.watch(fragment_dirpath, RecursiveMode::NonRecursive).unwrap();

        loop {
            match receiver.recv() {
                Ok(event) => {
                    messenger.send(event).unwrap();
                }
                Err(err) => println!("watch error: {:?}", err),
            };
        }
    });
}
