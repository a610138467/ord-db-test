use redb::{Database, Error, ReadableTable, TableDefinition};

type Result<T> = std::result::Result<T, Error>;
type OutPointValue = [u8; 36];

const OUTPOINT_TO_SAT_RANGES: TableDefinition<&OutPointValue, &[u8]> = TableDefinition::new("OUTPOINT_TO_SAT_RANGES");

pub struct OutpointToSatRanges {}

impl OutpointToSatRanges {

    pub fn scan(db: &Database, count: u64) -> Result<()> {
        println!("OutpointToSatRanges scan: ");
        let begin = db.begin_read()?;
        let table = begin.open_table(OUTPOINT_TO_SAT_RANGES)?;
        let range = table.range::<&[u8; 36]>(&[0; 36]..)?;
        println!("tx_hash,index,sat_range,data_length");
        for next in range.rev().take(count.try_into().unwrap()) {
            let mut outpoint:OutPointValue = *next.0.value();
            let (txid_bytes, index_bytes) = outpoint.split_at_mut(32);
            let txid = txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let index = u32::from_le_bytes(index_bytes.try_into().unwrap());

            let sat_ranges = next.1.value();
            print!("{},{},[", txid, index);
            let mut first: bool = true;
            for chunk in sat_ranges.chunks_exact(11) {
                let [b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10]: [u8;11] = chunk.try_into().unwrap(); 

                let raw_base = u64::from_le_bytes([b0, b1, b2, b3, b4, b5, b6, 0]);
                let start = raw_base & ((1 << 51) - 1);

                let raw_delta = u64::from_le_bytes([b6, b7, b8, b9, b10, 0, 0, 0]);
                let delta = raw_delta >> 3;
                let end = start + delta;
                if !first {
                    print!(",");
                } else {
                    first = false;
                }
                print!("({},{})", start, end);
            }
            println!("],{}", sat_ranges.len());
        }
        println!("");
        println!("");
        println!("");
        println!("");
        Ok(())
    }

    pub fn all(db: &Database) -> Result<Vec<(OutPointValue, Vec<u8>)>> {
        Ok(
            db.begin_read()?
                .open_table(OUTPOINT_TO_SAT_RANGES)?
                .iter()?
                .take(u64::MAX.try_into().unwrap())
                .map(|(outpoint, sat_ranges)| (*outpoint.value(), Vec::from(sat_ranges.value())))
                .collect(),
        )
    }

}
