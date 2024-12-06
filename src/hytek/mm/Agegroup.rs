// 
// 
// CREATE TABLE [Agegroup]
// (
// [Low_age] Integer,
// [High_age] Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Agegroup {
    low_age : Option<u16>,
    high_age : Option<u16>
}
impl Agegroup {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Agegroup>, Box<dyn Error>> {
       let mut vec: Vec<Agegroup> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Agegroup = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Agegroup>, Error> {
        let mut vec: Vec<Agegroup> = Vec::new();
        match conn.execute("SELECT * FROM agegroup", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let low_age = read_u16(_batch.at(0, row_index));
                        let high_age = read_u16(_batch.at(1, row_index));
                        let obj = Agegroup {
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
