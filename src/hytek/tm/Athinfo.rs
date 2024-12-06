// 
// 
// CREATE TABLE [AthInfo]
// (
// [Athlete] Long Integer,
// [Addr] Text (60),
// [City] Text (60),
// [ZIP] Text (20),
// [Cntry] Text (40),
// [DayP1] Text (40),
// [EveP1] Text (40),
// [FaxP1] Text (40),
// [DayP2] Text (40),
// [EveP2] Text (40),
// [FaxP2] Text (40),
// [MailTo] Text (80),
// [Seasonal] Text (2),
// [FinaFed] Boolean NOT NULL,
// [RegDate] DateTime,
// [NRCD] Text (2),
// [EMail] Text (100),
// [Member] Boolean NOT NULL,
// [JrHS] Boolean NOT NULL,
// [HS] Boolean NOT NULL,
// [YMCA] Boolean NOT NULL,
// [Coll] Boolean NOT NULL,
// [SL] Boolean NOT NULL,
// [DUAL] Boolean NOT NULL,
// [Mast] Boolean NOT NULL,
// [Dis] Boolean NOT NULL,
// [Polo] Boolean NOT NULL,
// [Ethnic] Byte,
// [MedCondition] Text (240),
// [Medication] Text (240),
// [DoctorName] Text (60),
// [DoctorPhone] Text (40),
// [EmerContact] Text (60),
// [EmerPhone] Text (40),
// [MailTo2] Text (80),
// [Addr2] Text (60),
// [City2] Text (60),
// [State2] Text (6),
// [ZIP2] Text (20),
// [Cntry2] Text (40),
// [EMail2] Text (100),
// [UseMailTo] Boolean NOT NULL,
// [CusPrompt1] Text (40),
// [CusValue1] Text (40),
// [CusPrompt2] Text (40),
// [CusValue2] Text (40),
// [CusPrompt3] Text (40),
// [CusValue3] Text (40),
// [State] Text (6),
// [BLIND] Boolean NOT NULL,
// [DEAF] Boolean NOT NULL,
// [PHYSICAL] Boolean NOT NULL,
// [COGNITIVE] Boolean NOT NULL,
// [SecAddr] Text (60),
// [SecAddr2] Text (60),
// [PrimaryLast] Text (40),
// [PrimaryFirst1] Text (40),
// [PrimaryFirst2] Text (40),
// [SecondaryLast] Text (40),
// [SecondaryFirst1] Text (40),
// [SecondaryFirst2] Text (40),
// [CellP1] Text (40),
// [CellP2] Text (40),
// [UseBoth] Boolean NOT NULL,
// [EveP1P2] Text (40),
// [CellP1P2] Text (40),
// [EMail1P2] Text (100),
// [EveP2P2] Text (40),
// [CellP2P2] Text (40),
// [EMail2P2] Text (100),
// [CellAthlete] Text (40),
// [EMailAthlete] Text (100),
// [Middle] Text (40),
// [USSDonation] Boolean NOT NULL,
// [PrimaryLast2] Text (40),
// [FinaFedCntry] Text (6),
// [ExpAthEmail] Boolean NOT NULL,
// [ExpFatherEmail] Boolean NOT NULL,
// [ExpMotherEmail] Boolean NOT NULL,
// [ExpParent1Email] Boolean NOT NULL,
// [ExpParent2Email] Boolean NOT NULL,
// [RegEMail] Text (100),
// [USSNewsLetter] Boolean NOT NULL,
// [CusPrompt4] Text (40),
// [CusValue4] Text (40),
// [CusPrompt5] Text (40),
// [CusValue5] Text (40),
// [CusPrompt6] Text (40),
// [CusValue6] Text (40),
// [CusPrompt7] Text (40),
// [CusValue7] Text (40),
// [CusPrompt8] Text (40),
// [CusValue8] Text (40),
// [HSGradYear] Integer,
// [FINACompeted] Boolean NOT NULL,
// [CustUSSFee] Text (20)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Athinfo {
    athlete : Option<u64>,
    addr : Option<String>,
    city : Option<String>,
    ZIP : Option<String>,
    cntry : Option<String>,
    day_p1 : Option<String>,
    eve_p1 : Option<String>,
    fax_p1 : Option<String>,
    day_p2 : Option<String>,
    eve_p2 : Option<String>,
    fax_p2 : Option<String>,
    mail_to : Option<String>,
    seasonal : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    fina_fed : bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    reg_date : Option<NaiveDateTime>,
    NRCD : Option<String>,
    e_mail : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    member : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    jr_h_s : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    HS : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    YMCA : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    coll : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    SL : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    DUAL : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    mast : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    dis : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    polo : bool,
    ethnic : Option<u8>,
    med_condition : Option<String>,
    medication : Option<String>,
    doctor_name : Option<String>,
    doctor_phone : Option<String>,
    emer_contact : Option<String>,
    emer_phone : Option<String>,
    mail_to2 : Option<String>,
    addr2 : Option<String>,
    city2 : Option<String>,
    state2 : Option<String>,
    ZIP2 : Option<String>,
    cntry2 : Option<String>,
    e_mail2 : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_mail_to : bool,
    cus_prompt1 : Option<String>,
    cus_value1 : Option<String>,
    cus_prompt2 : Option<String>,
    cus_value2 : Option<String>,
    cus_prompt3 : Option<String>,
    cus_value3 : Option<String>,
    state : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    BLIND : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    DEAF : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    PHYSICAL : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    COGNITIVE : bool,
    sec_addr : Option<String>,
    sec_addr2 : Option<String>,
    primary_last : Option<String>,
    primary_first1 : Option<String>,
    primary_first2 : Option<String>,
    secondary_last : Option<String>,
    secondary_first1 : Option<String>,
    secondary_first2 : Option<String>,
    cell_p1 : Option<String>,
    cell_p2 : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_both : bool,
    eve_p1_p2 : Option<String>,
    cell_p1_p2 : Option<String>,
    e_mail1_p2 : Option<String>,
    eve_p2_p2 : Option<String>,
    cell_p2_p2 : Option<String>,
    e_mail2_p2 : Option<String>,
    cell_athlete : Option<String>,
    e_mail_athlete : Option<String>,
    middle : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    u_s_s_donation : bool,
    primary_last2 : Option<String>,
    fina_fed_cntry : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    exp_ath_email : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    exp_father_email : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    exp_mother_email : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    exp_parent1_email : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    exp_parent2_email : bool,
    reg_e_mail : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    u_s_s_news_letter : bool,
    cus_prompt4 : Option<String>,
    cus_value4 : Option<String>,
    cus_prompt5 : Option<String>,
    cus_value5 : Option<String>,
    cus_prompt6 : Option<String>,
    cus_value6 : Option<String>,
    cus_prompt7 : Option<String>,
    cus_value7 : Option<String>,
    cus_prompt8 : Option<String>,
    cus_value8 : Option<String>,
    h_s_grad_year : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    f_i_n_a_competed : bool,
    cust_u_s_s_fee : Option<String>
}
impl Athinfo {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Athinfo>, Box<dyn Error>> {
       let mut vec: Vec<Athinfo> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Athinfo = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Athinfo>, Box<dyn Error>> {
        let mut vec: Vec<Athinfo> = Vec::new();
        match conn.execute("SELECT * FROM ath info", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let athlete = read_u64(_batch.at(0, row_index))?;
                        let addr = read_string(_batch.at(1, row_index))?;
                        let city = read_string(_batch.at(2, row_index))?;
                        let ZIP = read_string(_batch.at(3, row_index))?;
                        let cntry = read_string(_batch.at(4, row_index))?;
                        let day_p1 = read_string(_batch.at(5, row_index))?;
                        let eve_p1 = read_string(_batch.at(6, row_index))?;
                        let fax_p1 = read_string(_batch.at(7, row_index))?;
                        let day_p2 = read_string(_batch.at(8, row_index))?;
                        let eve_p2 = read_string(_batch.at(9, row_index))?;
                        let fax_p2 = read_string(_batch.at(10, row_index))?;
                        let mail_to = read_string(_batch.at(11, row_index))?;
                        let seasonal = read_string(_batch.at(12, row_index))?;
                        let fina_fed = read_bool(_batch.at(13, row_index))?;
                        let reg_date = read_datetime(_batch.at(14, row_index))?;
                        let NRCD = read_string(_batch.at(15, row_index))?;
                        let e_mail = read_string(_batch.at(16, row_index))?;
                        let member = read_bool(_batch.at(17, row_index))?;
                        let jr_h_s = read_bool(_batch.at(18, row_index))?;
                        let HS = read_bool(_batch.at(19, row_index))?;
                        let YMCA = read_bool(_batch.at(20, row_index))?;
                        let coll = read_bool(_batch.at(21, row_index))?;
                        let SL = read_bool(_batch.at(22, row_index))?;
                        let DUAL = read_bool(_batch.at(23, row_index))?;
                        let mast = read_bool(_batch.at(24, row_index))?;
                        let dis = read_bool(_batch.at(25, row_index))?;
                        let polo = read_bool(_batch.at(26, row_index))?;
                        let ethnic = read_u8(_batch.at(27, row_index))?;
                        let med_condition = read_string(_batch.at(28, row_index))?;
                        let medication = read_string(_batch.at(29, row_index))?;
                        let doctor_name = read_string(_batch.at(30, row_index))?;
                        let doctor_phone = read_string(_batch.at(31, row_index))?;
                        let emer_contact = read_string(_batch.at(32, row_index))?;
                        let emer_phone = read_string(_batch.at(33, row_index))?;
                        let mail_to2 = read_string(_batch.at(34, row_index))?;
                        let addr2 = read_string(_batch.at(35, row_index))?;
                        let city2 = read_string(_batch.at(36, row_index))?;
                        let state2 = read_string(_batch.at(37, row_index))?;
                        let ZIP2 = read_string(_batch.at(38, row_index))?;
                        let cntry2 = read_string(_batch.at(39, row_index))?;
                        let e_mail2 = read_string(_batch.at(40, row_index))?;
                        let use_mail_to = read_bool(_batch.at(41, row_index))?;
                        let cus_prompt1 = read_string(_batch.at(42, row_index))?;
                        let cus_value1 = read_string(_batch.at(43, row_index))?;
                        let cus_prompt2 = read_string(_batch.at(44, row_index))?;
                        let cus_value2 = read_string(_batch.at(45, row_index))?;
                        let cus_prompt3 = read_string(_batch.at(46, row_index))?;
                        let cus_value3 = read_string(_batch.at(47, row_index))?;
                        let state = read_string(_batch.at(48, row_index))?;
                        let BLIND = read_bool(_batch.at(49, row_index))?;
                        let DEAF = read_bool(_batch.at(50, row_index))?;
                        let PHYSICAL = read_bool(_batch.at(51, row_index))?;
                        let COGNITIVE = read_bool(_batch.at(52, row_index))?;
                        let sec_addr = read_string(_batch.at(53, row_index))?;
                        let sec_addr2 = read_string(_batch.at(54, row_index))?;
                        let primary_last = read_string(_batch.at(55, row_index))?;
                        let primary_first1 = read_string(_batch.at(56, row_index))?;
                        let primary_first2 = read_string(_batch.at(57, row_index))?;
                        let secondary_last = read_string(_batch.at(58, row_index))?;
                        let secondary_first1 = read_string(_batch.at(59, row_index))?;
                        let secondary_first2 = read_string(_batch.at(60, row_index))?;
                        let cell_p1 = read_string(_batch.at(61, row_index))?;
                        let cell_p2 = read_string(_batch.at(62, row_index))?;
                        let use_both = read_bool(_batch.at(63, row_index))?;
                        let eve_p1_p2 = read_string(_batch.at(64, row_index))?;
                        let cell_p1_p2 = read_string(_batch.at(65, row_index))?;
                        let e_mail1_p2 = read_string(_batch.at(66, row_index))?;
                        let eve_p2_p2 = read_string(_batch.at(67, row_index))?;
                        let cell_p2_p2 = read_string(_batch.at(68, row_index))?;
                        let e_mail2_p2 = read_string(_batch.at(69, row_index))?;
                        let cell_athlete = read_string(_batch.at(70, row_index))?;
                        let e_mail_athlete = read_string(_batch.at(71, row_index))?;
                        let middle = read_string(_batch.at(72, row_index))?;
                        let u_s_s_donation = read_bool(_batch.at(73, row_index))?;
                        let primary_last2 = read_string(_batch.at(74, row_index))?;
                        let fina_fed_cntry = read_string(_batch.at(75, row_index))?;
                        let exp_ath_email = read_bool(_batch.at(76, row_index))?;
                        let exp_father_email = read_bool(_batch.at(77, row_index))?;
                        let exp_mother_email = read_bool(_batch.at(78, row_index))?;
                        let exp_parent1_email = read_bool(_batch.at(79, row_index))?;
                        let exp_parent2_email = read_bool(_batch.at(80, row_index))?;
                        let reg_e_mail = read_string(_batch.at(81, row_index))?;
                        let u_s_s_news_letter = read_bool(_batch.at(82, row_index))?;
                        let cus_prompt4 = read_string(_batch.at(83, row_index))?;
                        let cus_value4 = read_string(_batch.at(84, row_index))?;
                        let cus_prompt5 = read_string(_batch.at(85, row_index))?;
                        let cus_value5 = read_string(_batch.at(86, row_index))?;
                        let cus_prompt6 = read_string(_batch.at(87, row_index))?;
                        let cus_value6 = read_string(_batch.at(88, row_index))?;
                        let cus_prompt7 = read_string(_batch.at(89, row_index))?;
                        let cus_value7 = read_string(_batch.at(90, row_index))?;
                        let cus_prompt8 = read_string(_batch.at(91, row_index))?;
                        let cus_value8 = read_string(_batch.at(92, row_index))?;
                        let h_s_grad_year = read_u16(_batch.at(93, row_index))?;
                        let f_i_n_a_competed = read_bool(_batch.at(94, row_index))?;
                        let cust_u_s_s_fee = read_string(_batch.at(95, row_index))?;
                        let obj = Athinfo {
                            athlete,
                            addr,
                            city,
                            ZIP,
                            cntry,
                            day_p1,
                            eve_p1,
                            fax_p1,
                            day_p2,
                            eve_p2,
                            fax_p2,
                            mail_to,
                            seasonal,
                            fina_fed,
                            reg_date,
                            NRCD,
                            e_mail,
                            member,
                            jr_h_s,
                            HS,
                            YMCA,
                            coll,
                            SL,
                            DUAL,
                            mast,
                            dis,
                            polo,
                            ethnic,
                            med_condition,
                            medication,
                            doctor_name,
                            doctor_phone,
                            emer_contact,
                            emer_phone,
                            mail_to2,
                            addr2,
                            city2,
                            state2,
                            ZIP2,
                            cntry2,
                            e_mail2,
                            use_mail_to,
                            cus_prompt1,
                            cus_value1,
                            cus_prompt2,
                            cus_value2,
                            cus_prompt3,
                            cus_value3,
                            state,
                            BLIND,
                            DEAF,
                            PHYSICAL,
                            COGNITIVE,
                            sec_addr,
                            sec_addr2,
                            primary_last,
                            primary_first1,
                            primary_first2,
                            secondary_last,
                            secondary_first1,
                            secondary_first2,
                            cell_p1,
                            cell_p2,
                            use_both,
                            eve_p1_p2,
                            cell_p1_p2,
                            e_mail1_p2,
                            eve_p2_p2,
                            cell_p2_p2,
                            e_mail2_p2,
                            cell_athlete,
                            e_mail_athlete,
                            middle,
                            u_s_s_donation,
                            primary_last2,
                            fina_fed_cntry,
                            exp_ath_email,
                            exp_father_email,
                            exp_mother_email,
                            exp_parent1_email,
                            exp_parent2_email,
                            reg_e_mail,
                            u_s_s_news_letter,
                            cus_prompt4,
                            cus_value4,
                            cus_prompt5,
                            cus_value5,
                            cus_prompt6,
                            cus_value6,
                            cus_prompt7,
                            cus_value7,
                            cus_prompt8,
                            cus_value8,
                            h_s_grad_year,
                            f_i_n_a_competed,
                            cust_u_s_s_fee
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
