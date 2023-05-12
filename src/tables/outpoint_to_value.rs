use redb::{Database, Error, ReadableTable, TableDefinition};

pub(super) type OutPointValue = [u8; 36];
const OUTPOINT_TO_VALUE: TableDefinition<&OutPointValue, u64> = TableDefinition::new("OUTPOINT_TO_VALUE");

pub struct OutpointToValue {}

impl OutpointToValue {

    pub fn scan(db: &Database, count: u64) -> Result<(), Error> {
        println!("OutpointToValue scan:");
        let begin = db.begin_read()?;
        let table = begin.open_table(OUTPOINT_TO_VALUE)?;
        let range = table.range::<&[u8; 36]>(&[0; 36]..)?;
        println!("tx_hash,index,value");
        for next in range.rev().take(count.try_into().unwrap()) {
            let mut outpoint:OutPointValue = *next.0.value();
            let (txid_bytes, index_bytes) = outpoint.split_at_mut(32);
            let txid = txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let index = u32::from_le_bytes(index_bytes.try_into().unwrap());

            println!("{}{},{}", txid, index, next.1.value());
        }
        println!("");
        println!("");
        println!("");
        println!("");
        Ok(())
    }

}
