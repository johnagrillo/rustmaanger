// 
// 
// CREATE TABLE [OPTIONS]
// (
// [State] Text (6),
// [Citizen] Text (6),
// [LSC] Text (6),
// [Standard] Text (16),
// [Record] Text (16),
// [Course] Text (2),
// [TeamType] Text (6),
// [RegType] Text (8),
// [Line1] Text (80),
// [Line2] Text (80),
// [RptUCase] Boolean NOT NULL,
// [SysAgeUp] DateTime,
// [MtAgeUp] Integer,
// [MtDate] DateTime,
// [FavSex] Byte,
// [NoAge] Byte,
// [MinAge] Byte,
// [FavTeam] Text (10),
// [FavLSC] Text (6),
// [CvtFactor] Double,
// [CvtType] Byte,
// [Cvt500] Double,
// [Cvt1650] Double,
// [FreeTurn] Double,
// [BackTurn] Double,
// [BreastTurn] Double,
// [FlyTurn] Double,
// [IMTurn] Double,
// [Straight] Double,
// [Y2L] Boolean NOT NULL,
// [Language] Text (16),
// [LabelOption] Byte,
// [ShowMI] Boolean NOT NULL,
// [ShowAges] Boolean NOT NULL,
// [ShowBirth] Boolean NOT NULL,
// [ShowClass] Boolean NOT NULL,
// [ShowFirstFirst] Boolean NOT NULL,
// [Favorite] Text (16),
// [ToScreen] Boolean NOT NULL,
// [NumCopies] Byte,
// [PaperSize] Byte,
// [UseLeadOff] Boolean NOT NULL,
// [UseStrokeRates] Boolean NOT NULL,
// [CusPrompt1] Text (40),
// [CusPrompt2] Text (40),
// [CusPrompt3] Text (40),
// [RegDate] DateTime,
// [ShowOnlyFast] Boolean NOT NULL,
// [LabelType] Byte,
// [ImportDir] Text (240),
// [ExportDir] Text (240),
// [BackupDir] Text (240),
// [ReportDir] Text (240),
// [LSCFee] Single,
// [RegAddr] Text (100),
// [RegCity] Text (40),
// [RegState] Text (6),
// [RegZIP] Text (20),
// [RegMailTo] Text (80),
// [RegPayTo] Text (80),
// [PalmImportDir] Text (240),
// [LSCSeasonalFee] Single,
// [BackupCheck] Byte,
// [UpdateCheck] Byte,
// [LastBackup] DateTime,
// [LastSplitExtract] DateTime,
// [AgeupToCurrent] Boolean NOT NULL,
// [DbVersion] Text (10),
// [DefaultDir] Text (240),
// [RelayOrder] Text (14),
// [LastDir] Text (240),
// [SplitPref] Integer,
// [rptLastFirst] Boolean NOT NULL,
// [rptShowMI] Boolean NOT NULL,
// [rptFILast] Boolean NOT NULL,
// [rptFirstLI] Boolean NOT NULL,
// [rptShowPref] Boolean NOT NULL,
// [OfficialLine1] Text (216),
// [OfficialLine2] Text (216),
// [OfficialLine3] Text (216),
// [UseProvince] Boolean NOT NULL,
// [DefCity] Text (60),
// [CusPrompt4] Text (40),
// [CusPrompt5] Text (40),
// [CusPrompt6] Text (40),
// [CusPrompt7] Text (40),
// [CusPrompt8] Text (40),
// [OnlineRegistrationType] Text (2),
// [DefZip] Text (20),
// [DontShowMeetSharing] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Options {
    state : Option<String>,
    citizen : Option<String>,
    LSC : Option<String>,
    standard : Option<String>,
    record : Option<String>,
    course : Option<String>,
    team_type : Option<String>,
    reg_type : Option<String>,
    line1 : Option<String>,
    line2 : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    rpt_u_case : bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    sys_age_up : Option<NaiveDateTime>,
    mt_age_up : Option<u16>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    mt_date : Option<NaiveDateTime>,
    fav_sex : Option<u8>,
    no_age : Option<u8>,
    min_age : Option<u8>,
    fav_team : Option<String>,
    fav_l_s_c : Option<String>,
    cvt_factor : Option<f64>,
    cvt_type : Option<u8>,
    cvt5_0_0 : Option<f64>,
    cvt1_6_5_0 : Option<f64>,
    free_turn : Option<f64>,
    back_turn : Option<f64>,
    breast_turn : Option<f64>,
    fly_turn : Option<f64>,
    i_m_turn : Option<f64>,
    straight : Option<f64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    Y2L : bool,
    language : Option<String>,
    label_option : Option<u8>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_m_i : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_ages : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_birth : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_class : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_first_first : bool,
    favorite : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    to_screen : bool,
    num_copies : Option<u8>,
    paper_size : Option<u8>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_lead_off : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_stroke_rates : bool,
    cus_prompt1 : Option<String>,
    cus_prompt2 : Option<String>,
    cus_prompt3 : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    reg_date : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_only_fast : bool,
    label_type : Option<u8>,
    import_dir : Option<String>,
    export_dir : Option<String>,
    backup_dir : Option<String>,
    report_dir : Option<String>,
    l_s_c_fee : Option<f32>,
    reg_addr : Option<String>,
    reg_city : Option<String>,
    reg_state : Option<String>,
    reg_z_i_p : Option<String>,
    reg_mail_to : Option<String>,
    reg_pay_to : Option<String>,
    palm_import_dir : Option<String>,
    l_s_c_seasonal_fee : Option<f32>,
    backup_check : Option<u8>,
    update_check : Option<u8>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    last_backup : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    last_split_extract : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    ageup_to_current : bool,
    db_version : Option<String>,
    default_dir : Option<String>,
    relay_order : Option<String>,
    last_dir : Option<String>,
    split_pref : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    rpt_last_first : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    rpt_show_m_i : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    rpt_f_i_last : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    rpt_first_l_i : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    rpt_show_pref : bool,
    official_line1 : Option<String>,
    official_line2 : Option<String>,
    official_line3 : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_province : bool,
    def_city : Option<String>,
    cus_prompt4 : Option<String>,
    cus_prompt5 : Option<String>,
    cus_prompt6 : Option<String>,
    cus_prompt7 : Option<String>,
    cus_prompt8 : Option<String>,
    online_registration_type : Option<String>,
    def_zip : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    dont_show_meet_sharing : bool
}
impl Options {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Options>, Box<dyn Error>> {
       let mut vec: Vec<Options> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Options = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Options>, Box<dyn Error>> {
        let mut vec: Vec<Options> = Vec::new();
        match conn.execute("SELECT * FROM OPTIONS", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let state = read_string(_batch.at(0, row_index))?;
                        let citizen = read_string(_batch.at(1, row_index))?;
                        let LSC = read_string(_batch.at(2, row_index))?;
                        let standard = read_string(_batch.at(3, row_index))?;
                        let record = read_string(_batch.at(4, row_index))?;
                        let course = read_string(_batch.at(5, row_index))?;
                        let team_type = read_string(_batch.at(6, row_index))?;
                        let reg_type = read_string(_batch.at(7, row_index))?;
                        let line1 = read_string(_batch.at(8, row_index))?;
                        let line2 = read_string(_batch.at(9, row_index))?;
                        let rpt_u_case = read_bool(_batch.at(10, row_index))?;
                        let sys_age_up = read_datetime(_batch.at(11, row_index))?;
                        let mt_age_up = read_u16(_batch.at(12, row_index))?;
                        let mt_date = read_datetime(_batch.at(13, row_index))?;
                        let fav_sex = read_u8(_batch.at(14, row_index))?;
                        let no_age = read_u8(_batch.at(15, row_index))?;
                        let min_age = read_u8(_batch.at(16, row_index))?;
                        let fav_team = read_string(_batch.at(17, row_index))?;
                        let fav_l_s_c = read_string(_batch.at(18, row_index))?;
                        let cvt_factor = read_f64(_batch.at(19, row_index))?;
                        let cvt_type = read_u8(_batch.at(20, row_index))?;
                        let cvt5_0_0 = read_f64(_batch.at(21, row_index))?;
                        let cvt1_6_5_0 = read_f64(_batch.at(22, row_index))?;
                        let free_turn = read_f64(_batch.at(23, row_index))?;
                        let back_turn = read_f64(_batch.at(24, row_index))?;
                        let breast_turn = read_f64(_batch.at(25, row_index))?;
                        let fly_turn = read_f64(_batch.at(26, row_index))?;
                        let i_m_turn = read_f64(_batch.at(27, row_index))?;
                        let straight = read_f64(_batch.at(28, row_index))?;
                        let Y2L = read_bool(_batch.at(29, row_index))?;
                        let language = read_string(_batch.at(30, row_index))?;
                        let label_option = read_u8(_batch.at(31, row_index))?;
                        let show_m_i = read_bool(_batch.at(32, row_index))?;
                        let show_ages = read_bool(_batch.at(33, row_index))?;
                        let show_birth = read_bool(_batch.at(34, row_index))?;
                        let show_class = read_bool(_batch.at(35, row_index))?;
                        let show_first_first = read_bool(_batch.at(36, row_index))?;
                        let favorite = read_string(_batch.at(37, row_index))?;
                        let to_screen = read_bool(_batch.at(38, row_index))?;
                        let num_copies = read_u8(_batch.at(39, row_index))?;
                        let paper_size = read_u8(_batch.at(40, row_index))?;
                        let use_lead_off = read_bool(_batch.at(41, row_index))?;
                        let use_stroke_rates = read_bool(_batch.at(42, row_index))?;
                        let cus_prompt1 = read_string(_batch.at(43, row_index))?;
                        let cus_prompt2 = read_string(_batch.at(44, row_index))?;
                        let cus_prompt3 = read_string(_batch.at(45, row_index))?;
                        let reg_date = read_datetime(_batch.at(46, row_index))?;
                        let show_only_fast = read_bool(_batch.at(47, row_index))?;
                        let label_type = read_u8(_batch.at(48, row_index))?;
                        let import_dir = read_string(_batch.at(49, row_index))?;
                        let export_dir = read_string(_batch.at(50, row_index))?;
                        let backup_dir = read_string(_batch.at(51, row_index))?;
                        let report_dir = read_string(_batch.at(52, row_index))?;
                        let l_s_c_fee = read_f32(_batch.at(53, row_index))?;
                        let reg_addr = read_string(_batch.at(54, row_index))?;
                        let reg_city = read_string(_batch.at(55, row_index))?;
                        let reg_state = read_string(_batch.at(56, row_index))?;
                        let reg_z_i_p = read_string(_batch.at(57, row_index))?;
                        let reg_mail_to = read_string(_batch.at(58, row_index))?;
                        let reg_pay_to = read_string(_batch.at(59, row_index))?;
                        let palm_import_dir = read_string(_batch.at(60, row_index))?;
                        let l_s_c_seasonal_fee = read_f32(_batch.at(61, row_index))?;
                        let backup_check = read_u8(_batch.at(62, row_index))?;
                        let update_check = read_u8(_batch.at(63, row_index))?;
                        let last_backup = read_datetime(_batch.at(64, row_index))?;
                        let last_split_extract = read_datetime(_batch.at(65, row_index))?;
                        let ageup_to_current = read_bool(_batch.at(66, row_index))?;
                        let db_version = read_string(_batch.at(67, row_index))?;
                        let default_dir = read_string(_batch.at(68, row_index))?;
                        let relay_order = read_string(_batch.at(69, row_index))?;
                        let last_dir = read_string(_batch.at(70, row_index))?;
                        let split_pref = read_u16(_batch.at(71, row_index))?;
                        let rpt_last_first = read_bool(_batch.at(72, row_index))?;
                        let rpt_show_m_i = read_bool(_batch.at(73, row_index))?;
                        let rpt_f_i_last = read_bool(_batch.at(74, row_index))?;
                        let rpt_first_l_i = read_bool(_batch.at(75, row_index))?;
                        let rpt_show_pref = read_bool(_batch.at(76, row_index))?;
                        let official_line1 = read_string(_batch.at(77, row_index))?;
                        let official_line2 = read_string(_batch.at(78, row_index))?;
                        let official_line3 = read_string(_batch.at(79, row_index))?;
                        let use_province = read_bool(_batch.at(80, row_index))?;
                        let def_city = read_string(_batch.at(81, row_index))?;
                        let cus_prompt4 = read_string(_batch.at(82, row_index))?;
                        let cus_prompt5 = read_string(_batch.at(83, row_index))?;
                        let cus_prompt6 = read_string(_batch.at(84, row_index))?;
                        let cus_prompt7 = read_string(_batch.at(85, row_index))?;
                        let cus_prompt8 = read_string(_batch.at(86, row_index))?;
                        let online_registration_type = read_string(_batch.at(87, row_index))?;
                        let def_zip = read_string(_batch.at(88, row_index))?;
                        let dont_show_meet_sharing = read_bool(_batch.at(89, row_index))?;
                        let obj = Options {
                            state,
                            citizen,
                            LSC,
                            standard,
                            record,
                            course,
                            team_type,
                            reg_type,
                            line1,
                            line2,
                            rpt_u_case,
                            sys_age_up,
                            mt_age_up,
                            mt_date,
                            fav_sex,
                            no_age,
                            min_age,
                            fav_team,
                            fav_l_s_c,
                            cvt_factor,
                            cvt_type,
                            cvt5_0_0,
                            cvt1_6_5_0,
                            free_turn,
                            back_turn,
                            breast_turn,
                            fly_turn,
                            i_m_turn,
                            straight,
                            Y2L,
                            language,
                            label_option,
                            show_m_i,
                            show_ages,
                            show_birth,
                            show_class,
                            show_first_first,
                            favorite,
                            to_screen,
                            num_copies,
                            paper_size,
                            use_lead_off,
                            use_stroke_rates,
                            cus_prompt1,
                            cus_prompt2,
                            cus_prompt3,
                            reg_date,
                            show_only_fast,
                            label_type,
                            import_dir,
                            export_dir,
                            backup_dir,
                            report_dir,
                            l_s_c_fee,
                            reg_addr,
                            reg_city,
                            reg_state,
                            reg_z_i_p,
                            reg_mail_to,
                            reg_pay_to,
                            palm_import_dir,
                            l_s_c_seasonal_fee,
                            backup_check,
                            update_check,
                            last_backup,
                            last_split_extract,
                            ageup_to_current,
                            db_version,
                            default_dir,
                            relay_order,
                            last_dir,
                            split_pref,
                            rpt_last_first,
                            rpt_show_m_i,
                            rpt_f_i_last,
                            rpt_first_l_i,
                            rpt_show_pref,
                            official_line1,
                            official_line2,
                            official_line3,
                            use_province,
                            def_city,
                            cus_prompt4,
                            cus_prompt5,
                            cus_prompt6,
                            cus_prompt7,
                            cus_prompt8,
                            online_registration_type,
                            def_zip,
                            dont_show_meet_sharing
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
