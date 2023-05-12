use redb::{Database, Error, ReadableTable, TableDefinition};

pub(super) type SatPointValue = [u8; 44];
pub(super) type InscriptionIdValue = [u8; 36];
const SATPOINT_TO_INSCRIPTION_ID: TableDefinition<&SatPointValue, &InscriptionIdValue> = TableDefinition::new("SATPOINT_TO_INSCRIPTION_ID");

pub struct SatpointToInscriptionId {}

impl SatpointToInscriptionId {

    pub fn scan(db: &Database, count: u64) -> Result<(), Error> {
        println!("SatpointToInscriptionId scan:");
        let begin = db.begin_read()?;
        let table = begin.open_table(SATPOINT_TO_INSCRIPTION_ID)?;
        let range = table.range::<&[u8; 44]>(&[0; 44]..)?;
        println!("current_tx,current_index,offset,genesis_tx,genesis_index");
        for next in range.rev().take(count.try_into().unwrap()) {
            let mut satpoint: SatPointValue = *next.0.value();
            let (satpoint_txid_bytes, satpoint_index_and_offset_bytes) = satpoint.split_at_mut(32);
            let (satpoint_index_bytes, satpoint_offset_bytes) = satpoint_index_and_offset_bytes.split_at_mut(4);
            let satpoint_tx = satpoint_txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let satpoint_index = u32::from_le_bytes(satpoint_index_bytes.try_into().unwrap());
            let satpoint_offset = u64::from_le_bytes(satpoint_offset_bytes.try_into().unwrap());

            let mut inscription_id:InscriptionIdValue = *next.1.value();
            let (txid_bytes, index_bytes) = inscription_id.split_at_mut(32); 
            let tx = txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let index = i32::from_le_bytes(index_bytes.try_into().unwrap());

            //let satpoint = next.1.value().as_ref().iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            println!("{},{},{},{},{}", satpoint_tx, satpoint_index, satpoint_offset, tx, index);

        }
        println!("");
        println!("");
        println!("");
        println!("");
        Ok(())
    }

}
