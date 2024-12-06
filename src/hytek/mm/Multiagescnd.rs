// 
// 
// CREATE TABLE [MultiageScnd]
// (
// [event_ptr] Long Integer,
// [low_age] Integer,
// [high_age] Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Multiagescnd {
    event_ptr : Option<u64>,
    low_age : Option<u16>,
    high_age : Option<u16>
}
impl Multiagescnd {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Multiagescnd>, Box<dyn Error>> {
       let mut vec: Vec<Multiagescnd> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Multiagescnd = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Multiagescnd>, Error> {
        let mut vec: Vec<Multiagescnd> = Vec::new();
        match conn.execute("SELECT * FROM multiage scnd", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let event_ptr = read_u64(_batch.at(0, row_index));
                        let low_age = read_u16(_batch.at(1, row_index));
                        let high_age = read_u16(_batch.at(2, row_index));
                        let obj = Multiagescnd {
                            event_ptr,
                            low_age,
                            high_age
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
