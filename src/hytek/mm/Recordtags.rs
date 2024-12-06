// 
// 
// CREATE TABLE [RecordTags]
// (
// [tag_ptr] Long Integer,
// [tag_order] Integer,
// [tag_name] Text (24),
// [tag_flag] Text (2),
// [team_no] Long Integer,
// [allow_exh] Boolean NOT NULL,
// [allow_foreigner] Boolean NOT NULL,
// [tag_lsc] Text (6)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Recordtags {
    tag_ptr : Option<u64>,
    tag_order : Option<u16>,
    tag_name : Option<String>,
    tag_flag : Option<String>,
    team_no : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    allow_exh : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    allow_foreigner : bool,
    tag_lsc : Option<String>
}
impl Recordtags {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Recordtags>, Box<dyn Error>> {
       let mut vec: Vec<Recordtags> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Recordtags = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Recordtags>, Error> {
        let mut vec: Vec<Recordtags> = Vec::new();
        match conn.execute("SELECT * FROM record tags", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let tag_ptr = read_u64(_batch.at(0, row_index));
                        let tag_order = read_u16(_batch.at(1, row_index));
                        let tag_name = read_string(_batch.at(2, row_index));
                        let tag_flag = read_string(_batch.at(3, row_index));
                        let team_no = read_u64(_batch.at(4, row_index));
                        let allow_exh = read_bool(_batch.at(5, row_index));
                        let allow_foreigner = read_bool(_batch.at(6, row_index));
                        let tag_lsc = read_string(_batch.at(7, row_index));
                        let obj = Recordtags {
                            tag_ptr,
                            tag_order,
                            tag_name,
                            tag_flag,
                            team_no,
                            allow_exh,
                            allow_foreigner,
                            tag_lsc
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
