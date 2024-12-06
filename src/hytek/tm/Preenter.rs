// 
// 
// CREATE TABLE [PREENTER]
// (
// [THEATHLETE] Long Integer,
// [THEMEET] Long Integer,
// [PREENTERED] Boolean NOT NULL,
// [SESSION1] Text (6),
// [SESSION2] Text (6),
// [SESSION3] Text (6),
// [SESSION4] Text (6),
// [SESSION5] Text (6),
// [SESSION6] Text (6),
// [SESSION7] Text (6),
// [SESSION8] Text (6),
// [SESSION9] Text (6),
// [SESSION10] Text (6),
// [NOTATTENDING] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Preenter {
    THEATHLETE : Option<u64>,
    THEMEET : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    PREENTERED : bool,
    SESSION1 : Option<String>,
    SESSION2 : Option<String>,
    SESSION3 : Option<String>,
    SESSION4 : Option<String>,
    SESSION5 : Option<String>,
    SESSION6 : Option<String>,
    SESSION7 : Option<String>,
    SESSION8 : Option<String>,
    SESSION9 : Option<String>,
    SESSION10 : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    NOTATTENDING : bool
}
impl Preenter {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Preenter>, Box<dyn Error>> {
       let mut vec: Vec<Preenter> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Preenter = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Preenter>, Box<dyn Error>> {
        let mut vec: Vec<Preenter> = Vec::new();
        match conn.execute("SELECT * FROM PREENTER", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let THEATHLETE = read_u64(_batch.at(0, row_index))?;
                        let THEMEET = read_u64(_batch.at(1, row_index))?;
                        let PREENTERED = read_bool(_batch.at(2, row_index))?;
                        let SESSION1 = read_string(_batch.at(3, row_index))?;
                        let SESSION2 = read_string(_batch.at(4, row_index))?;
                        let SESSION3 = read_string(_batch.at(5, row_index))?;
                        let SESSION4 = read_string(_batch.at(6, row_index))?;
                        let SESSION5 = read_string(_batch.at(7, row_index))?;
                        let SESSION6 = read_string(_batch.at(8, row_index))?;
                        let SESSION7 = read_string(_batch.at(9, row_index))?;
                        let SESSION8 = read_string(_batch.at(10, row_index))?;
                        let SESSION9 = read_string(_batch.at(11, row_index))?;
                        let SESSION10 = read_string(_batch.at(12, row_index))?;
                        let NOTATTENDING = read_bool(_batch.at(13, row_index))?;
                        let obj = Preenter {
                            THEATHLETE,
                            THEMEET,
                            PREENTERED,
                            SESSION1,
                            SESSION2,
                            SESSION3,
                            SESSION4,
                            SESSION5,
                            SESSION6,
                            SESSION7,
                            SESSION8,
                            SESSION9,
                            SESSION10,
                            NOTATTENDING
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
