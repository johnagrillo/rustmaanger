// 
// 
// CREATE TABLE [RecordsbyEvent]
// (
// [tag_ptr] Long Integer,
// [event_ptr] Long Integer,
// [low_age] Integer,
// [high_Age] Integer,
// [Record_month] Integer,
// [Record_day] Integer,
// [Record_year] Integer,
// [Record_Holder] Text (60),
// [Record_Holderteam] Text (32),
// [Relay_Names] Text (100),
// [Record_Time] Single,
// [Record_course] Text (2),
// [tag_gender] Text (2),
// [hide_me] Boolean NOT NULL,
// [Record_teamabbr] Text (10),
// [Record_teamlsc] Text (4),
// [div_abbr] Text (20)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Recordsbyevent {
    tag_ptr : Option<u64>,
    event_ptr : Option<u64>,
    low_age : Option<u16>,
    high__age : Option<u16>,
    record_month : Option<u16>,
    record_day : Option<u16>,
    record_year : Option<u16>,
    record__holder : Option<String>,
    record__holderteam : Option<String>,
    relay__names : Option<String>,
    record__time : Option<f32>,
    record_course : Option<String>,
    tag_gender : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    hide_me : bool,
    record_teamabbr : Option<String>,
    record_teamlsc : Option<String>,
    div_abbr : Option<String>
}
impl Recordsbyevent {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Recordsbyevent>, Box<dyn Error>> {
       let mut vec: Vec<Recordsbyevent> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Recordsbyevent = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Recordsbyevent>, Error> {
        let mut vec: Vec<Recordsbyevent> = Vec::new();
        match conn.execute("SELECT * FROM recordsby event", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let tag_ptr = read_u64(_batch.at(0, row_index));
                        let event_ptr = read_u64(_batch.at(1, row_index));
                        let low_age = read_u16(_batch.at(2, row_index));
                        let high__age = read_u16(_batch.at(3, row_index));
                        let record_month = read_u16(_batch.at(4, row_index));
                        let record_day = read_u16(_batch.at(5, row_index));
                        let record_year = read_u16(_batch.at(6, row_index));
                        let record__holder = read_string(_batch.at(7, row_index));
                        let record__holderteam = read_string(_batch.at(8, row_index));
                        let relay__names = read_string(_batch.at(9, row_index));
                        let record__time = read_f32(_batch.at(10, row_index));
                        let record_course = read_string(_batch.at(11, row_index));
                        let tag_gender = read_string(_batch.at(12, row_index));
                        let hide_me = read_bool(_batch.at(13, row_index));
                        let record_teamabbr = read_string(_batch.at(14, row_index));
                        let record_teamlsc = read_string(_batch.at(15, row_index));
                        let div_abbr = read_string(_batch.at(16, row_index));
                        let obj = Recordsbyevent {
                            tag_ptr,
                            event_ptr,
                            low_age,
                            high__age,
                            record_month,
                            record_day,
                            record_year,
                            record__holder,
                            record__holderteam,
                            relay__names,
                            record__time,
                            record_course,
                            tag_gender,
                            hide_me,
                            record_teamabbr,
                            record_teamlsc,
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
