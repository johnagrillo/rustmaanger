// 
// 
// CREATE TABLE [MTEVENT]
// (
// [Meet] Long Integer,
// [MtEv] Integer,
// [MtEvX] Text (2),
// [Lo_Hi] Integer,
// [Course] Text (2),
// [MtEvent] Long Integer,
// [Distance] Integer,
// [Stroke] Integer,
// [Sex] Text (2),
// [I_R] Text (2),
// [Session] Byte,
// [Division] Text (6),
// [EventType] Text (2),
// [SessX] Text (2)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Mtevent {
    meet : Option<u64>,
    mt_ev : Option<u16>,
    mt_ev_x : Option<String>,
    lo__hi : Option<u16>,
    course : Option<String>,
    mt_event : u64,
    distance : Option<u16>,
    stroke : Option<u16>,
    sex : Option<String>,
    I_R : Option<String>,
    session : Option<u8>,
    division : Option<String>,
    event_type : Option<String>,
    sess_x : Option<String>
}
impl Mtevent {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Mtevent>, Box<dyn Error>> {
       let mut vec: Vec<Mtevent> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Mtevent = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Mtevent>, Box<dyn Error>> {
        let mut vec: Vec<Mtevent> = Vec::new();
        match conn.execute("SELECT * FROM MTEVENT", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let meet = read_u64(_batch.at(0, row_index))?;
                        let mt_ev = read_u16(_batch.at(1, row_index))?;
                        let mt_ev_x = read_string(_batch.at(2, row_index))?;
                        let lo__hi = read_u16(_batch.at(3, row_index))?;
                        let course = read_string(_batch.at(4, row_index))?;
                        let mt_event = read_u64(_batch.at(5, row_index))?;
                        let distance = read_u16(_batch.at(6, row_index))?;
                        let stroke = read_u16(_batch.at(7, row_index))?;
                        let sex = read_string(_batch.at(8, row_index))?;
                        let I_R = read_string(_batch.at(9, row_index))?;
                        let session = read_u8(_batch.at(10, row_index))?;
                        let division = read_string(_batch.at(11, row_index))?;
                        let event_type = read_string(_batch.at(12, row_index))?;
                        let sess_x = read_string(_batch.at(13, row_index))?;
                        let obj = Mtevent {
                            meet,
                            mt_ev,
                            mt_ev_x,
                            lo__hi,
                            course,
                            mt_event,
                            distance,
                            stroke,
                            sex,
                            I_R,
                            session,
                            division,
                            event_type,
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
