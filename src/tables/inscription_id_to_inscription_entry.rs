use redb::{Database, Error, ReadableTable, TableDefinition};

type Result<T> = std::result::Result<T, Error>;
type InscriptionIdValue = [u8; 36];
type InscriptionEntryValue = (u64, u64, u64, u64, u32);

const INSCRIPTION_ID_TO_INSCRIPTION_ENTRY: TableDefinition<&InscriptionIdValue, InscriptionEntryValue> = TableDefinition::new("INSCRIPTION_ID_TO_INSCRIPTION_ENTRY");

pub struct InscriptionIdToInscriptionEntry {}

impl InscriptionIdToInscriptionEntry {

    pub fn scan(db: &Database, count: u64) -> Result<()> {
        println!("InscriptionIdToInscriptionEntry scan:");
        let begin = db.begin_read()?;
        let table = begin.open_table(INSCRIPTION_ID_TO_INSCRIPTION_ENTRY)?;
        let range = table.range::<&[u8; 36]>(&[0; 36]..)?;
        println!("tx_hash,vout_index,fee,height,number,sat,timestamp");
        for next in range.rev().take(count.try_into().unwrap()) {
            let mut inscription_id:InscriptionIdValue = *next.0.value();
            let (txid_bytes, index_bytes) = inscription_id.split_at_mut(32); 
            let tx = txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let index = i32::from_le_bytes(index_bytes.try_into().unwrap());
            println!("{},{},{},{},{},{},{}", tx, index, next.1.value().0, next.1.value().1, next.1.value().2, next.1.value().3, next.1.value().4);
        }
        println!("");
        println!("");
        println!("");
        println!("");
        Ok(())
    }

    pub fn get(db: &Database, inscription_id: &InscriptionIdValue) -> Result<InscriptionEntryValue> {
        Ok(
            db.begin_read()?
                .open_table(INSCRIPTION_ID_TO_INSCRIPTION_ENTRY)?
                .get(inscription_id)?
                .map(|entry| entry.value())
                .unwrap()
        )
    }

}
