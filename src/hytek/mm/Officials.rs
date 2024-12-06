// 
// 
// CREATE TABLE [Officials]
// (
// [Official_no] Long Integer,
// [Last_name] Text (40),
// [First_name] Text (40),
// [Initial] Text (2),
// [Pref_name] Text (40),
// [Home_email] Text (100)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Officials {
    official_no : Option<u64>,
    last_name : Option<String>,
    first_name : Option<String>,
    initial : Option<String>,
    pref_name : Option<String>,
    home_email : Option<String>
}
impl Officials {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Officials>, Box<dyn Error>> {
       let mut vec: Vec<Officials> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Officials = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Officials>, Error> {
        let mut vec: Vec<Officials> = Vec::new();
        match conn.execute("SELECT * FROM officials", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let official_no = read_u64(_batch.at(0, row_index));
                        let last_name = read_string(_batch.at(1, row_index));
                        let first_name = read_string(_batch.at(2, row_index));
                        let initial = read_string(_batch.at(3, row_index));
                        let pref_name = read_string(_batch.at(4, row_index));
                        let home_email = read_string(_batch.at(5, row_index));
                        let obj = Officials {
                            official_no,
                            last_name,
                            first_name,
                            initial,
                            pref_name,
                            home_email
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
