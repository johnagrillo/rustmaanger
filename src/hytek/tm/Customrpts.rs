// 
// 
// CREATE TABLE [CUSTOMRPTS]
// (
// [Name] Text (60),
// [Type] Integer,
// [Team] Long Integer,
// [Group] Text (6),
// [SubGr] Text (6),
// [Class] Text (6),
// [SortBy] Integer,
// [Sex] Integer,
// [Prelim] Integer,
// [Splits] Integer,
// [Stroke] Integer,
// [Distance] Integer,
// [I_R] Integer,
// [LSC] Text (6),
// [LoAge] Integer,
// [HiAge] Integer,
// [chkIncRelayLO] Boolean NOT NULL,
// [chkStandards] Boolean NOT NULL,
// [chkInactive] Boolean NOT NULL,
// [chkSince] Boolean NOT NULL,
// [SinceDate] DateTime,
// [chkImprove] Integer,
// [chkUntil] Boolean NOT NULL,
// [UntilDate] DateTime,
// [chkRecords] Boolean NOT NULL,
// [RECORD] Long Integer,
// [chkNamesOnly] Boolean NOT NULL,
// [chkBlankLine] Boolean NOT NULL,
// [Grouping] Integer,
// [AgeGroups] Integer,
// [MeetType] Text (6),
// [TopN] Integer,
// [StanAtLeast] Text (8),
// [Course] Integer,
// [chkChronological] Boolean NOT NULL,
// [Orientation] Integer,
// [chkActual] Boolean NOT NULL,
// [chkNoTimes] Boolean NOT NULL,
// [chkOnlyElig] Boolean NOT NULL,
// [chkBest] Integer,
// [RelaySwimmers] Integer,
// [StdName] Text (16),
// [Designator] Text (8),
// [chkStrokeRate] Boolean NOT NULL,
// [LabelIndex] Integer,
// [chkNISCA] Boolean NOT NULL,
// [chkNewStd] Boolean NOT NULL,
// [ColIndex] Integer,
// [chkGoals] Boolean NOT NULL,
// [chkIncRelayLegs] Boolean NOT NULL,
// [chkShowRegistration] Boolean NOT NULL,
// [chkNT] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Customrpts {
    name : Option<String>,
    ttype : Option<u16>,
    team : Option<u64>,
    group : Option<String>,
    sub_gr : Option<String>,
    cclass : Option<String>,
    sort_by : Option<u16>,
    sex : Option<u16>,
    prelim : Option<u16>,
    splits : Option<u16>,
    stroke : Option<u16>,
    distance : Option<u16>,
    I_R : Option<u16>,
    LSC : Option<String>,
    lo_age : Option<u16>,
    hi_age : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_inc_relay_l_o : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_standards : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_inactive : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_since : bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    since_date : Option<NaiveDateTime>,
    chk_improve : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_until : bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    until_date : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_records : bool,
    RECORD : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_names_only : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_blank_line : bool,
    grouping : Option<u16>,
    age_groups : Option<u16>,
    meet_type : Option<String>,
    top_n : Option<u16>,
    stan_at_least : Option<String>,
    course : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_chronological : bool,
    orientation : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_actual : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_no_times : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_only_elig : bool,
    chk_best : Option<u16>,
    relay_swimmers : Option<u16>,
    std_name : Option<String>,
    designator : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_stroke_rate : bool,
    label_index : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_n_i_s_c_a : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_new_std : bool,
    col_index : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_goals : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_inc_relay_legs : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_show_registration : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    chk_n_t : bool
}
impl Customrpts {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Customrpts>, Box<dyn Error>> {
       let mut vec: Vec<Customrpts> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Customrpts = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Customrpts>, Box<dyn Error>> {
        let mut vec: Vec<Customrpts> = Vec::new();
        match conn.execute("SELECT * FROM CUSTOMRPTS", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let name = read_string(_batch.at(0, row_index))?;
                        let ttype = read_u16(_batch.at(1, row_index))?;
                        let team = read_u64(_batch.at(2, row_index))?;
                        let group = read_string(_batch.at(3, row_index))?;
                        let sub_gr = read_string(_batch.at(4, row_index))?;
                        let cclass = read_string(_batch.at(5, row_index))?;
                        let sort_by = read_u16(_batch.at(6, row_index))?;
                        let sex = read_u16(_batch.at(7, row_index))?;
                        let prelim = read_u16(_batch.at(8, row_index))?;
                        let splits = read_u16(_batch.at(9, row_index))?;
                        let stroke = read_u16(_batch.at(10, row_index))?;
                        let distance = read_u16(_batch.at(11, row_index))?;
                        let I_R = read_u16(_batch.at(12, row_index))?;
                        let LSC = read_string(_batch.at(13, row_index))?;
                        let lo_age = read_u16(_batch.at(14, row_index))?;
                        let hi_age = read_u16(_batch.at(15, row_index))?;
                        let chk_inc_relay_l_o = read_bool(_batch.at(16, row_index))?;
                        let chk_standards = read_bool(_batch.at(17, row_index))?;
                        let chk_inactive = read_bool(_batch.at(18, row_index))?;
                        let chk_since = read_bool(_batch.at(19, row_index))?;
                        let since_date = read_datetime(_batch.at(20, row_index))?;
                        let chk_improve = read_u16(_batch.at(21, row_index))?;
                        let chk_until = read_bool(_batch.at(22, row_index))?;
                        let until_date = read_datetime(_batch.at(23, row_index))?;
                        let chk_records = read_bool(_batch.at(24, row_index))?;
                        let RECORD = read_u64(_batch.at(25, row_index))?;
                        let chk_names_only = read_bool(_batch.at(26, row_index))?;
                        let chk_blank_line = read_bool(_batch.at(27, row_index))?;
                        let grouping = read_u16(_batch.at(28, row_index))?;
                        let age_groups = read_u16(_batch.at(29, row_index))?;
                        let meet_type = read_string(_batch.at(30, row_index))?;
                        let top_n = read_u16(_batch.at(31, row_index))?;
                        let stan_at_least = read_string(_batch.at(32, row_index))?;
                        let course = read_u16(_batch.at(33, row_index))?;
                        let chk_chronological = read_bool(_batch.at(34, row_index))?;
                        let orientation = read_u16(_batch.at(35, row_index))?;
                        let chk_actual = read_bool(_batch.at(36, row_index))?;
                        let chk_no_times = read_bool(_batch.at(37, row_index))?;
                        let chk_only_elig = read_bool(_batch.at(38, row_index))?;
                        let chk_best = read_u16(_batch.at(39, row_index))?;
                        let relay_swimmers = read_u16(_batch.at(40, row_index))?;
                        let std_name = read_string(_batch.at(41, row_index))?;
                        let designator = read_string(_batch.at(42, row_index))?;
                        let chk_stroke_rate = read_bool(_batch.at(43, row_index))?;
                        let label_index = read_u16(_batch.at(44, row_index))?;
                        let chk_n_i_s_c_a = read_bool(_batch.at(45, row_index))?;
                        let chk_new_std = read_bool(_batch.at(46, row_index))?;
                        let col_index = read_u16(_batch.at(47, row_index))?;
                        let chk_goals = read_bool(_batch.at(48, row_index))?;
                        let chk_inc_relay_legs = read_bool(_batch.at(49, row_index))?;
                        let chk_show_registration = read_bool(_batch.at(50, row_index))?;
                        let chk_n_t = read_bool(_batch.at(51, row_index))?;
                        let obj = Customrpts {
                            name,
                            ttype,
                            team,
                            group,
                            sub_gr,
                            cclass,
                            sort_by,
                            sex,
                            prelim,
                            splits,
                            stroke,
                            distance,
                            I_R,
                            LSC,
                            lo_age,
                            hi_age,
                            chk_inc_relay_l_o,
                            chk_standards,
                            chk_inactive,
                            chk_since,
                            since_date,
                            chk_improve,
                            chk_until,
                            until_date,
                            chk_records,
                            RECORD,
                            chk_names_only,
                            chk_blank_line,
                            grouping,
                            age_groups,
                            meet_type,
                            top_n,
                            stan_at_least,
                            course,
                            chk_chronological,
                            orientation,
                            chk_actual,
                            chk_no_times,
                            chk_only_elig,
                            chk_best,
                            relay_swimmers,
                            std_name,
                            designator,
                            chk_stroke_rate,
                            label_index,
                            chk_n_i_s_c_a,
                            chk_new_std,
                            col_index,
                            chk_goals,
                            chk_inc_relay_legs,
                            chk_show_registration,
                            chk_n_t
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
