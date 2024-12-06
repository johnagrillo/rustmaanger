// 
// 
// CREATE TABLE [CODE]
// (
// [ABBR] Text (6),
// [DESC] Text (60),
// [CODE] Long Integer,
// [TYPE] Byte
// );
#[derive(Serialize,Deserialize,Debug)]
struct Code {
    ABBR : Option<String>,
    DESC : Option<String>,
    CODE : u64,
    TYPE : Option<u8>
}
impl Code {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Code>, Box<dyn Error>> {
       let mut vec: Vec<Code> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Code = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Code>, Box<dyn Error>> {
        let mut vec: Vec<Code> = Vec::new();
        match conn.execute("SELECT * FROM CODE", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let ABBR = read_string(_batch.at(0, row_index))?;
                        let DESC = read_string(_batch.at(1, row_index))?;
                        let CODE = read_u64(_batch.at(2, row_index))?;
                        let TYPE = read_u8(_batch.at(3, row_index))?;
                        let obj = Code {
                            ABBR,
                            DESC,
                            CODE,
                            TYPE
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
