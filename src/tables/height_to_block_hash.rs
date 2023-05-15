use redb::{Database, Error, ReadableTable, TableDefinition};

type Result<T> = std::result::Result<T, Error>;
const HEIGHT_TO_BLOCK_HASH: TableDefinition<u64, &[u8;32]> = TableDefinition::new("HEIGHT_TO_BLOCK_HASH");

pub struct HeightToBlockHash {}

impl HeightToBlockHash {

    pub fn scan(db: &Database, count: u64) -> Result<()> {
        println!("HeightToBlockHash scan:");
        let begin = db.begin_read()?;
        let table = begin.open_table(HEIGHT_TO_BLOCK_HASH)?;
        let range = table.range(0..)?;
        println!("block_number,block_hash");
        for next in range.rev().take(count.try_into().unwrap()) {
            let block_hash = next.1.value().as_ref().iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            println!("{},{}", next.0.value(), block_hash);
        }
        println!("");
        println!("");
        println!("");
        println!("");
        Ok(())
    }

    pub fn all(db: &Database) -> Result<Vec<(u64, [u8;32])>> {
        Ok(
            db.begin_read()?
                .open_table(HEIGHT_TO_BLOCK_HASH)?
                .iter()?
                .take(u64::MAX.try_into().unwrap())
                .map(|(block_number, block_hash)| (block_number.value(), block_hash.value().clone()))
                .collect(),
        )
    }

    pub fn range(db: &Database, from: u64, size: usize) -> Result<Vec<(u64, [u8;32])>> {
        Ok(
            db.begin_read()?
                .open_table(HEIGHT_TO_BLOCK_HASH)?
                .range(from..)?
                .take(size.try_into().unwrap())
                .map(|(block_number, block_hash)| (block_number.value(), block_hash.value().clone()))
                .collect(),
        )
    }

}
