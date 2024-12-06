// 
// 
// CREATE TABLE [DELETEENTRY]
// (
// [Meet] Long Integer,
// [Athlete] Long Integer,
// [MtEvent] Long Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Deleteentry {
    meet : Option<u64>,
    athlete : Option<u64>,
    mt_event : Option<u64>
}
impl Deleteentry {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Deleteentry>, Box<dyn Error>> {
       let mut vec: Vec<Deleteentry> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Deleteentry = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Deleteentry>, Box<dyn Error>> {
        let mut vec: Vec<Deleteentry> = Vec::new();
        match conn.execute("SELECT * FROM DELETEENTRY", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let meet = read_u64(_batch.at(0, row_index))?;
                        let athlete = read_u64(_batch.at(1, row_index))?;
                        let mt_event = read_u64(_batch.at(2, row_index))?;
                        let obj = Deleteentry {
                            meet,
                            athlete,
                            mt_event
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
