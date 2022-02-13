use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use crate::disk::{DiskManager, PAGE_SIZE, PageId};

pub type Page = [u8; PAGE_SIZE];

pub struct BufferId(pub u64);

pub struct Buffer {
    pub page_id: PageId,
    pub page: RefCell<Page>,
    pub is_dirty: Cell<bool>,
}

pub struct Frame {
    usage_count: u64,
    buffer: Rc<Buffer>,
}

pub struct BufferPool {
    buffers: Vec<Frame>,
    next_victim_id: BufferId,
}

impl BufferPool {
    fn evict(&mut self) -> Option<BufferId> {
        let pool_size = self.size();
        let mut consecutive_pinned = 0

    }
}

pub struct BufferPoolManager{
    disk: DiskManager,
    pool: BufferPool,
    page_table: HashMap<PageId,BufferId>
}
