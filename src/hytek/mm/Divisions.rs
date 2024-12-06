// 
// 
// CREATE TABLE [Divisions]
// (
// [Div_no] Long Integer,
// [Div_name] Text (40),
// [old_date] DateTime,
// [young_date] DateTime,
// [div_abbr] Text (20)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Divisions {
    div_no : Option<u64>,
    div_name : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    old_date : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    young_date : Option<NaiveDateTime>,
    div_abbr : Option<String>
}
impl Divisions {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Divisions>, Box<dyn Error>> {
       let mut vec: Vec<Divisions> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Divisions = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Divisions>, Error> {
        let mut vec: Vec<Divisions> = Vec::new();
        match conn.execute("SELECT * FROM divisions", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let div_no = read_u64(_batch.at(0, row_index));
                        let div_name = read_string(_batch.at(1, row_index));
                        let old_date = read_datetime(_batch.at(2, row_index));
                        let young_date = read_datetime(_batch.at(3, row_index));
                        let div_abbr = read_string(_batch.at(4, row_index));
                        let obj = Divisions {
                            div_no,
                            div_name,
                            old_date,
                            young_date,
                            div_abbr
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
