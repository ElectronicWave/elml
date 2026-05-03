use arc_swap::ArcSwap;
use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use std::{fmt::Debug, future::Future, pin::Pin, sync::Arc};
use tokio::task;

#[derive(Clone, Debug)]
pub enum Snapshot<T, E> {
    Idle,
    Loading {
        previous: Option<Arc<T>>,
    },
    Ready(Arc<T>),
    Error {
        error: Arc<E>,
        previous: Option<Arc<T>>,
    },
}
#[derive(Debug)]
pub struct Task<T, E> {
    pub snapshot: Arc<ArcSwap<Snapshot<T, E>>>,
}

impl<T, E> Default for Task<T, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, E> Clone for Task<T, E> {
    fn clone(&self) -> Self {
        Self {
            snapshot: self.snapshot.clone(),
        }
    }
}

impl<T, E> Task<T, E> {
    pub fn new() -> Self {
        Self {
            snapshot: Arc::new(ArcSwap::new(Snapshot::Idle.into())),
        }
    }

    pub fn set_snapshot(&self, snapshot: Snapshot<T, E>) {
        self.snapshot.store(Arc::new(snapshot));
    }
}
type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
pub struct FutureWidget<T, E> {
    loading: Box<dyn Fn(Rect, &mut Buffer, Option<Arc<T>>)>,
    ready: Box<dyn Fn(Rect, &mut Buffer, Arc<T>)>,
    error: Box<dyn Fn(Rect, &mut Buffer, Arc<E>, Option<Arc<T>>)>,
    factory: Box<dyn Fn() -> BoxFuture<Result<T, E>> + Send + Sync>,
}

impl<T, E> FutureWidget<T, E> {
    pub fn new() -> Self {
        Self {
            loading: Box::new(|_, _, _| {}),
            ready: Box::new(|_, _, _| {}),
            error: Box::new(|_, _, _, _| {}),
            factory: Box::new(|| panic!("Factory function is not set for FutureWidget")),
        }
    }
    pub fn factory(
        mut self,
        factory: impl Fn() -> BoxFuture<Result<T, E>> + Send + Sync + 'static,
    ) -> Self {
        self.factory = Box::new(factory);
        self
    }

    pub fn loading(mut self, render: impl Fn(Rect, &mut Buffer, Option<Arc<T>>) + 'static) -> Self {
        self.loading = Box::new(render);
        self
    }

    pub fn ready(mut self, render: impl Fn(Rect, &mut Buffer, Arc<T>) + 'static) -> Self {
        self.ready = Box::new(render);
        self
    }

    pub fn error(
        mut self,
        render: impl Fn(Rect, &mut Buffer, Arc<E>, Option<Arc<T>>) + 'static,
    ) -> Self {
        self.error = Box::new(render);
        self
    }
}
#[derive(Debug)]
pub struct FutureWidgetState<T, E> {
    pub task: Task<T, E>,
}

impl<T, E> Clone for FutureWidgetState<T, E> {
    fn clone(&self) -> Self {
        Self {
            task: self.task.clone(),
        }
    }
}

impl<T, E> Default for FutureWidgetState<T, E> {
    fn default() -> Self {
        Self::new(Task::new())
    }
}

impl<T, E> FutureWidgetState<T, E> {
    pub fn new(task: Task<T, E>) -> Self {
        Self { task }
    }
}

impl<T, E> StatefulWidget for FutureWidget<T, E>
where
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    type State = FutureWidgetState<T, E>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let snapshot = state.task.snapshot.load();
        match &**snapshot {
            Snapshot::Idle => {
                // Declare task has been obtained
                state
                    .task
                    .set_snapshot(Snapshot::Loading { previous: None });
                let task = state.task.clone();
                task::spawn(async move {
                    let result = (self.factory)().await;
                    match result {
                        Ok(value) => task.set_snapshot(Snapshot::Ready(Arc::new(value))),
                        Err(error) => task.set_snapshot(Snapshot::Error {
                            error: Arc::new(error),
                            previous: None,
                        }),
                    }
                });
            }
            Snapshot::Loading { previous } => {
                (self.loading)(area, buf, previous.clone());
            }

            Snapshot::Ready(value) => {
                (self.ready)(area, buf, value.clone());
            }

            Snapshot::Error { error, previous } => {
                (self.error)(area, buf, error.clone(), previous.clone());
            }
        }
    }
}
