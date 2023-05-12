use redb::{Database, Error, ReadableTable, TableDefinition};

pub(super) type InscriptionIdValue = [u8; 36];
const SAT_TO_INSCRIPTION_ID: TableDefinition<u64, &InscriptionIdValue> = TableDefinition::new("SAT_TO_INSCRIPTION_ID");

pub struct SatToInscriptionId {}

impl SatToInscriptionId {

    pub fn scan(db: &Database, count: u64) -> Result<(), Error> {
        println!("SatToInscriptionId scan:");
        let begin = db.begin_read()?;
        let table = begin.open_table(SAT_TO_INSCRIPTION_ID)?;
        let range = table.range(0..)?;
        println!("sat,genesis_tx_hash,genesis_index");
        for next in range.rev().take(count.try_into().unwrap()) {
            let mut inscription_id:InscriptionIdValue = *next.1.value();
            let (txid_bytes, index_bytes) = inscription_id.split_at_mut(32); 
            let tx = txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let index = i32::from_le_bytes(index_bytes.try_into().unwrap());

            println!("{},{},{}", next.0.value(), tx, index);
        }
        println!("");
        println!("");
        println!("");
        println!("");
        Ok(())
    }

}
