// 
// 
// CREATE TABLE [MSDecline]
// (
// [MName] Text (90),
// [Start] DateTime,
// [MeetSharingMeetID] Long Integer,
// [MeetRegistrationOpens] DateTime,
// [MeetRegistrationCloses] DateTime,
// [MeetSharingStatus] Text (40),
// [MeetSharingPayStatus] Text (40)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Msdecline {
    m_name : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    start : Option<NaiveDateTime>,
    meet_sharing_meet_i_d : Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    meet_registration_opens : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    meet_registration_closes : Option<NaiveDateTime>,
    meet_sharing_status : Option<String>,
    meet_sharing_pay_status : Option<String>
}
impl Msdecline {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Msdecline>, Box<dyn Error>> {
       let mut vec: Vec<Msdecline> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Msdecline = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Msdecline>, Box<dyn Error>> {
        let mut vec: Vec<Msdecline> = Vec::new();
        match conn.execute("SELECT * FROM m s decline", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let m_name = read_string(_batch.at(0, row_index))?;
                        let start = read_datetime(_batch.at(1, row_index))?;
                        let meet_sharing_meet_i_d = read_u64(_batch.at(2, row_index))?;
                        let meet_registration_opens = read_datetime(_batch.at(3, row_index))?;
                        let meet_registration_closes = read_datetime(_batch.at(4, row_index))?;
                        let meet_sharing_status = read_string(_batch.at(5, row_index))?;
                        let meet_sharing_pay_status = read_string(_batch.at(6, row_index))?;
                        let obj = Msdecline {
                            m_name,
                            start,
                            meet_sharing_meet_i_d,
                            meet_registration_opens,
                            meet_registration_closes,
                            meet_sharing_status,
                            meet_sharing_pay_status
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
