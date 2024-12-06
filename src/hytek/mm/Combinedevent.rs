// 
// 
// CREATE TABLE [CombinedEvent]
// (
// [Event_no] Integer,
// [Event_ltr] Text (2),
// [CombEvent_ptr] Long Integer,
// [Event_sex] Text (2),
// [Event_gender] Text (2),
// [Low_age] Integer,
// [High_Age] Integer,
// [Num_events] Integer,
// [Score_event] Boolean NOT NULL,
// [Div_no] Integer,
// [Comm_1] Text (72),
// [Comm_2] Text (72),
// [Comm_3] Text (72),
// [Comm_4] Text (72),
// [Entry_fee] Currency,
// [Event_note] Text (40),
// [Event_ptr1] Long Integer,
// [Event_ptr2] Long Integer,
// [Event_ptr3] Long Integer,
// [Event_ptr4] Long Integer,
// [Event_ptr5] Long Integer,
// [Event_ptr6] Long Integer,
// [Event_ptr7] Long Integer,
// [Event_ptr8] Long Integer,
// [Event_ptr9] Long Integer,
// [Event_ptr10] Long Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Combinedevent {
    event_no : Option<u16>,
    event_ltr : Option<String>,
    comb_event_ptr : Option<u64>,
    event_sex : Option<String>,
    event_gender : Option<String>,
    low_age : Option<u16>,
    high__age : Option<u16>,
    num_events : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    score_event : bool,
    div_no : Option<u16>,
    comm__1 : Option<String>,
    comm__2 : Option<String>,
    comm__3 : Option<String>,
    comm__4 : Option<String>,
    entry_fee : Option<Decimal>,
    event_note : Option<String>,
    event_ptr1 : Option<u64>,
    event_ptr2 : Option<u64>,
    event_ptr3 : Option<u64>,
    event_ptr4 : Option<u64>,
    event_ptr5 : Option<u64>,
    event_ptr6 : Option<u64>,
    event_ptr7 : Option<u64>,
    event_ptr8 : Option<u64>,
    event_ptr9 : Option<u64>,
    event_ptr1_0 : Option<u64>
}
impl Combinedevent {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Combinedevent>, Box<dyn Error>> {
       let mut vec: Vec<Combinedevent> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Combinedevent = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Combinedevent>, Error> {
        let mut vec: Vec<Combinedevent> = Vec::new();
        match conn.execute("SELECT * FROM combined event", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let event_no = read_u16(_batch.at(0, row_index));
                        let event_ltr = read_string(_batch.at(1, row_index));
                        let comb_event_ptr = read_u64(_batch.at(2, row_index));
                        let event_sex = read_string(_batch.at(3, row_index));
                        let event_gender = read_string(_batch.at(4, row_index));
                        let low_age = read_u16(_batch.at(5, row_index));
                        let high__age = read_u16(_batch.at(6, row_index));
                        let num_events = read_u16(_batch.at(7, row_index));
                        let score_event = read_bool(_batch.at(8, row_index));
                        let div_no = read_u16(_batch.at(9, row_index));
                        let comm__1 = read_string(_batch.at(10, row_index));
                        let comm__2 = read_string(_batch.at(11, row_index));
                        let comm__3 = read_string(_batch.at(12, row_index));
                        let comm__4 = read_string(_batch.at(13, row_index));
                        let entry_fee = read_decimal(_batch.at(14, row_index));
                        let event_note = read_string(_batch.at(15, row_index));
                        let event_ptr1 = read_u64(_batch.at(16, row_index));
                        let event_ptr2 = read_u64(_batch.at(17, row_index));
                        let event_ptr3 = read_u64(_batch.at(18, row_index));
                        let event_ptr4 = read_u64(_batch.at(19, row_index));
                        let event_ptr5 = read_u64(_batch.at(20, row_index));
                        let event_ptr6 = read_u64(_batch.at(21, row_index));
                        let event_ptr7 = read_u64(_batch.at(22, row_index));
                        let event_ptr8 = read_u64(_batch.at(23, row_index));
                        let event_ptr9 = read_u64(_batch.at(24, row_index));
                        let event_ptr1_0 = read_u64(_batch.at(25, row_index));
                        let obj = Combinedevent {
                            event_no,
                            event_ltr,
                            comb_event_ptr,
                            event_sex,
                            event_gender,
                            low_age,
                            high__age,
                            num_events,
                            score_event,
                            div_no,
                            comm__1,
                            comm__2,
                            comm__3,
                            comm__4,
                            entry_fee,
                            event_note,
                            event_ptr1,
                            event_ptr2,
                            event_ptr3,
                            event_ptr4,
                            event_ptr5,
                            event_ptr6,
                            event_ptr7,
                            event_ptr8,
                            event_ptr9,
                            event_ptr1_0
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
