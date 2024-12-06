// 
// 
// CREATE TABLE [MEETMOBILE2OPTIONS]
// (
// [MeetMobile2MeetID] Long Integer,
// [AgencyID] Text (80),
// [EV5_uploaded] Boolean NOT NULL,
// [Heatpsych_uploaded] Boolean NOT NULL,
// [Heatsheet_amount] Single,
// [Teamscoring_choice] Integer,
// [Contract_agreed] Boolean NOT NULL,
// [DoNotShow_timeline] Boolean NOT NULL,
// [HeatPsych_choice] Integer,
// [Contract_PersonName] Text (80),
// [Contract_PersonBirth] DateTime,
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
// [Token] Text (160),
// [OMEWebSite] Text (200),
// [License] Text (240),
// [NoShowActiveComSetup] Boolean NOT NULL,
// [AnnounceEV5_uploaded] Boolean NOT NULL,
// [SharingEV5_uploaded] Boolean NOT NULL,
// [FileSharingMeetID] Long Integer,
// [FileSharingEvtsOk] Boolean NOT NULL,
// [FileSharingPricingOk] Boolean NOT NULL,
// [FileSharingEntryLimitsOk] Boolean NOT NULL,
// [FileSharingQualTimesOk] Boolean NOT NULL,
// [FileSharingMeetSetupOk] Boolean NOT NULL,
// [NotInterestedIn_FileSharing] Boolean NOT NULL,
// [DoNotShow_MeetSharingImportMsg] Boolean NOT NULL,
// [ResultsFile_uploaded] Boolean NOT NULL,
// [MeetResultsID] Long Integer,
// [NotInterestedIn_MeetMobile] Boolean NOT NULL,
// [Contract_agreedMeetSharing] Boolean NOT NULL,
// [Contract_PersonNameMeetSharing] Text (80),
// [Contract_PersonBirthMeetSharing] DateTime,
// [HeatSheetsAre_Free] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Meetmobile2options {
    meet_mobile2_meet_i_d : Option<u64>,
    agency_i_d : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    e_v5__uploaded : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    heatpsych_uploaded : bool,
    heatsheet_amount : Option<f32>,
    teamscoring_choice : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    contract_agreed : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    do_not_show_timeline : bool,
    heat_psych_choice : Option<u16>,
    contract__person_name : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    contract__person_birth : Option<NaiveDateTime>,
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
    token : Option<String>,
    o_m_e_web_site : Option<String>,
    license : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    no_show_active_com_setup : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    announce_e_v5__uploaded : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    sharing_e_v5__uploaded : bool,
    file_sharing_meet_i_d : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    file_sharing_evts_ok : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    file_sharing_pricing_ok : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    file_sharing_entry_limits_ok : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    file_sharing_qual_times_ok : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    file_sharing_meet_setup_ok : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    not_interested_in__file_sharing : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    do_not_show__meet_sharing_import_msg : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    results_file_uploaded : bool,
    meet_results_i_d : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    not_interested_in__meet_mobile : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    contract_agreed_meet_sharing : bool,
    contract__person_name_meet_sharing : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    contract__person_birth_meet_sharing : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    heat_sheets_are__free : bool
}
impl Meetmobile2options {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Meetmobile2options>, Box<dyn Error>> {
       let mut vec: Vec<Meetmobile2options> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Meetmobile2options = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Meetmobile2options>, Error> {
        let mut vec: Vec<Meetmobile2options> = Vec::new();
        match conn.execute("SELECT * FROM MEETMOBILE2OPTIONS", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let meet_mobile2_meet_i_d = read_u64(_batch.at(0, row_index));
                        let agency_i_d = read_string(_batch.at(1, row_index));
                        let e_v5__uploaded = read_bool(_batch.at(2, row_index));
                        let heatpsych_uploaded = read_bool(_batch.at(3, row_index));
                        let heatsheet_amount = read_f32(_batch.at(4, row_index));
                        let teamscoring_choice = read_u16(_batch.at(5, row_index));
                        let contract_agreed = read_bool(_batch.at(6, row_index));
                        let do_not_show_timeline = read_bool(_batch.at(7, row_index));
                        let heat_psych_choice = read_u16(_batch.at(8, row_index));
                        let contract__person_name = read_string(_batch.at(9, row_index));
                        let contract__person_birth = read_datetime(_batch.at(10, row_index));
                        let last_name = read_string(_batch.at(11, row_index));
                        let first_name = read_string(_batch.at(12, row_index));
                        let phone = read_string(_batch.at(13, row_index));
                        let e_mail = read_string(_batch.at(14, row_index));
                        let team_name = read_string(_batch.at(15, row_index));
                        let addr = read_string(_batch.at(16, row_index));
                        let sec_addr = read_string(_batch.at(17, row_index));
                        let city = read_string(_batch.at(18, row_index));
                        let state = read_string(_batch.at(19, row_index));
                        let zip = read_string(_batch.at(20, row_index));
                        let cntry = read_string(_batch.at(21, row_index));
                        let time_zone = read_string(_batch.at(22, row_index));
                        let pay_to = read_string(_batch.at(23, row_index));
                        let classification = read_string(_batch.at(24, row_index));
                        let web_site = read_string(_batch.at(25, row_index));
                        let token = read_string(_batch.at(26, row_index));
                        let o_m_e_web_site = read_string(_batch.at(27, row_index));
                        let license = read_string(_batch.at(28, row_index));
                        let no_show_active_com_setup = read_bool(_batch.at(29, row_index));
                        let announce_e_v5__uploaded = read_bool(_batch.at(30, row_index));
                        let sharing_e_v5__uploaded = read_bool(_batch.at(31, row_index));
                        let file_sharing_meet_i_d = read_u64(_batch.at(32, row_index));
                        let file_sharing_evts_ok = read_bool(_batch.at(33, row_index));
                        let file_sharing_pricing_ok = read_bool(_batch.at(34, row_index));
                        let file_sharing_entry_limits_ok = read_bool(_batch.at(35, row_index));
                        let file_sharing_qual_times_ok = read_bool(_batch.at(36, row_index));
                        let file_sharing_meet_setup_ok = read_bool(_batch.at(37, row_index));
                        let not_interested_in__file_sharing = read_bool(_batch.at(38, row_index));
                        let do_not_show__meet_sharing_import_msg = read_bool(_batch.at(39, row_index));
                        let results_file_uploaded = read_bool(_batch.at(40, row_index));
                        let meet_results_i_d = read_u64(_batch.at(41, row_index));
                        let not_interested_in__meet_mobile = read_bool(_batch.at(42, row_index));
                        let contract_agreed_meet_sharing = read_bool(_batch.at(43, row_index));
                        let contract__person_name_meet_sharing = read_string(_batch.at(44, row_index));
                        let contract__person_birth_meet_sharing = read_datetime(_batch.at(45, row_index));
                        let heat_sheets_are__free = read_bool(_batch.at(46, row_index));
                        let obj = Meetmobile2options {
                            meet_mobile2_meet_i_d,
                            agency_i_d,
                            e_v5__uploaded,
                            heatpsych_uploaded,
                            heatsheet_amount,
                            teamscoring_choice,
                            contract_agreed,
                            do_not_show_timeline,
                            heat_psych_choice,
                            contract__person_name,
                            contract__person_birth,
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
                            token,
                            o_m_e_web_site,
                            license,
                            no_show_active_com_setup,
                            announce_e_v5__uploaded,
                            sharing_e_v5__uploaded,
                            file_sharing_meet_i_d,
                            file_sharing_evts_ok,
                            file_sharing_pricing_ok,
                            file_sharing_entry_limits_ok,
                            file_sharing_qual_times_ok,
                            file_sharing_meet_setup_ok,
                            not_interested_in__file_sharing,
                            do_not_show__meet_sharing_import_msg,
                            results_file_uploaded,
                            meet_results_i_d,
                            not_interested_in__meet_mobile,
                            contract_agreed_meet_sharing,
                            contract__person_name_meet_sharing,
                            contract__person_birth_meet_sharing,
                            heat_sheets_are__free
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
