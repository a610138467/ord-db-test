use redb::{Database, Error, ReadableTable, TableDefinition};

const STATISTIC_TO_COUNT: TableDefinition<u64, u64> = TableDefinition::new("STATISTIC_TO_COUNT");

pub struct StatisticToCount {}

impl StatisticToCount {

    pub fn scan(db: &Database, count: u64) -> Result<(), Error> {
        println!("StatisticToCount scan:");
        let begin = db.begin_read()?;
        let table = begin.open_table(STATISTIC_TO_COUNT)?;
        let range = table.range(0..)?;
        println!("num,count");
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
