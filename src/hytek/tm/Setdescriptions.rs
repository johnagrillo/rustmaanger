// 
// 
// CREATE TABLE [SetDescriptions]
// (
// [set_name] Text (60)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Setdescriptions {
    set_name : Option<String>
}
impl Setdescriptions {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Setdescriptions>, Box<dyn Error>> {
       let mut vec: Vec<Setdescriptions> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Setdescriptions = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Setdescriptions>, Box<dyn Error>> {
        let mut vec: Vec<Setdescriptions> = Vec::new();
        match conn.execute("SELECT * FROM set descriptions", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let set_name = read_string(_batch.at(0, row_index))?;
                        let obj = Setdescriptions {
                            set_name
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
