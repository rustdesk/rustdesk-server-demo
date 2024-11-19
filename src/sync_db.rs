use std::{future::Future, pin::Pin, sync::Arc, time::Instant};

use hbb_common::{tokio::sync::RwLock, ResultType};

use crate::common::CycleLoop;


pub(crate) type SyncTask = Pin<Box<dyn Future<Output = ResultType<()>> + Send>>;

pub(crate) struct CycleTaskDb {
    inner: Arc<RwLock<CycleTaskDbInner>>
}
unsafe impl Send for CycleTaskDb {}
unsafe impl Sync for CycleTaskDb {}
struct CycleTaskDbInner {
    sync_to_db: SyncDb<SyncTask>,
    db_step: u64,
    expire: std::time::Instant,
}
unsafe impl Send for CycleTaskDbInner {}
unsafe impl Sync for CycleTaskDbInner {}
impl CycleTaskDb {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(CycleTaskDbInner {
                sync_to_db: SyncDb::new(),
                db_step: 0,
                expire: std::time::Instant::now(),
            }))
        }
    }
    pub(crate) async fn insert_sync_task(&self, task: SyncTask) {
        self.inner.write().await.sync_to_db.push_task(task);
    }
    pub(crate) async fn inc_step(&self) {
        let mut inner = self.inner.write().await;
        inner.db_step = inner.db_step + 1;
        drop(inner);
    }
    pub(crate) async fn update_expire(&self) {
        self.inner.write().await.expire = std::time::Instant::now();
    }
}

impl CycleLoop for CycleTaskDb {
    async fn cycle(&self) -> hbb_common::ResultType<()> {
        loop {
            hbb_common::tokio::time::sleep(std::time::Duration::from_millis(hbb_common::config::SYNC_MEM_DB_CHECK_CYCLE)).await;
            let mut inner = self.inner.write().await;
            if inner.db_step > hbb_common::config::SYNC_MEM_DB_STEP ||
            (inner.expire.elapsed().as_millis() > hbb_common::config::SYNC_MEM_DB_EXPIRE && inner.sync_to_db.task_len() > 0) || 
            inner.sync_to_db.task_len() > hbb_common::config::SYNC_MEM_DB_MAX_COUNT {
                hbb_common::log::info!("CycleTaskDb::cycle::execute() - SyncDb::order_tasks: len={:?}", inner.sync_to_db.task_len());
                inner.sync_to_db.run().await?;
                inner.db_step = 0;
                inner.expire = std::time::Instant::now();
            }
            drop(inner);
        }
    }
}

pub(crate) struct SyncDb<F> {
    order_tasks: std::collections::LinkedList<F>,
}
impl<F> SyncDb<F> 
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    pub(crate) fn new() -> Self {
        Self {
            order_tasks: std::collections::LinkedList::new(),
        }
    }
    pub(crate) fn push_task(&mut self, task: F) {
        self.order_tasks.push_back(task );
    }
    pub(crate) fn task_len(&self) -> usize {
        self.order_tasks.len()
    }
    pub(crate) async fn run(&mut self) -> ResultType<()> {
        while let Some(t) = self.order_tasks.pop_front() {
            t.await;
        }
        Ok(())
    }
}