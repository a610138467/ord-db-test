use redb::{Database, Error, ReadableTable, TableDefinition};

type Result<T> = std::result::Result<T, Error>;
type InscriptionIdValue = [u8; 36];
type SatPointValue = [u8; 44];
const INSCRIPTION_ID_TO_SATPOINT: TableDefinition<&InscriptionIdValue, &SatPointValue> = TableDefinition::new("INSCRIPTION_ID_TO_SATPOINT");

pub struct InscriptionIdToSatpoint {}

impl InscriptionIdToSatpoint {

    pub fn scan(db: &Database, count: u64) -> Result<()> {
        println!("InscriptionIdToSatpoint sccan:");
        let begin = db.begin_read()?;
        let table = begin.open_table(INSCRIPTION_ID_TO_SATPOINT)?;
        let range = table.range::<&[u8; 36]>(&[0; 36]..)?;
        println!("genesis_tx,genesis_index,current_tx,current_index,offset");
        for next in range.rev().take(count.try_into().unwrap()) {
            let mut inscription_id:InscriptionIdValue = *next.0.value();
            let (txid_bytes, index_bytes) = inscription_id.split_at_mut(32); 
            let tx = txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let index = i32::from_le_bytes(index_bytes.try_into().unwrap());

            let mut satpoint: SatPointValue = *next.1.value();
            let (satpoint_txid_bytes, satpoint_index_and_offset_bytes) = satpoint.split_at_mut(32);
            let (satpoint_index_bytes, satpoint_offset_bytes) = satpoint_index_and_offset_bytes.split_at_mut(4);
            let satpoint_tx = satpoint_txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let satpoint_index = u32::from_le_bytes(satpoint_index_bytes.try_into().unwrap());
            let satpoint_offset = u64::from_le_bytes(satpoint_offset_bytes.try_into().unwrap());

            //let satpoint = next.1.value().as_ref().iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            println!("{},{},{},{},{}", tx, index, satpoint_tx, satpoint_index, satpoint_offset);
        }
        println!("");
        println!("");
        println!("");
        println!("");
        Ok(())
    }

    pub fn get(db: &Database, inscription_id: &InscriptionIdValue) -> Result<SatPointValue> {
        Ok(
            db.begin_read()?
                .open_table(INSCRIPTION_ID_TO_SATPOINT)?
                .get(inscription_id)?
                .map(|satpoint| *satpoint.value())
                .unwrap()
        )
    }

}
