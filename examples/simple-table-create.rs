use anyhow::Result;
use didactic_funicular::buffer::{BufferPool, BufferPoolManager};
use didactic_funicular::disk::{DiskManager, PageId};
use didactic_funicular::table::SimpleTable;


fn main() -> Result<()> {
    // 1
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    // 2
    let mut table = SimpleTable {
        meta_page_id: PageId(0),
        num_key_elems: 1,
    };

    // 3
    table.create(&mut bufmgr)?;
    dbg!(&table);

    // 4
    table.insert(&mut bufmgr, &[b"z", b"Alice", b"Smith"])?;
    table.insert(&mut bufmgr, &[b"x", b"Bob", b"Johnson"])?;
    table.insert(&mut bufmgr, &[b"y", b"Charlie", b"Williams"])?;
    table.insert(&mut bufmgr, &[b"w", b"Dave", b"Miller"])?;
    table.insert(&mut bufmgr, &[b"v", b"Eve", b"Brown"])?;

    // 5
    bufmgr.flush()?;
    Ok(())
}
