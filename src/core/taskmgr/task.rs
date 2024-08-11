use std::future::Future;

use dashmap::DashMap;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tracing::{debug, info};

pub struct KillableTask {
    name: String,
    handle: JoinHandle<()>,
    cancel_fn: oneshot::Sender<bool>
}

impl KillableTask {
    pub fn new<F>(name: String, fut: F) -> Self
    where F: Future<Output = ()> + Send + 'static
    {
        let (kill_sender, kill_rec) = oneshot::channel();
        let cloned_name = name.clone();

        let handle = tokio::spawn(async move {
            tokio::select! {
                _ = fut => {
                    info!("Handled task {}", cloned_name);
                },
                _ = kill_rec => {
                    info!("Killing task {}", cloned_name);
                    return;
                }
            }
        });

        KillableTask {
            name,
            handle,
            cancel_fn: kill_sender
        }
    }

    pub async fn kill(self) {
        let _ = self.cancel_fn.send(true);
        let _ = self.handle.await;
    }
}

pub struct Manager {
    tasks: DashMap<String, KillableTask>
}

impl Manager {
    pub fn new() -> Self {
        Manager { tasks: DashMap::new() }
    }

    pub fn has_task(&self, name: String) -> bool {
        self.tasks.contains_key(&name)
    }

    pub async fn kill_task(&self, name: String) -> bool {
        let (task_name, killable_task) = self.tasks.remove(&name).unwrap();

        let _ = killable_task.kill().await;

        debug!("Task {} killed", &task_name);

        true
    }

    pub fn add_task(&self, handle: KillableTask) -> bool {
        self.tasks.insert(handle.name.clone(), handle);
        true
    }
}