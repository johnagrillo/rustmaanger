// 
// 
// CREATE TABLE [MEET]
// (
// [Meet] Long Integer,
// [MName] Text (90),
// [Start] DateTime,
// [End] DateTime,
// [AgeUp] DateTime,
// [Since] DateTime,
// [Course] Text (4),
// [Location] Text (90),
// [Remarks] Text (100),
// [IndCharge] Single,
// [RelCharge] Single,
// [SurCharge] Single,
// [Type] Text (6),
// [Sanction] Text (30),
// [MaxIndEnt] Integer,
// [MaxRelEnt] Integer,
// [MaxEnt] Integer,
// [RestrictBest] Boolean NOT NULL,
// [NonConform] Double,
// [EnterAtQTime] Boolean NOT NULL,
// [FacilityFee] Single,
// [TeamFee] Single,
// [Instructions] Text (500),
// [MinAge] Integer,
// [EnforceQualifying] Boolean NOT NULL,
// [Altitude] Integer,
// [EnforceSlowQtime] Boolean NOT NULL,
// [BanNoTimes] Boolean NOT NULL,
// [Lanes] Integer,
// [EvenOrOdd] Integer,
// [FastToSlow] Boolean NOT NULL,
// [Masters] Boolean NOT NULL,
// [ActiveFeeXMLSent] Boolean NOT NULL,
// [MinAge10AndUnder] Integer,
// [SeedLanes] Text (32),
// [DeadLine] DateTime,
// [ActiveMeetID] Long Integer,
// [CustIndCharge] Single,
// [CustRelCharge] Single,
// [CustSurCharge] Single,
// [CustIndAction] Byte,
// [CustRelAction] Byte,
// [CustSurAction] Byte,
// [Addr] Text (60),
// [Addr2] Text (60),
// [City] Text (60),
// [State] Text (6),
// [ZIP] Text (20),
// [Cntry] Text (40),
// [UseCustomFees] Boolean NOT NULL,
// [OMEEntryStyle] Byte,
// [SwimmerEntryDeadLine] DateTime,
// [SwimmerEntryOpen] DateTime,
// [AllowCustomTimes] Boolean NOT NULL,
// [ExportEntriesDate] DateTime,
// [LastSyncDate] DateTime,
// [FinalizeEntriesDate] DateTime,
// [UseSwimmersTeam] Boolean NOT NULL,
// [HonorInviteList] Boolean NOT NULL,
// [EntryTeam] Long Integer,
// [EMailFrom] Text (100),
// [EMailText] Text (508),
// [EMailSubject] Text (200),
// [OnlyPreEntered] Boolean NOT NULL,
// [License] Text (240),
// [CollectFeesOnline] Boolean NOT NULL,
// [OMEBillDate] DateTime,
// [InviteEmailDate] DateTime,
// [EntryEMailText] Text (508),
// [EntryEMailSubject] Text (200),
// [EntryEMailFrom] Text (100),
// [OMEValidated] Boolean NOT NULL,
// [OMECustomTeam] Boolean NOT NULL,
// [OMEFilterTeam] Long Integer,
// [OMENeedsSync] Boolean NOT NULL,
// [OMENeedSync] Boolean NOT NULL,
// [MeetSharingStatus] Text (40),
// [FileExportType] Integer,
// [MeetRegistrationOpens] DateTime,
// [MeetRegistrationCloses] DateTime,
// [MeetSharingMeetID] Long Integer,
// [MeetSharingPayStatus] Text (40),
// [MeetSharingResultDate] DateTime,
// [EV3Version] Byte
// );
#[derive(Serialize,Deserialize,Debug)]
struct Meet {
    meet : u64,
    m_name : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    start : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    end : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    age_up : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    since : Option<NaiveDateTime>,
    course : Option<String>,
    location : Option<String>,
    remarks : Option<String>,
    ind_charge : Option<f32>,
    rel_charge : Option<f32>,
    sur_charge : Option<f32>,
    ttype : Option<String>,
    sanction : Option<String>,
    max_ind_ent : Option<u16>,
    max_rel_ent : Option<u16>,
    max_ent : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    restrict_best : bool,
    non_conform : Option<f64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    enter_at_q_time : bool,
    facility_fee : Option<f32>,
    team_fee : Option<f32>,
    instructions : Option<String>,
    min_age : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    enforce_qualifying : bool,
    altitude : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    enforce_slow_qtime : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    ban_no_times : bool,
    lanes : Option<u16>,
    even_or_odd : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    fast_to_slow : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    masters : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    active_fee_x_m_l_sent : bool,
    min_age1_0_and_under : Option<u16>,
    seed_lanes : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    dead_line : Option<NaiveDateTime>,
    active_meet_i_d : Option<u64>,
    cust_ind_charge : Option<f32>,
    cust_rel_charge : Option<f32>,
    cust_sur_charge : Option<f32>,
    cust_ind_action : Option<u8>,
    cust_rel_action : Option<u8>,
    cust_sur_action : Option<u8>,
    addr : Option<String>,
    addr2 : Option<String>,
    city : Option<String>,
    state : Option<String>,
    ZIP : Option<String>,
    cntry : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_custom_fees : bool,
    o_m_e_entry_style : Option<u8>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    swimmer_entry_dead_line : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    swimmer_entry_open : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    allow_custom_times : bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    export_entries_date : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    last_sync_date : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    finalize_entries_date : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_swimmers_team : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    honor_invite_list : bool,
    entry_team : Option<u64>,
    e_mail_from : Option<String>,
    e_mail_text : Option<String>,
    e_mail_subject : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    only_pre_entered : bool,
    license : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    collect_fees_online : bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    o_m_e_bill_date : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    invite_email_date : Option<NaiveDateTime>,
    entry_e_mail_text : Option<String>,
    entry_e_mail_subject : Option<String>,
    entry_e_mail_from : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    o_m_e_validated : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    o_m_e_custom_team : bool,
    o_m_e_filter_team : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    o_m_e_needs_sync : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    o_m_e_need_sync : bool,
    meet_sharing_status : Option<String>,
    file_export_type : Option<u16>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    meet_registration_opens : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    meet_registration_closes : Option<NaiveDateTime>,
    meet_sharing_meet_i_d : Option<u64>,
    meet_sharing_pay_status : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    meet_sharing_result_date : Option<NaiveDateTime>,
    e_v3_version : Option<u8>
}
impl Meet {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Meet>, Box<dyn Error>> {
       let mut vec: Vec<Meet> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Meet = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Meet>, Box<dyn Error>> {
        let mut vec: Vec<Meet> = Vec::new();
        match conn.execute("SELECT * FROM MEET", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let meet = read_u64(_batch.at(0, row_index))?;
                        let m_name = read_string(_batch.at(1, row_index))?;
                        let start = read_datetime(_batch.at(2, row_index))?;
                        let end = read_datetime(_batch.at(3, row_index))?;
                        let age_up = read_datetime(_batch.at(4, row_index))?;
                        let since = read_datetime(_batch.at(5, row_index))?;
                        let course = read_string(_batch.at(6, row_index))?;
                        let location = read_string(_batch.at(7, row_index))?;
                        let remarks = read_string(_batch.at(8, row_index))?;
                        let ind_charge = read_f32(_batch.at(9, row_index))?;
                        let rel_charge = read_f32(_batch.at(10, row_index))?;
                        let sur_charge = read_f32(_batch.at(11, row_index))?;
                        let ttype = read_string(_batch.at(12, row_index))?;
                        let sanction = read_string(_batch.at(13, row_index))?;
                        let max_ind_ent = read_u16(_batch.at(14, row_index))?;
                        let max_rel_ent = read_u16(_batch.at(15, row_index))?;
                        let max_ent = read_u16(_batch.at(16, row_index))?;
                        let restrict_best = read_bool(_batch.at(17, row_index))?;
                        let non_conform = read_f64(_batch.at(18, row_index))?;
                        let enter_at_q_time = read_bool(_batch.at(19, row_index))?;
                        let facility_fee = read_f32(_batch.at(20, row_index))?;
                        let team_fee = read_f32(_batch.at(21, row_index))?;
                        let instructions = read_string(_batch.at(22, row_index))?;
                        let min_age = read_u16(_batch.at(23, row_index))?;
                        let enforce_qualifying = read_bool(_batch.at(24, row_index))?;
                        let altitude = read_u16(_batch.at(25, row_index))?;
                        let enforce_slow_qtime = read_bool(_batch.at(26, row_index))?;
                        let ban_no_times = read_bool(_batch.at(27, row_index))?;
                        let lanes = read_u16(_batch.at(28, row_index))?;
                        let even_or_odd = read_u16(_batch.at(29, row_index))?;
                        let fast_to_slow = read_bool(_batch.at(30, row_index))?;
                        let masters = read_bool(_batch.at(31, row_index))?;
                        let active_fee_x_m_l_sent = read_bool(_batch.at(32, row_index))?;
                        let min_age1_0_and_under = read_u16(_batch.at(33, row_index))?;
                        let seed_lanes = read_string(_batch.at(34, row_index))?;
                        let dead_line = read_datetime(_batch.at(35, row_index))?;
                        let active_meet_i_d = read_u64(_batch.at(36, row_index))?;
                        let cust_ind_charge = read_f32(_batch.at(37, row_index))?;
                        let cust_rel_charge = read_f32(_batch.at(38, row_index))?;
                        let cust_sur_charge = read_f32(_batch.at(39, row_index))?;
                        let cust_ind_action = read_u8(_batch.at(40, row_index))?;
                        let cust_rel_action = read_u8(_batch.at(41, row_index))?;
                        let cust_sur_action = read_u8(_batch.at(42, row_index))?;
                        let addr = read_string(_batch.at(43, row_index))?;
                        let addr2 = read_string(_batch.at(44, row_index))?;
                        let city = read_string(_batch.at(45, row_index))?;
                        let state = read_string(_batch.at(46, row_index))?;
                        let ZIP = read_string(_batch.at(47, row_index))?;
                        let cntry = read_string(_batch.at(48, row_index))?;
                        let use_custom_fees = read_bool(_batch.at(49, row_index))?;
                        let o_m_e_entry_style = read_u8(_batch.at(50, row_index))?;
                        let swimmer_entry_dead_line = read_datetime(_batch.at(51, row_index))?;
                        let swimmer_entry_open = read_datetime(_batch.at(52, row_index))?;
                        let allow_custom_times = read_bool(_batch.at(53, row_index))?;
                        let export_entries_date = read_datetime(_batch.at(54, row_index))?;
                        let last_sync_date = read_datetime(_batch.at(55, row_index))?;
                        let finalize_entries_date = read_datetime(_batch.at(56, row_index))?;
                        let use_swimmers_team = read_bool(_batch.at(57, row_index))?;
                        let honor_invite_list = read_bool(_batch.at(58, row_index))?;
                        let entry_team = read_u64(_batch.at(59, row_index))?;
                        let e_mail_from = read_string(_batch.at(60, row_index))?;
                        let e_mail_text = read_string(_batch.at(61, row_index))?;
                        let e_mail_subject = read_string(_batch.at(62, row_index))?;
                        let only_pre_entered = read_bool(_batch.at(63, row_index))?;
                        let license = read_string(_batch.at(64, row_index))?;
                        let collect_fees_online = read_bool(_batch.at(65, row_index))?;
                        let o_m_e_bill_date = read_datetime(_batch.at(66, row_index))?;
                        let invite_email_date = read_datetime(_batch.at(67, row_index))?;
                        let entry_e_mail_text = read_string(_batch.at(68, row_index))?;
                        let entry_e_mail_subject = read_string(_batch.at(69, row_index))?;
                        let entry_e_mail_from = read_string(_batch.at(70, row_index))?;
                        let o_m_e_validated = read_bool(_batch.at(71, row_index))?;
                        let o_m_e_custom_team = read_bool(_batch.at(72, row_index))?;
                        let o_m_e_filter_team = read_u64(_batch.at(73, row_index))?;
                        let o_m_e_needs_sync = read_bool(_batch.at(74, row_index))?;
                        let o_m_e_need_sync = read_bool(_batch.at(75, row_index))?;
                        let meet_sharing_status = read_string(_batch.at(76, row_index))?;
                        let file_export_type = read_u16(_batch.at(77, row_index))?;
                        let meet_registration_opens = read_datetime(_batch.at(78, row_index))?;
                        let meet_registration_closes = read_datetime(_batch.at(79, row_index))?;
                        let meet_sharing_meet_i_d = read_u64(_batch.at(80, row_index))?;
                        let meet_sharing_pay_status = read_string(_batch.at(81, row_index))?;
                        let meet_sharing_result_date = read_datetime(_batch.at(82, row_index))?;
                        let e_v3_version = read_u8(_batch.at(83, row_index))?;
                        let obj = Meet {
                            meet,
                            m_name,
                            start,
                            end,
                            age_up,
                            since,
                            course,
                            location,
                            remarks,
                            ind_charge,
                            rel_charge,
                            sur_charge,
                            ttype,
                            sanction,
                            max_ind_ent,
                            max_rel_ent,
                            max_ent,
                            restrict_best,
                            non_conform,
                            enter_at_q_time,
                            facility_fee,
                            team_fee,
                            instructions,
                            min_age,
                            enforce_qualifying,
                            altitude,
                            enforce_slow_qtime,
                            ban_no_times,
                            lanes,
                            even_or_odd,
                            fast_to_slow,
                            masters,
                            active_fee_x_m_l_sent,
                            min_age1_0_and_under,
                            seed_lanes,
                            dead_line,
                            active_meet_i_d,
                            cust_ind_charge,
                            cust_rel_charge,
                            cust_sur_charge,
                            cust_ind_action,
                            cust_rel_action,
                            cust_sur_action,
                            addr,
                            addr2,
                            city,
                            state,
                            ZIP,
                            cntry,
                            use_custom_fees,
                            o_m_e_entry_style,
                            swimmer_entry_dead_line,
                            swimmer_entry_open,
                            allow_custom_times,
                            export_entries_date,
                            last_sync_date,
                            finalize_entries_date,
                            use_swimmers_team,
                            honor_invite_list,
                            entry_team,
                            e_mail_from,
                            e_mail_text,
                            e_mail_subject,
                            only_pre_entered,
                            license,
                            collect_fees_online,
                            o_m_e_bill_date,
                            invite_email_date,
                            entry_e_mail_text,
                            entry_e_mail_subject,
                            entry_e_mail_from,
                            o_m_e_validated,
                            o_m_e_custom_team,
                            o_m_e_filter_team,
                            o_m_e_needs_sync,
                            o_m_e_need_sync,
                            meet_sharing_status,
                            file_export_type,
                            meet_registration_opens,
                            meet_registration_closes,
                            meet_sharing_meet_i_d,
                            meet_sharing_pay_status,
                            meet_sharing_result_date,
                            e_v3_version
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
