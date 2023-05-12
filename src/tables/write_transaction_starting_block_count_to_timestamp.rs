use redb::{Database, Error, ReadableTable, TableDefinition};

const WRITE_TRANSACTION_STARTING_BLOCK_COUNT_TO_TIMESTAMP: TableDefinition<u64, u128> = TableDefinition::new("WRITE_TRANSACTION_STARTING_BLOCK_COUNT_TO_TIMESTAMP");

pub struct WriteTransactionStartingBlockCountToTimestamp {}

impl WriteTransactionStartingBlockCountToTimestamp {

    pub fn scan(db: &Database, count: u64) -> Result<(), Error> {
        println!("WriteTransactionStartingBlockCountToTimestamp: ");
        let begin = db.begin_read()?;
        let table = begin.open_table(WRITE_TRANSACTION_STARTING_BLOCK_COUNT_TO_TIMESTAMP)?;
        let range = table.range(0..)?;
        println!("block_number,timestamp");
        for next in range.rev().take(count.try_into().unwrap()) {
            println!("{},{}", next.0.value(), next.1.value());
        }
        println!("");
        println!("");
        println!("");
        println!("");
        Ok(())
    }

}
