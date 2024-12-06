// 
// 
// CREATE TABLE [SPLITS]
// (
// [SplitID] Long Integer,
// [SplitIndex] Byte,
// [Split] Long Integer,
// [StrokeRate] Single
// );
#[derive(Serialize,Deserialize,Debug)]
struct Splits {
    split_i_d : Option<u64>,
    split_index : Option<u8>,
    split : Option<u64>,
    stroke_rate : Option<f32>
}
impl Splits {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Splits>, Box<dyn Error>> {
       let mut vec: Vec<Splits> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Splits = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Splits>, Box<dyn Error>> {
        let mut vec: Vec<Splits> = Vec::new();
        match conn.execute("SELECT * FROM SPLITS", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let split_i_d = read_u64(_batch.at(0, row_index))?;
                        let split_index = read_u8(_batch.at(1, row_index))?;
                        let split = read_u64(_batch.at(2, row_index))?;
                        let stroke_rate = read_f32(_batch.at(3, row_index))?;
                        let obj = Splits {
                            split_i_d,
                            split_index,
                            split,
                            stroke_rate
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
