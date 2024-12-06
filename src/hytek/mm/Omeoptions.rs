// 
// 
// CREATE TABLE [OMEOPTIONS]
// (
// [OMEEntryStyle] Byte,
// [OMEReviewTime] Byte,
// [EMailFrom] Text (100),
// [EMailSubject] Text (200),
// [EMailText] Text (508),
// [LastName] Text (100),
// [FirstName] Text (100),
// [Phone] Text (40),
// [EMail] Text (100),
// [TeamName] Text (100),
// [Addr] Text (100),
// [SecAddr] Text (60),
// [City] Text (60),
// [State] Text (6),
// [Zip] Text (20),
// [Cntry] Text (6),
// [TimeZone] Text (100),
// [PayTo] Text (100),
// [Classification] Text (100),
// [WebSite] Text (100),
// [NoShowMeetList] Boolean NOT NULL,
// [NoShowMeetSetup] Boolean NOT NULL,
// [NoShowActiveComSetup] Boolean NOT NULL,
// [Token] Text (160),
// [OMEWebSite] Text (200),
// [AgencyID] Text (80),
// [License] Text (240),
// [ActiveMeetID] Long Integer,
// [EV4_uploaded] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Omeoptions {
    o_m_e_entry_style : Option<u8>,
    o_m_e_review_time : Option<u8>,
    e_mail_from : Option<String>,
    e_mail_subject : Option<String>,
    e_mail_text : Option<String>,
    last_name : Option<String>,
    first_name : Option<String>,
    phone : Option<String>,
    e_mail : Option<String>,
    team_name : Option<String>,
    addr : Option<String>,
    sec_addr : Option<String>,
    city : Option<String>,
    state : Option<String>,
    zip : Option<String>,
    cntry : Option<String>,
    time_zone : Option<String>,
    pay_to : Option<String>,
    classification : Option<String>,
    web_site : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    no_show_meet_list : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    no_show_meet_setup : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    no_show_active_com_setup : bool,
    token : Option<String>,
    o_m_e_web_site : Option<String>,
    agency_i_d : Option<String>,
    license : Option<String>,
    active_meet_i_d : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    e_v4__uploaded : bool
}
impl Omeoptions {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Omeoptions>, Box<dyn Error>> {
       let mut vec: Vec<Omeoptions> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Omeoptions = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Omeoptions>, Error> {
        let mut vec: Vec<Omeoptions> = Vec::new();
        match conn.execute("SELECT * FROM OMEOPTIONS", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let o_m_e_entry_style = read_u8(_batch.at(0, row_index));
                        let o_m_e_review_time = read_u8(_batch.at(1, row_index));
                        let e_mail_from = read_string(_batch.at(2, row_index));
                        let e_mail_subject = read_string(_batch.at(3, row_index));
                        let e_mail_text = read_string(_batch.at(4, row_index));
                        let last_name = read_string(_batch.at(5, row_index));
                        let first_name = read_string(_batch.at(6, row_index));
                        let phone = read_string(_batch.at(7, row_index));
                        let e_mail = read_string(_batch.at(8, row_index));
                        let team_name = read_string(_batch.at(9, row_index));
                        let addr = read_string(_batch.at(10, row_index));
                        let sec_addr = read_string(_batch.at(11, row_index));
                        let city = read_string(_batch.at(12, row_index));
                        let state = read_string(_batch.at(13, row_index));
                        let zip = read_string(_batch.at(14, row_index));
                        let cntry = read_string(_batch.at(15, row_index));
                        let time_zone = read_string(_batch.at(16, row_index));
                        let pay_to = read_string(_batch.at(17, row_index));
                        let classification = read_string(_batch.at(18, row_index));
                        let web_site = read_string(_batch.at(19, row_index));
                        let no_show_meet_list = read_bool(_batch.at(20, row_index));
                        let no_show_meet_setup = read_bool(_batch.at(21, row_index));
                        let no_show_active_com_setup = read_bool(_batch.at(22, row_index));
                        let token = read_string(_batch.at(23, row_index));
                        let o_m_e_web_site = read_string(_batch.at(24, row_index));
                        let agency_i_d = read_string(_batch.at(25, row_index));
                        let license = read_string(_batch.at(26, row_index));
                        let active_meet_i_d = read_u64(_batch.at(27, row_index));
                        let e_v4__uploaded = read_bool(_batch.at(28, row_index));
                        let obj = Omeoptions {
                            o_m_e_entry_style,
                            o_m_e_review_time,
                            e_mail_from,
                            e_mail_subject,
                            e_mail_text,
                            last_name,
                            first_name,
                            phone,
                            e_mail,
                            team_name,
                            addr,
                            sec_addr,
                            city,
                            state,
                            zip,
                            cntry,
                            time_zone,
                            pay_to,
                            classification,
                            web_site,
                            no_show_meet_list,
                            no_show_meet_setup,
                            no_show_active_com_setup,
                            token,
                            o_m_e_web_site,
                            agency_i_d,
                            license,
                            active_meet_i_d,
                            e_v4__uploaded
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
