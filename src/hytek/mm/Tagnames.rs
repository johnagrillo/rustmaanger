// 
// 
// CREATE TABLE [TagNames]
// (
// [tag_ptr] Long Integer,
// [tag_name] Text (8),
// [for_scoring] Boolean NOT NULL,
// [for_entryqual] Boolean NOT NULL,
// [for_timestd] Boolean NOT NULL,
// [tag_desc] Text (40)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Tagnames {
    tag_ptr : Option<u64>,
    tag_name : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    for_scoring : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    for_entryqual : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    for_timestd : bool,
    tag_desc : Option<String>
}
impl Tagnames {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Tagnames>, Box<dyn Error>> {
       let mut vec: Vec<Tagnames> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Tagnames = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Tagnames>, Error> {
        let mut vec: Vec<Tagnames> = Vec::new();
        match conn.execute("SELECT * FROM tag names", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let tag_ptr = read_u64(_batch.at(0, row_index));
                        let tag_name = read_string(_batch.at(1, row_index));
                        let for_scoring = read_bool(_batch.at(2, row_index));
                        let for_entryqual = read_bool(_batch.at(3, row_index));
                        let for_timestd = read_bool(_batch.at(4, row_index));
                        let tag_desc = read_string(_batch.at(5, row_index));
                        let obj = Tagnames {
                            tag_ptr,
                            tag_name,
                            for_scoring,
                            for_entryqual,
                            for_timestd,
                            tag_desc
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
