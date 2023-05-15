use redb::{Database, Error};

mod tables;

#[allow(unused)]
fn main() -> Result<(), Error> {
    //let db = Database::create("data/index.redb")?;
    let db = Database::create("data/regtest/index.redb")?;
    //导出当前所有的NFT信息
    //token_id, genesis_tx_hash, genesis_vout_index, genesis_fee, genesis_height, sat, genesis_timestamp, currrent_tx, current_index, offset
    let mut start: u64 = 0;
    let size: usize = 1;
    loop {
        //首先遍历得到所有的tokenId
        let mut nfts = tables::InscriptionNumberToInscriptionId::range(&db, start, size)?;
        start = start + size as u64;
        for (token_id, inscription_id) in nfts.iter_mut() {
            let (txid_bytes, index_bytes) = inscription_id.split_at_mut(32); 
            let genesis_tx = txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let genesis_index = i32::from_le_bytes(index_bytes.try_into().unwrap());
            //每一个inscription_id获取一个inscription_entry信息
            let mut entry = tables::InscriptionIdToInscriptionEntry::get(&db, &inscription_id)?;
            //获取他当前对应的satpoint
            let mut satpoint = tables::InscriptionIdToSatpoint::get(&db, &inscription_id)?;
            let (satpoint_txid_bytes, satpoint_index_and_offset_bytes) = satpoint.split_at_mut(32);
            let (satpoint_index_bytes, satpoint_offset_bytes) = satpoint_index_and_offset_bytes.split_at_mut(4);
            let current_tx = satpoint_txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            let current_index = u32::from_le_bytes(satpoint_index_bytes.try_into().unwrap());
            let offset = u64::from_le_bytes(satpoint_offset_bytes.try_into().unwrap());

            println!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}", token_id, genesis_tx, genesis_index, entry.0, entry.1, entry.3, entry.4, current_tx, current_index, offset);
        }
        if (nfts.len() < size) {
            break;
        }
    }
    /*
    //导出所有outpoint以及对应的sat range
    let mut outpoint_to_range = tables::OutpointToSatRanges::all(&db)?;
    for (outpoint, satrangges) in outpoint_to_range.iter_mut() {
        let (txid_bytes, index_bytes) = outpoint.split_at_mut(32);
        let txid = txid_bytes.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
        let index = u32::from_le_bytes(index_bytes.try_into().unwrap());
        println!("{}, {}", txid, index);
        
    }
    */
    /*
    //导出所有的blocknumber->hash
    let mut start: u64 = 0;
    let size: usize = 1;
    loop {
        let mut block_number_to_hash = tables::HeightToBlockHash::range(&db, start, size)?;
        start = start + size as u64;
        for (block_height, block_hash) in block_number_to_hash.iter() {
            let block_str = block_hash.iter().rev().map(|b| format!("{:02x}", b)).collect::<String>();
            println!("{},{}", block_height, block_str);
        }
        if (block_number_to_hash.len() < size) {
            break;
        }
    }
    */

    /*
    tables::HeightToBlockHash::scan(&db, /*u64::MAX*/ 10)?;
    tables::InscriptionIdToInscriptionEntry::scan(&db, /*u64::MAX*/ 10)?;
    tables::InscriptionIdToSatpoint::scan(&db, /*u64::MAX*/ 10)?;
    tables::InscriptionNumberToInscriptionId::scan(&db, /*u64::MAX*/ 10)?;
    tables::OutpointToSatRanges::scan(&db, u64::MAX)?;
    tables::OutpointToValue::scan(&db, u64::MAX)?;
    tables::SatpointToInscriptionId::scan(&db, /*u64::MAX*/ 10)?;
    tables::SatToInscriptionId::scan(&db, /*u64::MAX*/ 10)?;
    tables::SatToSatpoint::scan(&db, /*u64::MAX*/ 0)?;
    tables::StatisticToCount::scan(&db, /*u64::MAX*/ 0)?;
    tables::WriteTransactionStartingBlockCountToTimestamp::scan(&db, /*u64::MAX*/ 0)?;
    */
    Ok(())
}
