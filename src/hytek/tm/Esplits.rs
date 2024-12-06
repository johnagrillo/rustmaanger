// 
// 
// CREATE TABLE [ESPLITS]
// (
// [SplitID] Long Integer,
// [SplitIndex] Byte,
// [Split] Long Integer,
// [RelayLeg] Text (2)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Esplits {
    split_i_d : Option<u64>,
    split_index : Option<u8>,
    split : Option<u64>,
    relay_leg : Option<String>
}
impl Esplits {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Esplits>, Box<dyn Error>> {
       let mut vec: Vec<Esplits> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Esplits = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Esplits>, Box<dyn Error>> {
        let mut vec: Vec<Esplits> = Vec::new();
        match conn.execute("SELECT * FROM ESPLITS", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let split_i_d = read_u64(_batch.at(0, row_index))?;
                        let split_index = read_u8(_batch.at(1, row_index))?;
                        let split = read_u64(_batch.at(2, row_index))?;
                        let relay_leg = read_string(_batch.at(3, row_index))?;
                        let obj = Esplits {
                            split_i_d,
                            split_index,
                            split,
                            relay_leg
                        };
                        vec.push(obj);
                    };
                }
            }
	    None => todo!()
        }
        Ok(vec)
    }
}
