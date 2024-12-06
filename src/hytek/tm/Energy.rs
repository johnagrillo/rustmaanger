// 
// 
// CREATE TABLE [Energy]
// (
// [energy_no] Integer,
// [energy_abbr] Text (6),
// [energy_name] Text (40),
// [stress_index] Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Energy {
    energy_no : Option<u16>,
    energy_abbr : Option<String>,
    energy_name : Option<String>,
    stress_index : Option<u16>
}
impl Energy {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Energy>, Box<dyn Error>> {
       let mut vec: Vec<Energy> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Energy = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Energy>, Box<dyn Error>> {
        let mut vec: Vec<Energy> = Vec::new();
        match conn.execute("SELECT * FROM energy", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let energy_no = read_u16(_batch.at(0, row_index))?;
                        let energy_abbr = read_string(_batch.at(1, row_index))?;
                        let energy_name = read_string(_batch.at(2, row_index))?;
                        let stress_index = read_u16(_batch.at(3, row_index))?;
                        let obj = Energy {
                            energy_no,
                            energy_abbr,
                            energy_name,
                            stress_index
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
