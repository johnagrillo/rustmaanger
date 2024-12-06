// 
// 
// CREATE TABLE [Session]
// (
// [Sess_no] Integer,
// [Sess_ltr] Text (2),
// [Sess_ptr] Long Integer,
// [Sess_day] Integer,
// [Sess_starttime] Long Integer,
// [Sess_entrymax] Integer,
// [Sess_name] Text (120),
// [Sess_interval] Integer,
// [Sess_course] Text (2),
// [Sess_entrymaxind] Integer,
// [Sess_entrymaxrel] Integer,
// [Sess_backinterval] Integer,
// [Sess_divinginterval] Integer,
// [Sess_chaseinterval] Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Session {
    sess_no : Option<u16>,
    sess_ltr : Option<String>,
    sess_ptr : Option<u64>,
    sess_day : Option<u16>,
    sess_starttime : Option<u64>,
    sess_entrymax : Option<u16>,
    sess_name : Option<String>,
    sess_interval : Option<u16>,
    sess_course : Option<String>,
    sess_entrymaxind : Option<u16>,
    sess_entrymaxrel : Option<u16>,
    sess_backinterval : Option<u16>,
    sess_divinginterval : Option<u16>,
    sess_chaseinterval : Option<u16>
}
impl Session {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Session>, Box<dyn Error>> {
       let mut vec: Vec<Session> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Session = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Session>, Error> {
        let mut vec: Vec<Session> = Vec::new();
        match conn.execute("SELECT * FROM session", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let sess_no = read_u16(_batch.at(0, row_index));
                        let sess_ltr = read_string(_batch.at(1, row_index));
                        let sess_ptr = read_u64(_batch.at(2, row_index));
                        let sess_day = read_u16(_batch.at(3, row_index));
                        let sess_starttime = read_u64(_batch.at(4, row_index));
                        let sess_entrymax = read_u16(_batch.at(5, row_index));
                        let sess_name = read_string(_batch.at(6, row_index));
                        let sess_interval = read_u16(_batch.at(7, row_index));
                        let sess_course = read_string(_batch.at(8, row_index));
                        let sess_entrymaxind = read_u16(_batch.at(9, row_index));
                        let sess_entrymaxrel = read_u16(_batch.at(10, row_index));
                        let sess_backinterval = read_u16(_batch.at(11, row_index));
                        let sess_divinginterval = read_u16(_batch.at(12, row_index));
                        let sess_chaseinterval = read_u16(_batch.at(13, row_index));
                        let obj = Session {
                            sess_no,
                            sess_ltr,
                            sess_ptr,
                            sess_day,
                            sess_starttime,
                            sess_entrymax,
                            sess_name,
                            sess_interval,
                            sess_course,
                            sess_entrymaxind,
                            sess_entrymaxrel,
                            sess_backinterval,
                            sess_divinginterval,
                            sess_chaseinterval
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
