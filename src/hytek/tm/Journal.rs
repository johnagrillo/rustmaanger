// 
// 
// CREATE TABLE [JOURNAL]
// (
// [JDate] DateTime,
// [Subject] Text (40),
// [Comment] Text (510)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Journal {
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    j_date : Option<NaiveDateTime>,
    subject : Option<String>,
    comment : Option<String>
}
impl Journal {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Journal>, Box<dyn Error>> {
       let mut vec: Vec<Journal> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Journal = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Journal>, Box<dyn Error>> {
        let mut vec: Vec<Journal> = Vec::new();
        match conn.execute("SELECT * FROM JOURNAL", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let j_date = read_datetime(_batch.at(0, row_index))?;
                        let subject = read_string(_batch.at(1, row_index))?;
                        let comment = read_string(_batch.at(2, row_index))?;
                        let obj = Journal {
                            j_date,
                            subject,
                            comment
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
