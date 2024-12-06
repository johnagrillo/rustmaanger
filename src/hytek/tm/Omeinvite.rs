// 
// 
// CREATE TABLE [OMEINVITE]
// (
// [IMEET] Long Integer,
// [IATHLETE] Long Integer,
// [INVITEDATE] DateTime,
// [NEWADD] Boolean NOT NULL,
// [DELETED] Boolean NOT NULL,
// [ITEAM] Long Integer,
// [ITEAMCODE] Text (10),
// [INVITEEMAILSENT] DateTime
// );
#[derive(Serialize,Deserialize,Debug)]
struct Omeinvite {
    IMEET : Option<u64>,
    IATHLETE : Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    INVITEDATE : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    NEWADD : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    DELETED : bool,
    ITEAM : Option<u64>,
    ITEAMCODE : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    INVITEEMAILSENT : Option<NaiveDateTime>
}
impl Omeinvite {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Omeinvite>, Box<dyn Error>> {
       let mut vec: Vec<Omeinvite> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Omeinvite = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Omeinvite>, Box<dyn Error>> {
        let mut vec: Vec<Omeinvite> = Vec::new();
        match conn.execute("SELECT * FROM OMEINVITE", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let IMEET = read_u64(_batch.at(0, row_index))?;
                        let IATHLETE = read_u64(_batch.at(1, row_index))?;
                        let INVITEDATE = read_datetime(_batch.at(2, row_index))?;
                        let NEWADD = read_bool(_batch.at(3, row_index))?;
                        let DELETED = read_bool(_batch.at(4, row_index))?;
                        let ITEAM = read_u64(_batch.at(5, row_index))?;
                        let ITEAMCODE = read_string(_batch.at(6, row_index))?;
                        let INVITEEMAILSENT = read_datetime(_batch.at(7, row_index))?;
                        let obj = Omeinvite {
                            IMEET,
                            IATHLETE,
                            INVITEDATE,
                            NEWADD,
                            DELETED,
                            ITEAM,
                            ITEAMCODE,
                            INVITEEMAILSENT
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
