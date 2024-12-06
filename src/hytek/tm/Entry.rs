// 
// 
// CREATE TABLE [ENTRY]
// (
// [Meet] Long Integer,
// [Athlete] Long Integer,
// [I_R] Text (2),
// [Team] Long Integer,
// [Course] Text (4),
// [Score] Long Integer,
// [Ex] Text (2),
// [MtEvent] Long Integer,
// [Misc] Text (2),
// [Entry] Long Integer,
// [Division] Text (6),
// [HEAT] Byte,
// [LANE] Byte,
// [FromOME] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Entry {
    meet : Option<u64>,
    athlete : Option<u64>,
    I_R : Option<String>,
    team : Option<u64>,
    course : Option<String>,
    score : Option<u64>,
    ex : Option<String>,
    mt_event : Option<u64>,
    misc : Option<String>,
    entry : u64,
    division : Option<String>,
    HEAT : Option<u8>,
    LANE : Option<u8>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    from_o_m_e : bool
}
impl Entry {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Entry>, Box<dyn Error>> {
       let mut vec: Vec<Entry> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Entry = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Entry>, Box<dyn Error>> {
        let mut vec: Vec<Entry> = Vec::new();
        match conn.execute("SELECT * FROM ENTRY", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let meet = read_u64(_batch.at(0, row_index))?;
                        let athlete = read_u64(_batch.at(1, row_index))?;
                        let I_R = read_string(_batch.at(2, row_index))?;
                        let team = read_u64(_batch.at(3, row_index))?;
                        let course = read_string(_batch.at(4, row_index))?;
                        let score = read_u64(_batch.at(5, row_index))?;
                        let ex = read_string(_batch.at(6, row_index))?;
                        let mt_event = read_u64(_batch.at(7, row_index))?;
                        let misc = read_string(_batch.at(8, row_index))?;
                        let entry = read_u64(_batch.at(9, row_index))?;
                        let division = read_string(_batch.at(10, row_index))?;
                        let HEAT = read_u8(_batch.at(11, row_index))?;
                        let LANE = read_u8(_batch.at(12, row_index))?;
                        let from_o_m_e = read_bool(_batch.at(13, row_index))?;
                        let obj = Entry {
                            meet,
                            athlete,
                            I_R,
                            team,
                            course,
                            score,
                            ex,
                            mt_event,
                            misc,
                            entry,
                            division,
                            HEAT,
                            LANE,
                            from_o_m_e
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
