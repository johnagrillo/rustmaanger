// 
// 
// CREATE TABLE [Relay]
// (
// [Event_ptr] Long Integer,
// [Team_no] Long Integer,
// [Team_ltr] Text (2),
// [Rel_age] Integer,
// [Rel_sex] Text (2),
// [ActSeed_course] Text (2),
// [ActualSeed_time] Single,
// [ConvSeed_course] Text (2),
// [ConvSeed_time] Single,
// [Scr_stat] Boolean NOT NULL,
// [Spec_stat] Text (2),
// [Dec_stat] Text (2),
// [Alt_stat] Boolean NOT NULL,
// [Bonus_event] Boolean NOT NULL,
// [Div_no] Long Integer,
// [Ev_score] Single,
// [dq_type] Text (2),
// [Pre_heat] Integer,
// [Pre_lane] Integer,
// [Pre_stat] Text (2),
// [Pre_Time] Single,
// [Pre_course] Text (2),
// [Pre_heatplace] Integer,
// [Pre_place] Integer,
// [Pre_jdplace] Integer,
// [Pre_exh] Text (2),
// [Pre_points] Integer,
// [Pre_back1] Single,
// [Pre_back2] Single,
// [Pre_back3] Single,
// [Fin_heat] Integer,
// [Fin_lane] Integer,
// [Fin_group] Integer,
// [Fin_stat] Text (2),
// [Fin_Time] Single,
// [Fin_course] Text (2),
// [Fin_heatplace] Integer,
// [Fin_jdheatplace] Integer,
// [Fin_place] Integer,
// [Fin_jdplace] Integer,
// [Fin_ptsplace] Integer,
// [Fin_exh] Text (2),
// [Fin_points] Integer,
// [Fin_back1] Single,
// [Fin_back2] Single,
// [Fin_back3] Single,
// [Sem_heat] Integer,
// [Sem_lane] Integer,
// [Sem_stat] Text (2),
// [Sem_Time] Single,
// [Sem_course] Text (2),
// [Sem_heatplace] Integer,
// [Sem_place] Integer,
// [Sem_jdplace] Integer,
// [Sem_exh] Text (2),
// [Sem_points] Integer,
// [Sem_back1] Single,
// [Sem_back2] Single,
// [Sem_back3] Single,
// [Swimoff_heat] Integer,
// [Swimoff_lane] Integer,
// [Swimoff_stat] Text (2),
// [Swimoff_Time] Single,
// [Swimoff_course] Text (2),
// [Swimoff_heatplace] Integer,
// [Swimoff_place] Integer,
// [Swimoff_jdplace] Integer,
// [Swimoff_points] Integer,
// [Swimoff_back1] Single,
// [Swimoff_back2] Single,
// [Swimoff_back3] Single,
// [JDEv_score] Single,
// [Relay_no] Long Integer,
// [Seed_place] Integer,
// [fin_heatltr] Text (2),
// [Pre_watch1] Single,
// [Pre_pad] Single,
// [Sem_watch1] Single,
// [Sem_pad] Single,
// [Fin_watch1] Single,
// [Fin_pad] Single,
// [Fin_reactiontime1] Text (10),
// [Pre_reactiontime1] Text (10),
// [Sem_reactiontime1] Text (10),
// [Fin_dqcode] Text (4),
// [Pre_dqcode] Text (4),
// [Sem_dqcode] Text (4),
// [Fin_dqcodeSecond] Text (4),
// [Pre_dqcodeSecond] Text (4),
// [Sem_dqcodeSecond] Text (4),
// [Fin_reactiontime2] Text (10),
// [Pre_reactiontime2] Text (10),
// [Sem_reactiontime2] Text (10),
// [Fin_reactiontime3] Text (10),
// [Pre_reactiontime3] Text (10),
// [Sem_reactiontime3] Text (10),
// [Fin_reactiontime4] Text (10),
// [Pre_reactiontime4] Text (10),
// [Sem_reactiontime4] Text (10),
// [Fin_TimeType] Text (2),
// [Pre_TimeType] Text (2),
// [Sem_TimeType] Text (2),
// [Fin_dolphin1] Single,
// [Fin_dolphin2] Single,
// [Fin_dolphin3] Single,
// [Pre_dolphin1] Single,
// [Pre_dolphin2] Single,
// [Pre_dolphin3] Single,
// [Sem_dolphin1] Single,
// [Sem_dolphin2] Single,
// [Sem_dolphin3] Single,
// [early_seed] Boolean NOT NULL,
// [fin_adjuststat] Text (2),
// [pre_adjuststat] Text (2),
// [sem_adjuststat] Text (2),
// [entry_method] Text (2),
// [fin_dqofficial] Long Integer,
// [pre_dqofficial] Long Integer,
// [sem_dqofficial] Long Integer,
// [pre_contacted] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Relay {
    event_ptr : Option<u64>,
    team_no : Option<u64>,
    team_ltr : Option<String>,
    rel_age : Option<u16>,
    rel_sex : Option<String>,
    act_seed_course : Option<String>,
    actual_seed_time : Option<f32>,
    conv_seed_course : Option<String>,
    conv_seed_time : Option<f32>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    scr_stat : bool,
    spec_stat : Option<String>,
    dec_stat : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    alt_stat : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    bonus_event : bool,
    div_no : Option<u64>,
    ev_score : Option<f32>,
    dq_type : Option<String>,
    pre_heat : Option<u16>,
    pre_lane : Option<u16>,
    pre_stat : Option<String>,
    pre__time : Option<f32>,
    pre_course : Option<String>,
    pre_heatplace : Option<u16>,
    pre_place : Option<u16>,
    pre_jdplace : Option<u16>,
    pre_exh : Option<String>,
    pre_points : Option<u16>,
    pre_back1 : Option<f32>,
    pre_back2 : Option<f32>,
    pre_back3 : Option<f32>,
    fin_heat : Option<u16>,
    fin_lane : Option<u16>,
    fin_group : Option<u16>,
    fin_stat : Option<String>,
    fin__time : Option<f32>,
    fin_course : Option<String>,
    fin_heatplace : Option<u16>,
    fin_jdheatplace : Option<u16>,
    fin_place : Option<u16>,
    fin_jdplace : Option<u16>,
    fin_ptsplace : Option<u16>,
    fin_exh : Option<String>,
    fin_points : Option<u16>,
    fin_back1 : Option<f32>,
    fin_back2 : Option<f32>,
    fin_back3 : Option<f32>,
    sem_heat : Option<u16>,
    sem_lane : Option<u16>,
    sem_stat : Option<String>,
    sem__time : Option<f32>,
    sem_course : Option<String>,
    sem_heatplace : Option<u16>,
    sem_place : Option<u16>,
    sem_jdplace : Option<u16>,
    sem_exh : Option<String>,
    sem_points : Option<u16>,
    sem_back1 : Option<f32>,
    sem_back2 : Option<f32>,
    sem_back3 : Option<f32>,
    swimoff_heat : Option<u16>,
    swimoff_lane : Option<u16>,
    swimoff_stat : Option<String>,
    swimoff__time : Option<f32>,
    swimoff_course : Option<String>,
    swimoff_heatplace : Option<u16>,
    swimoff_place : Option<u16>,
    swimoff_jdplace : Option<u16>,
    swimoff_points : Option<u16>,
    swimoff_back1 : Option<f32>,
    swimoff_back2 : Option<f32>,
    swimoff_back3 : Option<f32>,
    j_d_ev_score : Option<f32>,
    relay_no : Option<u64>,
    seed_place : Option<u16>,
    fin_heatltr : Option<String>,
    pre_watch1 : Option<f32>,
    pre_pad : Option<f32>,
    sem_watch1 : Option<f32>,
    sem_pad : Option<f32>,
    fin_watch1 : Option<f32>,
    fin_pad : Option<f32>,
    fin_reactiontime1 : Option<String>,
    pre_reactiontime1 : Option<String>,
    sem_reactiontime1 : Option<String>,
    fin_dqcode : Option<String>,
    pre_dqcode : Option<String>,
    sem_dqcode : Option<String>,
    fin_dqcode_second : Option<String>,
    pre_dqcode_second : Option<String>,
    sem_dqcode_second : Option<String>,
    fin_reactiontime2 : Option<String>,
    pre_reactiontime2 : Option<String>,
    sem_reactiontime2 : Option<String>,
    fin_reactiontime3 : Option<String>,
    pre_reactiontime3 : Option<String>,
    sem_reactiontime3 : Option<String>,
    fin_reactiontime4 : Option<String>,
    pre_reactiontime4 : Option<String>,
    sem_reactiontime4 : Option<String>,
    fin__time_type : Option<String>,
    pre__time_type : Option<String>,
    sem__time_type : Option<String>,
    fin_dolphin1 : Option<f32>,
    fin_dolphin2 : Option<f32>,
    fin_dolphin3 : Option<f32>,
    pre_dolphin1 : Option<f32>,
    pre_dolphin2 : Option<f32>,
    pre_dolphin3 : Option<f32>,
    sem_dolphin1 : Option<f32>,
    sem_dolphin2 : Option<f32>,
    sem_dolphin3 : Option<f32>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    early_seed : bool,
    fin_adjuststat : Option<String>,
    pre_adjuststat : Option<String>,
    sem_adjuststat : Option<String>,
    entry_method : Option<String>,
    fin_dqofficial : Option<u64>,
    pre_dqofficial : Option<u64>,
    sem_dqofficial : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    pre_contacted : bool
}
impl Relay {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Relay>, Box<dyn Error>> {
       let mut vec: Vec<Relay> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Relay = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Relay>, Error> {
        let mut vec: Vec<Relay> = Vec::new();
        match conn.execute("SELECT * FROM relay", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let event_ptr = read_u64(_batch.at(0, row_index));
                        let team_no = read_u64(_batch.at(1, row_index));
                        let team_ltr = read_string(_batch.at(2, row_index));
                        let rel_age = read_u16(_batch.at(3, row_index));
                        let rel_sex = read_string(_batch.at(4, row_index));
                        let act_seed_course = read_string(_batch.at(5, row_index));
                        let actual_seed_time = read_f32(_batch.at(6, row_index));
                        let conv_seed_course = read_string(_batch.at(7, row_index));
                        let conv_seed_time = read_f32(_batch.at(8, row_index));
                        let scr_stat = read_bool(_batch.at(9, row_index));
                        let spec_stat = read_string(_batch.at(10, row_index));
                        let dec_stat = read_string(_batch.at(11, row_index));
                        let alt_stat = read_bool(_batch.at(12, row_index));
                        let bonus_event = read_bool(_batch.at(13, row_index));
                        let div_no = read_u64(_batch.at(14, row_index));
                        let ev_score = read_f32(_batch.at(15, row_index));
                        let dq_type = read_string(_batch.at(16, row_index));
                        let pre_heat = read_u16(_batch.at(17, row_index));
                        let pre_lane = read_u16(_batch.at(18, row_index));
                        let pre_stat = read_string(_batch.at(19, row_index));
                        let pre__time = read_f32(_batch.at(20, row_index));
                        let pre_course = read_string(_batch.at(21, row_index));
                        let pre_heatplace = read_u16(_batch.at(22, row_index));
                        let pre_place = read_u16(_batch.at(23, row_index));
                        let pre_jdplace = read_u16(_batch.at(24, row_index));
                        let pre_exh = read_string(_batch.at(25, row_index));
                        let pre_points = read_u16(_batch.at(26, row_index));
                        let pre_back1 = read_f32(_batch.at(27, row_index));
                        let pre_back2 = read_f32(_batch.at(28, row_index));
                        let pre_back3 = read_f32(_batch.at(29, row_index));
                        let fin_heat = read_u16(_batch.at(30, row_index));
                        let fin_lane = read_u16(_batch.at(31, row_index));
                        let fin_group = read_u16(_batch.at(32, row_index));
                        let fin_stat = read_string(_batch.at(33, row_index));
                        let fin__time = read_f32(_batch.at(34, row_index));
                        let fin_course = read_string(_batch.at(35, row_index));
                        let fin_heatplace = read_u16(_batch.at(36, row_index));
                        let fin_jdheatplace = read_u16(_batch.at(37, row_index));
                        let fin_place = read_u16(_batch.at(38, row_index));
                        let fin_jdplace = read_u16(_batch.at(39, row_index));
                        let fin_ptsplace = read_u16(_batch.at(40, row_index));
                        let fin_exh = read_string(_batch.at(41, row_index));
                        let fin_points = read_u16(_batch.at(42, row_index));
                        let fin_back1 = read_f32(_batch.at(43, row_index));
                        let fin_back2 = read_f32(_batch.at(44, row_index));
                        let fin_back3 = read_f32(_batch.at(45, row_index));
                        let sem_heat = read_u16(_batch.at(46, row_index));
                        let sem_lane = read_u16(_batch.at(47, row_index));
                        let sem_stat = read_string(_batch.at(48, row_index));
                        let sem__time = read_f32(_batch.at(49, row_index));
                        let sem_course = read_string(_batch.at(50, row_index));
                        let sem_heatplace = read_u16(_batch.at(51, row_index));
                        let sem_place = read_u16(_batch.at(52, row_index));
                        let sem_jdplace = read_u16(_batch.at(53, row_index));
                        let sem_exh = read_string(_batch.at(54, row_index));
                        let sem_points = read_u16(_batch.at(55, row_index));
                        let sem_back1 = read_f32(_batch.at(56, row_index));
                        let sem_back2 = read_f32(_batch.at(57, row_index));
                        let sem_back3 = read_f32(_batch.at(58, row_index));
                        let swimoff_heat = read_u16(_batch.at(59, row_index));
                        let swimoff_lane = read_u16(_batch.at(60, row_index));
                        let swimoff_stat = read_string(_batch.at(61, row_index));
                        let swimoff__time = read_f32(_batch.at(62, row_index));
                        let swimoff_course = read_string(_batch.at(63, row_index));
                        let swimoff_heatplace = read_u16(_batch.at(64, row_index));
                        let swimoff_place = read_u16(_batch.at(65, row_index));
                        let swimoff_jdplace = read_u16(_batch.at(66, row_index));
                        let swimoff_points = read_u16(_batch.at(67, row_index));
                        let swimoff_back1 = read_f32(_batch.at(68, row_index));
                        let swimoff_back2 = read_f32(_batch.at(69, row_index));
                        let swimoff_back3 = read_f32(_batch.at(70, row_index));
                        let j_d_ev_score = read_f32(_batch.at(71, row_index));
                        let relay_no = read_u64(_batch.at(72, row_index));
                        let seed_place = read_u16(_batch.at(73, row_index));
                        let fin_heatltr = read_string(_batch.at(74, row_index));
                        let pre_watch1 = read_f32(_batch.at(75, row_index));
                        let pre_pad = read_f32(_batch.at(76, row_index));
                        let sem_watch1 = read_f32(_batch.at(77, row_index));
                        let sem_pad = read_f32(_batch.at(78, row_index));
                        let fin_watch1 = read_f32(_batch.at(79, row_index));
                        let fin_pad = read_f32(_batch.at(80, row_index));
                        let fin_reactiontime1 = read_string(_batch.at(81, row_index));
                        let pre_reactiontime1 = read_string(_batch.at(82, row_index));
                        let sem_reactiontime1 = read_string(_batch.at(83, row_index));
                        let fin_dqcode = read_string(_batch.at(84, row_index));
                        let pre_dqcode = read_string(_batch.at(85, row_index));
                        let sem_dqcode = read_string(_batch.at(86, row_index));
                        let fin_dqcode_second = read_string(_batch.at(87, row_index));
                        let pre_dqcode_second = read_string(_batch.at(88, row_index));
                        let sem_dqcode_second = read_string(_batch.at(89, row_index));
                        let fin_reactiontime2 = read_string(_batch.at(90, row_index));
                        let pre_reactiontime2 = read_string(_batch.at(91, row_index));
                        let sem_reactiontime2 = read_string(_batch.at(92, row_index));
                        let fin_reactiontime3 = read_string(_batch.at(93, row_index));
                        let pre_reactiontime3 = read_string(_batch.at(94, row_index));
                        let sem_reactiontime3 = read_string(_batch.at(95, row_index));
                        let fin_reactiontime4 = read_string(_batch.at(96, row_index));
                        let pre_reactiontime4 = read_string(_batch.at(97, row_index));
                        let sem_reactiontime4 = read_string(_batch.at(98, row_index));
                        let fin__time_type = read_string(_batch.at(99, row_index));
                        let pre__time_type = read_string(_batch.at(100, row_index));
                        let sem__time_type = read_string(_batch.at(101, row_index));
                        let fin_dolphin1 = read_f32(_batch.at(102, row_index));
                        let fin_dolphin2 = read_f32(_batch.at(103, row_index));
                        let fin_dolphin3 = read_f32(_batch.at(104, row_index));
                        let pre_dolphin1 = read_f32(_batch.at(105, row_index));
                        let pre_dolphin2 = read_f32(_batch.at(106, row_index));
                        let pre_dolphin3 = read_f32(_batch.at(107, row_index));
                        let sem_dolphin1 = read_f32(_batch.at(108, row_index));
                        let sem_dolphin2 = read_f32(_batch.at(109, row_index));
                        let sem_dolphin3 = read_f32(_batch.at(110, row_index));
                        let early_seed = read_bool(_batch.at(111, row_index));
                        let fin_adjuststat = read_string(_batch.at(112, row_index));
                        let pre_adjuststat = read_string(_batch.at(113, row_index));
                        let sem_adjuststat = read_string(_batch.at(114, row_index));
                        let entry_method = read_string(_batch.at(115, row_index));
                        let fin_dqofficial = read_u64(_batch.at(116, row_index));
                        let pre_dqofficial = read_u64(_batch.at(117, row_index));
                        let sem_dqofficial = read_u64(_batch.at(118, row_index));
                        let pre_contacted = read_bool(_batch.at(119, row_index));
                        let obj = Relay {
                            event_ptr,
                            team_no,
                            team_ltr,
                            rel_age,
                            rel_sex,
                            act_seed_course,
                            actual_seed_time,
                            conv_seed_course,
                            conv_seed_time,
                            scr_stat,
                            spec_stat,
                            dec_stat,
                            alt_stat,
                            bonus_event,
                            div_no,
                            ev_score,
                            dq_type,
                            pre_heat,
                            pre_lane,
                            pre_stat,
                            pre__time,
                            pre_course,
                            pre_heatplace,
                            pre_place,
                            pre_jdplace,
                            pre_exh,
                            pre_points,
                            pre_back1,
                            pre_back2,
                            pre_back3,
                            fin_heat,
                            fin_lane,
                            fin_group,
                            fin_stat,
                            fin__time,
                            fin_course,
                            fin_heatplace,
                            fin_jdheatplace,
                            fin_place,
                            fin_jdplace,
                            fin_ptsplace,
                            fin_exh,
                            fin_points,
                            fin_back1,
                            fin_back2,
                            fin_back3,
                            sem_heat,
                            sem_lane,
                            sem_stat,
                            sem__time,
                            sem_course,
                            sem_heatplace,
                            sem_place,
                            sem_jdplace,
                            sem_exh,
                            sem_points,
                            sem_back1,
                            sem_back2,
                            sem_back3,
                            swimoff_heat,
                            swimoff_lane,
                            swimoff_stat,
                            swimoff__time,
                            swimoff_course,
                            swimoff_heatplace,
                            swimoff_place,
                            swimoff_jdplace,
                            swimoff_points,
                            swimoff_back1,
                            swimoff_back2,
                            swimoff_back3,
                            j_d_ev_score,
                            relay_no,
                            seed_place,
                            fin_heatltr,
                            pre_watch1,
                            pre_pad,
                            sem_watch1,
                            sem_pad,
                            fin_watch1,
                            fin_pad,
                            fin_reactiontime1,
                            pre_reactiontime1,
                            sem_reactiontime1,
                            fin_dqcode,
                            pre_dqcode,
                            sem_dqcode,
                            fin_dqcode_second,
                            pre_dqcode_second,
                            sem_dqcode_second,
                            fin_reactiontime2,
                            pre_reactiontime2,
                            sem_reactiontime2,
                            fin_reactiontime3,
                            pre_reactiontime3,
                            sem_reactiontime3,
                            fin_reactiontime4,
                            pre_reactiontime4,
                            sem_reactiontime4,
                            fin__time_type,
                            pre__time_type,
                            sem__time_type,
                            fin_dolphin1,
                            fin_dolphin2,
                            fin_dolphin3,
                            pre_dolphin1,
                            pre_dolphin2,
                            pre_dolphin3,
                            sem_dolphin1,
                            sem_dolphin2,
                            sem_dolphin3,
                            early_seed,
                            fin_adjuststat,
                            pre_adjuststat,
                            sem_adjuststat,
                            entry_method,
                            fin_dqofficial,
                            pre_dqofficial,
                            sem_dqofficial,
                            pre_contacted
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
