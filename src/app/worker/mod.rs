use std::sync::Arc;

use async_channel::{Receiver, Sender};
use eframe::egui::Context;
use futures::executor::ThreadPoolBuilder;

use crate::app::App;

pub mod calculator;
pub mod header;
pub mod menu;

pub trait Task: Send + Sync {
    // fn key(&self) -> String;
    fn execute(&self) -> Option<Arc<dyn Event>>;
}

pub trait Event: Send + Sync {
    fn handle(&self, app: &mut App);
}

pub fn start(ctx: Context) -> (Sender<Arc<dyn Task>>, Receiver<Arc<dyn Event>>) {
    let worker = ThreadPoolBuilder::new().create().unwrap();
    let (task_s, task_r) = async_channel::unbounded::<Arc<dyn Task>>();
    let (event_s, event_r) = async_channel::unbounded::<Arc<dyn Event>>();

    let pool = worker.clone();
    worker.spawn_ok(async move {
        while let Ok(task) = task_r.recv().await {
            // todo: 任务去重
            let pool = pool.clone();
            let event_s = event_s.clone();
            let ctx = ctx.clone();
            pool.spawn_ok(async move {
                if let Some(event) = task.execute() {
                    event_s.send(event).await.ok();
                }
                ctx.request_repaint();
            });
        }
    });

    (task_s, event_r)
}
