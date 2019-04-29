use crate::error::Error;
use crate::layouts::Entry;
use crate::CompiledShaders;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub struct Watch {
    _handler: Handler,
    pub rx: Receiver<Result<Message, Error>>,
}

struct Loader {
    vertex: PathBuf,
    fragment: PathBuf,
    tx: Sender<Result<Message, Error>>,
}

pub struct Message {
    pub shaders: CompiledShaders,
    pub entry: Entry,
}

impl Watch {
    pub fn new<T>(vertex: T, fragment: T) -> Self
    where
        T: AsRef<Path>,
    {
        let (handler, rx) = create_watch(
            vertex.as_ref().to_path_buf(),
            fragment.as_ref().to_path_buf(),
        );
        Watch {
            _handler: handler,
            rx,
        }
    }
}

impl Loader {
    fn create(vertex: PathBuf, fragment: PathBuf) -> (Self, Receiver<Result<Message, Error>>) {
        let (tx, rx) = mpsc::channel();
        let loader = Loader {
            vertex,
            fragment,
            tx,
        };
        loader.reload();
        (loader, rx)
    }

    fn reload(&self) {
        match crate::load(&self.vertex, &self.fragment) {
            Ok(shaders) => {
                let entry = crate::parse(&shaders);
                self.tx.send(Ok(Message { shaders, entry })).ok()
            }
            Err(e) => self.tx.send(Err(e)).ok(),
        };
    }
}

struct Handler {
    thread_tx: mpsc::Sender<()>,
    handle: Option<thread::JoinHandle<()>>,
    _watcher: RecommendedWatcher,
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.thread_tx.send(()).ok();
        if let Some(h) = self.handle.take() {
            h.join().ok();
        }
    }
}

fn create_watch(
    mut vert_path: PathBuf,
    mut frag_path: PathBuf,
) -> (Handler, mpsc::Receiver<Result<Message, Error>>) {
    let (notify_tx, notify_rx) = mpsc::channel();
    let (thread_tx, thread_rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(notify_tx, Duration::from_millis(50)).expect("failed to create watcher");

    vert_path.pop();
    frag_path.pop();
    watcher
        .watch(&vert_path, RecursiveMode::NonRecursive)
        .expect("failed to add vertex shader to notify");
    watcher
        .watch(&frag_path, RecursiveMode::NonRecursive)
        .expect("failed to add fragment shader to notify");

    let (loader, rx) = Loader::create(vert_path, frag_path);

    let handle = thread::spawn(move || 'watch_loop: loop {
        if let Ok(_) = thread_rx.try_recv() {
            break 'watch_loop;
        }
        if let Ok(notify::DebouncedEvent::Create(_))
        | Ok(notify::DebouncedEvent::Write(_)) = notify_rx.recv_timeout(Duration::from_secs(1))
        {
            loader.reload();
        }
    });
    let handle = Some(handle);
    let handler = Handler {
        thread_tx,
        handle,
        _watcher: watcher,
    };
    (handler, rx)
}
