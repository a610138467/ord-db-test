use redb::{Database, Error, ReadableTable, TableDefinition};

type Result<T> = std::result::Result<T, Error>;
type InscriptionIdValue = [u8; 36];

const INSCRIPTION_NUMBER_TO_INSCRIPTION_ID: TableDefinition<u64, &InscriptionIdValue> = TableDefinition::new("INSCRIPTION_NUMBER_TO_INSCRIPTION_ID");

pub struct InscriptionNumberToInscriptionId {}

impl InscriptionNumberToInscriptionId {

    pub fn scan(db: &Database, count: u64) -> Result<()> {
        println!("InscriptionNumberToInscriptionId scan:");
        let begin = db.begin_read()?;
        let table = begin.open_table(INSCRIPTION_NUMBER_TO_INSCRIPTION_ID)?;
        let range = table.range(0..)?;
        println!("token_id, tx_hash, index");
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

    pub fn all(db: &Database) -> Result<Vec<(u64, InscriptionIdValue)>> {
        Ok(
            db.begin_read()?
                .open_table(INSCRIPTION_NUMBER_TO_INSCRIPTION_ID)?
                .iter()?
                .take(u64::MAX.try_into().unwrap())
                .map(|(token_id, inscription_id)| (token_id.value(), inscription_id.value().clone()))
                .collect(),
        )
    }

}
