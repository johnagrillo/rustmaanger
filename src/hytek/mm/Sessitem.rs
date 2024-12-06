// 
// 
// CREATE TABLE [Sessitem]
// (
// [Sess_order] Long Integer,
// [Sess_ptr] Long Integer,
// [Event_ptr] Long Integer,
// [Sess_rnd] Text (2),
// [Rept_type] Text (2),
// [Delay_seconds] Long Integer,
// [Alt_With] Boolean NOT NULL,
// [Timed_finalheats] Integer,
// [EventTo_AlternateWith] Long Integer,
// [Delay_desc] Text (100),
// [AltHeats_StartCount] Integer,
// [Event_Interval] Long Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Sessitem {
    sess_order : Option<u64>,
    sess_ptr : Option<u64>,
    event_ptr : Option<u64>,
    sess_rnd : Option<String>,
    rept_type : Option<String>,
    delay_seconds : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    alt__with : bool,
    timed_finalheats : Option<u16>,
    event_to__alternate_with : Option<u64>,
    delay_desc : Option<String>,
    alt_heats__start_count : Option<u16>,
    event__interval : Option<u64>
}
impl Sessitem {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Sessitem>, Box<dyn Error>> {
       let mut vec: Vec<Sessitem> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Sessitem = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Sessitem>, Error> {
        let mut vec: Vec<Sessitem> = Vec::new();
        match conn.execute("SELECT * FROM sessitem", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let sess_order = read_u64(_batch.at(0, row_index));
                        let sess_ptr = read_u64(_batch.at(1, row_index));
                        let event_ptr = read_u64(_batch.at(2, row_index));
                        let sess_rnd = read_string(_batch.at(3, row_index));
                        let rept_type = read_string(_batch.at(4, row_index));
                        let delay_seconds = read_u64(_batch.at(5, row_index));
                        let alt__with = read_bool(_batch.at(6, row_index));
                        let timed_finalheats = read_u16(_batch.at(7, row_index));
                        let event_to__alternate_with = read_u64(_batch.at(8, row_index));
                        let delay_desc = read_string(_batch.at(9, row_index));
                        let alt_heats__start_count = read_u16(_batch.at(10, row_index));
                        let event__interval = read_u64(_batch.at(11, row_index));
                        let obj = Sessitem {
                            sess_order,
                            sess_ptr,
                            event_ptr,
                            sess_rnd,
                            rept_type,
                            delay_seconds,
                            alt__with,
                            timed_finalheats,
                            event_to__alternate_with,
                            delay_desc,
                            alt_heats__start_count,
                            event__interval
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
