// 
// 
// CREATE TABLE [Event]
// (
// [Event_no] Integer,
// [Event_ltr] Text (2),
// [Event_ptr] Long Integer,
// [Ind_rel] Text (2),
// [Event_sex] Text (2),
// [Event_gender] Text (2),
// [Event_dist] Single,
// [Event_stroke] Text (2),
// [Low_age] Integer,
// [High_Age] Integer,
// [Multi_age] Boolean NOT NULL,
// [Event_stat] Text (2),
// [Event_rounds] Integer,
// [Num_prelanes] Integer,
// [Num_finlanes] Integer,
// [Heats_infinal] Text (2),
// [Heats_insemi] Integer,
// [Std_lanes] Text (2),
// [Auto_seed] Boolean NOT NULL,
// [Twoperlane_req] Boolean NOT NULL,
// [Preheat_order] Text (2),
// [Finheat_order] Text (2),
// [Score_event] Boolean NOT NULL,
// [Div_no] Integer,
// [Relay_size] Integer,
// [Comm_1] Text (72),
// [Comm_2] Text (72),
// [Comm_3] Text (72),
// [Comm_4] Text (72),
// [Entry_fee] Currency,
// [Is_locked] Boolean NOT NULL,
// [Locked_by] Text (40),
// [Event_Type] Text (2),
// [Locked_list] Text (24),
// [Event_note] Text (40),
// [Suppress_stroke] Boolean NOT NULL,
// [Custom_ABCFinal] Boolean NOT NULL,
// [Num_dives] Integer,
// [Num_HeatsInFinal] Integer,
// [Multiage_SuperFinal] Boolean NOT NULL,
// [Finals_LanesVary] Boolean NOT NULL,
// [Finals_LanesVaryOrder] Text (36),
// [ABCfinal_order] Text (12),
// [ABC_Style] Boolean NOT NULL,
// [PrelimsAs_ExtendedFinal] Boolean NOT NULL,
// [Num_LanesInBestHeatsTimedFinal] Integer,
// [Num_BestHeatsTimedFinal] Integer,
// [TimedFinal_LanesVary] Boolean NOT NULL,
// [ScoreTimedFinal_asABC] Boolean NOT NULL,
// [Num_HeatsInTimedFinalToScore] Integer,
// [Pads_BothEnds] Boolean NOT NULL,
// [Multiage_SuperSeed] Boolean NOT NULL,
// [Suppress_distance] Boolean NOT NULL,
// [Fin_AwardsPrinted] Boolean NOT NULL,
// [Pre_AwardsPrinted] Boolean NOT NULL,
// [Sem_AwardsPrinted] Boolean NOT NULL,
// [FastTimeStd_Abbr] Text (8),
// [SlowTimeStd_Abbr] Text (8),
// [SuperFinal_ElimOldAgeGrp] Boolean NOT NULL,
// [SeedMultiAge_OldToYoung] Boolean NOT NULL,
// [Multi_ageScnd] Boolean NOT NULL,
// [Num_RelayLegs] Integer,
// [Pads_BothEndsFinals] Boolean NOT NULL,
// [Checkin_starttime] Long Integer,
// [Checkin_endtime] Long Integer,
// [Checkin_startdate] DateTime,
// [Checkin_enddate] DateTime,
// [Num_semlanes] Integer,
// [EvtMaxAgeFor_CFinal] Integer,
// [EvtMaxAgeNumHeats_CFinal] Integer,
// [fin_actualstarttime] Long Integer,
// [sem_actualstarttime] Long Integer,
// [pre_actualstarttime] Long Integer,
// [Swimoff_SourcePtr] Long Integer,
// [Swimoff_SourceRndLtr] Text (2),
// [Swimoff_Created] DateTime,
// [Multiage_BestRestFinal] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Event {
    event_no : Option<u16>,
    event_ltr : Option<String>,
    event_ptr : Option<u64>,
    ind_rel : Option<String>,
    event_sex : Option<String>,
    event_gender : Option<String>,
    event_dist : Option<f32>,
    event_stroke : Option<String>,
    low_age : Option<u16>,
    high__age : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    multi_age : bool,
    event_stat : Option<String>,
    event_rounds : Option<u16>,
    num_prelanes : Option<u16>,
    num_finlanes : Option<u16>,
    heats_infinal : Option<String>,
    heats_insemi : Option<u16>,
    std_lanes : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    auto_seed : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    twoperlane_req : bool,
    preheat_order : Option<String>,
    finheat_order : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    score_event : bool,
    div_no : Option<u16>,
    relay_size : Option<u16>,
    comm__1 : Option<String>,
    comm__2 : Option<String>,
    comm__3 : Option<String>,
    comm__4 : Option<String>,
    entry_fee : Option<Decimal>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    is_locked : bool,
    locked_by : Option<String>,
    event__type : Option<String>,
    locked_list : Option<String>,
    event_note : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress_stroke : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    custom__a_b_c_final : bool,
    num_dives : Option<u16>,
    num__heats_in_final : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    multiage__super_final : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    finals__lanes_vary : bool,
    finals__lanes_vary_order : Option<String>,
    a_b_cfinal_order : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    a_b_c__style : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    prelims_as__extended_final : bool,
    num__lanes_in_best_heats_timed_final : Option<u16>,
    num__best_heats_timed_final : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    timed_final__lanes_vary : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    score_timed_final_as_a_b_c : bool,
    num__heats_in_timed_final_to_score : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    pads__both_ends : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    multiage__super_seed : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress_distance : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    fin__awards_printed : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    pre__awards_printed : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    sem__awards_printed : bool,
    fast_time_std__abbr : Option<String>,
    slow_time_std__abbr : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    super_final__elim_old_age_grp : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    seed_multi_age__old_to_young : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    multi_age_scnd : bool,
    num__relay_legs : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    pads__both_ends_finals : bool,
    checkin_starttime : Option<u64>,
    checkin_endtime : Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    checkin_startdate : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    checkin_enddate : Option<NaiveDateTime>,
    num_semlanes : Option<u16>,
    evt_max_age_for__c_final : Option<u16>,
    evt_max_age_num_heats__c_final : Option<u16>,
    fin_actualstarttime : Option<u64>,
    sem_actualstarttime : Option<u64>,
    pre_actualstarttime : Option<u64>,
    swimoff__source_ptr : Option<u64>,
    swimoff__source_rnd_ltr : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    swimoff__created : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    multiage__best_rest_final : bool
}
impl Event {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Event>, Box<dyn Error>> {
       let mut vec: Vec<Event> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Event = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Event>, Error> {
        let mut vec: Vec<Event> = Vec::new();
        match conn.execute("SELECT * FROM event", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let event_no = read_u16(_batch.at(0, row_index));
                        let event_ltr = read_string(_batch.at(1, row_index));
                        let event_ptr = read_u64(_batch.at(2, row_index));
                        let ind_rel = read_string(_batch.at(3, row_index));
                        let event_sex = read_string(_batch.at(4, row_index));
                        let event_gender = read_string(_batch.at(5, row_index));
                        let event_dist = read_f32(_batch.at(6, row_index));
                        let event_stroke = read_string(_batch.at(7, row_index));
                        let low_age = read_u16(_batch.at(8, row_index));
                        let high__age = read_u16(_batch.at(9, row_index));
                        let multi_age = read_bool(_batch.at(10, row_index));
                        let event_stat = read_string(_batch.at(11, row_index));
                        let event_rounds = read_u16(_batch.at(12, row_index));
                        let num_prelanes = read_u16(_batch.at(13, row_index));
                        let num_finlanes = read_u16(_batch.at(14, row_index));
                        let heats_infinal = read_string(_batch.at(15, row_index));
                        let heats_insemi = read_u16(_batch.at(16, row_index));
                        let std_lanes = read_string(_batch.at(17, row_index));
                        let auto_seed = read_bool(_batch.at(18, row_index));
                        let twoperlane_req = read_bool(_batch.at(19, row_index));
                        let preheat_order = read_string(_batch.at(20, row_index));
                        let finheat_order = read_string(_batch.at(21, row_index));
                        let score_event = read_bool(_batch.at(22, row_index));
                        let div_no = read_u16(_batch.at(23, row_index));
                        let relay_size = read_u16(_batch.at(24, row_index));
                        let comm__1 = read_string(_batch.at(25, row_index));
                        let comm__2 = read_string(_batch.at(26, row_index));
                        let comm__3 = read_string(_batch.at(27, row_index));
                        let comm__4 = read_string(_batch.at(28, row_index));
                        let entry_fee = read_decimal(_batch.at(29, row_index));
                        let is_locked = read_bool(_batch.at(30, row_index));
                        let locked_by = read_string(_batch.at(31, row_index));
                        let event__type = read_string(_batch.at(32, row_index));
                        let locked_list = read_string(_batch.at(33, row_index));
                        let event_note = read_string(_batch.at(34, row_index));
                        let suppress_stroke = read_bool(_batch.at(35, row_index));
                        let custom__a_b_c_final = read_bool(_batch.at(36, row_index));
                        let num_dives = read_u16(_batch.at(37, row_index));
                        let num__heats_in_final = read_u16(_batch.at(38, row_index));
                        let multiage__super_final = read_bool(_batch.at(39, row_index));
                        let finals__lanes_vary = read_bool(_batch.at(40, row_index));
                        let finals__lanes_vary_order = read_string(_batch.at(41, row_index));
                        let a_b_cfinal_order = read_string(_batch.at(42, row_index));
                        let a_b_c__style = read_bool(_batch.at(43, row_index));
                        let prelims_as__extended_final = read_bool(_batch.at(44, row_index));
                        let num__lanes_in_best_heats_timed_final = read_u16(_batch.at(45, row_index));
                        let num__best_heats_timed_final = read_u16(_batch.at(46, row_index));
                        let timed_final__lanes_vary = read_bool(_batch.at(47, row_index));
                        let score_timed_final_as_a_b_c = read_bool(_batch.at(48, row_index));
                        let num__heats_in_timed_final_to_score = read_u16(_batch.at(49, row_index));
                        let pads__both_ends = read_bool(_batch.at(50, row_index));
                        let multiage__super_seed = read_bool(_batch.at(51, row_index));
                        let suppress_distance = read_bool(_batch.at(52, row_index));
                        let fin__awards_printed = read_bool(_batch.at(53, row_index));
                        let pre__awards_printed = read_bool(_batch.at(54, row_index));
                        let sem__awards_printed = read_bool(_batch.at(55, row_index));
                        let fast_time_std__abbr = read_string(_batch.at(56, row_index));
                        let slow_time_std__abbr = read_string(_batch.at(57, row_index));
                        let super_final__elim_old_age_grp = read_bool(_batch.at(58, row_index));
                        let seed_multi_age__old_to_young = read_bool(_batch.at(59, row_index));
                        let multi_age_scnd = read_bool(_batch.at(60, row_index));
                        let num__relay_legs = read_u16(_batch.at(61, row_index));
                        let pads__both_ends_finals = read_bool(_batch.at(62, row_index));
                        let checkin_starttime = read_u64(_batch.at(63, row_index));
                        let checkin_endtime = read_u64(_batch.at(64, row_index));
                        let checkin_startdate = read_datetime(_batch.at(65, row_index));
                        let checkin_enddate = read_datetime(_batch.at(66, row_index));
                        let num_semlanes = read_u16(_batch.at(67, row_index));
                        let evt_max_age_for__c_final = read_u16(_batch.at(68, row_index));
                        let evt_max_age_num_heats__c_final = read_u16(_batch.at(69, row_index));
                        let fin_actualstarttime = read_u64(_batch.at(70, row_index));
                        let sem_actualstarttime = read_u64(_batch.at(71, row_index));
                        let pre_actualstarttime = read_u64(_batch.at(72, row_index));
                        let swimoff__source_ptr = read_u64(_batch.at(73, row_index));
                        let swimoff__source_rnd_ltr = read_string(_batch.at(74, row_index));
                        let swimoff__created = read_datetime(_batch.at(75, row_index));
                        let multiage__best_rest_final = read_bool(_batch.at(76, row_index));
                        let obj = Event {
                            event_no,
                            event_ltr,
                            event_ptr,
                            ind_rel,
                            event_sex,
                            event_gender,
                            event_dist,
                            event_stroke,
                            low_age,
                            high__age,
                            multi_age,
                            event_stat,
                            event_rounds,
                            num_prelanes,
                            num_finlanes,
                            heats_infinal,
                            heats_insemi,
                            std_lanes,
                            auto_seed,
                            twoperlane_req,
                            preheat_order,
                            finheat_order,
                            score_event,
                            div_no,
                            relay_size,
                            comm__1,
                            comm__2,
                            comm__3,
                            comm__4,
                            entry_fee,
                            is_locked,
                            locked_by,
                            event__type,
                            locked_list,
                            event_note,
                            suppress_stroke,
                            custom__a_b_c_final,
                            num_dives,
                            num__heats_in_final,
                            multiage__super_final,
                            finals__lanes_vary,
                            finals__lanes_vary_order,
                            a_b_cfinal_order,
                            a_b_c__style,
                            prelims_as__extended_final,
                            num__lanes_in_best_heats_timed_final,
                            num__best_heats_timed_final,
                            timed_final__lanes_vary,
                            score_timed_final_as_a_b_c,
                            num__heats_in_timed_final_to_score,
                            pads__both_ends,
                            multiage__super_seed,
                            suppress_distance,
                            fin__awards_printed,
                            pre__awards_printed,
                            sem__awards_printed,
                            fast_time_std__abbr,
                            slow_time_std__abbr,
                            super_final__elim_old_age_grp,
                            seed_multi_age__old_to_young,
                            multi_age_scnd,
                            num__relay_legs,
                            pads__both_ends_finals,
                            checkin_starttime,
                            checkin_endtime,
                            checkin_startdate,
                            checkin_enddate,
                            num_semlanes,
                            evt_max_age_for__c_final,
                            evt_max_age_num_heats__c_final,
                            fin_actualstarttime,
                            sem_actualstarttime,
                            pre_actualstarttime,
                            swimoff__source_ptr,
                            swimoff__source_rnd_ltr,
                            swimoff__created,
                            multiage__best_rest_final
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
