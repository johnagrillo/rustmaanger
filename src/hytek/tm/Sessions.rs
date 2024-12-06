// 
// 
// CREATE TABLE [SESSIONS]
// (
// [MEETID] Long Integer,
// [SESSION] Byte,
// [MAXIND] Byte,
// [MAXREL] Byte,
// [MAXCOMBINED] Byte,
// [DAY] Byte,
// [STARTTIME] Text (10),
// [AMPM] Boolean NOT NULL,
// [SessX] Text (2)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Sessions {
    MEETID : Option<u64>,
    SESSION : Option<u8>,
    MAXIND : Option<u8>,
    MAXREL : Option<u8>,
    MAXCOMBINED : Option<u8>,
    DAY : Option<u8>,
    STARTTIME : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    AMPM : bool,
    sess_x : Option<String>
}
impl Sessions {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Sessions>, Box<dyn Error>> {
       let mut vec: Vec<Sessions> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Sessions = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Sessions>, Box<dyn Error>> {
        let mut vec: Vec<Sessions> = Vec::new();
        match conn.execute("SELECT * FROM SESSIONS", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let MEETID = read_u64(_batch.at(0, row_index))?;
                        let SESSION = read_u8(_batch.at(1, row_index))?;
                        let MAXIND = read_u8(_batch.at(2, row_index))?;
                        let MAXREL = read_u8(_batch.at(3, row_index))?;
                        let MAXCOMBINED = read_u8(_batch.at(4, row_index))?;
                        let DAY = read_u8(_batch.at(5, row_index))?;
                        let STARTTIME = read_string(_batch.at(6, row_index))?;
                        let AMPM = read_bool(_batch.at(7, row_index))?;
                        let sess_x = read_string(_batch.at(8, row_index))?;
                        let obj = Sessions {
                            MEETID,
                            SESSION,
                            MAXIND,
                            MAXREL,
                            MAXCOMBINED,
                            DAY,
                            STARTTIME,
                            AMPM,
                            sess_x
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
