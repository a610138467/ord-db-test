use redb::{Database, Error, ReadableTable, TableDefinition};

pub(super) type SatPointValue = [u8; 44];
const SAT_TO_SATPOINT: TableDefinition<u64, &SatPointValue> = TableDefinition::new("SAT_TO_SATPOINT");

pub struct SatToSatpoint {}

impl SatToSatpoint {

    pub fn scan(db: &Database, count: u64) -> Result<(), Error> {
        println!("SatToSatpoint scan:");
        let begin = db.begin_read()?;
        let table = begin.open_table(SAT_TO_SATPOINT)?;
        let range = table.range(0..)?;
        println!("sat,tx_hash,index,offset");
        for next in range.rev().take(count.try_into().unwrap()) {
            let mut satpoint: SatPointValue = *next.1.value();
            let (satpoint_txid_bytes, satpoint_index_and_offset_bytes) = satpoint.split_at_mut(32);
            let (satpoint_index_bytes, satpoint_offset_bytes) = satpoint_index_and_offset_bytes.split_at_mut(4);
            let satpoint_tx = satpoint_txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let satpoint_index = u32::from_le_bytes(satpoint_index_bytes.try_into().unwrap());
            let satpoint_offset = u64::from_le_bytes(satpoint_offset_bytes.try_into().unwrap());

            println!("{},{},{},{}", next.0.value(), satpoint_tx, satpoint_index, satpoint_offset);
        }
        println!("");
        println!("");
        println!("");
        println!("");
        Ok(())
    }

}
