use anyhow::Result;
use didactic_funicular::btree::{BTree, SearchMode};
use didactic_funicular::buffer::{BufferPool, BufferPoolManager};
use didactic_funicular::disk::{DiskManager, PageId};
use didactic_funicular::tuple;


fn main() -> Result<()> {
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree.search(&mut bufmgr, SearchMode::Start)?;

    while let Some((key, value)) = iter.next(&mut bufmgr)? {
        let mut record = vec![];
        tuple::decode(&key, &mut record);
        tuple::decode(&value, &mut record);
        println!("{:?}", tuple::Pretty(&record));
    }
    Ok(())
}
