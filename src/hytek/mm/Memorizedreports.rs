// 
// 
// CREATE TABLE [MemorizedReports]
// (
// [Mem_Name] Text (80),
// [Mem_Type] Integer,
// [Mem_Ptr] Long Integer,
// [Num_Columns] Integer,
// [Sort_Order] Integer,
// [Date_Time] Integer,
// [ID_Type] Integer,
// [Top_HowMany] Integer,
// [Num_RelayNames] Integer,
// [Show_StartTimes] Boolean NOT NULL,
// [Incl_Records] Boolean NOT NULL,
// [Incl_TimeStds] Boolean NOT NULL,
// [Incl_QualTimes] Boolean NOT NULL,
// [Incl_EvtComments] Boolean NOT NULL,
// [Line_ForResults] Boolean NOT NULL,
// [Incl_NoEntries] Boolean NOT NULL,
// [Incl_PriorResults] Boolean NOT NULL,
// [Incl_Rnd1Alt] Boolean NOT NULL,
// [Incl_EmptyLanes] Boolean NOT NULL,
// [Show_SeedTimes] Boolean NOT NULL,
// [Sep_ABFinal] Boolean NOT NULL,
// [OneEvent_PerPage] Boolean NOT NULL,
// [Ref_Format] Boolean NOT NULL,
// [OneHeat_PerPage] Boolean NOT NULL,
// [Dbl_Space] Boolean NOT NULL,
// [Show_Ranks] Boolean NOT NULL,
// [MultiAge_Split] Boolean NOT NULL,
// [Incl_QualifiedAlts] Boolean NOT NULL,
// [ScrAltExhSpec_Filters] Integer,
// [Incl_Scratches] Boolean NOT NULL,
// [Ignore_Psych] Boolean NOT NULL,
// [Sess_Row] Long Integer,
// [Evt_Gender] Integer,
// [Evt_LowAge] Integer,
// [Evt_HighAge] Integer,
// [Team_Abbr] Text (18),
// [Evt_Round] Integer,
// [Evt_IndivOrRelay] Integer,
// [Report_Type] Integer,
// [Sort_OrderAthAge] Integer,
// [Incl_AthNoEntries] Boolean NOT NULL,
// [Incl_AthNoEntries4Col] Boolean NOT NULL,
// [AddApost_ClassYear] Boolean NOT NULL,
// [Incl_CompNo] Boolean NOT NULL,
// [Incl_CompNo4Col] Boolean NOT NULL,
// [AddrSort_ByTeam] Boolean NOT NULL,
// [AddrSort_ByZip] Boolean NOT NULL,
// [Incl_ScrInEntryCount] Boolean NOT NULL,
// [Incl_AltInEntryCount] Boolean NOT NULL,
// [Incl_BirthDate] Boolean NOT NULL,
// [Incl_TeamAddr] Boolean NOT NULL,
// [Incl_Coaches] Boolean NOT NULL,
// [AthUseAbbr_ForTeam] Boolean NOT NULL,
// [Div_Abbr] Text (60),
// [Report_Format] Integer,
// [Incl_HeatLane] Boolean NOT NULL,
// [Add_LineSpace] Boolean NOT NULL,
// [Incl_RegID] Boolean NOT NULL,
// [Show_CheckIn] Boolean NOT NULL,
// [NumAth_PerPage] Integer,
// [Splits_Choice] Integer,
// [Results_ByHeat] Boolean NOT NULL,
// [Page_Break] Boolean NOT NULL,
// [Incl_SpecPts] Boolean NOT NULL,
// [Incl_TimeTrials] Boolean NOT NULL,
// [Incl_NoShows] Boolean NOT NULL,
// [Incl_TeamPts] Boolean NOT NULL,
// [Low_Lane] Integer,
// [High_Lane] Integer,
// [Score_Female] Boolean NOT NULL,
// [Score_Male] Boolean NOT NULL,
// [Score_Combined] Boolean NOT NULL,
// [Score_CombinedBoth] Boolean NOT NULL,
// [BAG_CATS] Boolean NOT NULL,
// [Flat_HTML] Boolean NOT NULL,
// [DotMatrix_LabelChoice] Text (80),
// [Laser_LabelChoice] Text (80),
// [Incl_TeamScore] Boolean NOT NULL,
// [Incl_FemaleTeamScore] Boolean NOT NULL,
// [Incl_MaleTeamScore] Boolean NOT NULL,
// [CombineDivisions_ForTeamPoints] Boolean NOT NULL,
// [Incl_DQCodes] Boolean NOT NULL,
// [Incl_ReactionTimes] Boolean NOT NULL,
// [Incl_Backups] Boolean NOT NULL,
// [UseLaser_Label] Integer,
// [UseDQTimesfor_CombinedEvents] Boolean NOT NULL,
// [Incl_EntryTimes] Boolean NOT NULL,
// [Incl_PriorResultsSplits] Boolean NOT NULL,
// [Incl_LogosinFooter] Boolean NOT NULL,
// [LaneTimer_Pads] Boolean NOT NULL,
// [UseBestTimes_AllRounds] Boolean NOT NULL,
// [Qual_Club] Boolean NOT NULL,
// [QualClub_Scorers] Boolean NOT NULL,
// [PtBreakOut_HighPt] Boolean NOT NULL,
// [RTF_export] Boolean NOT NULL,
// [Results_ByHeatInclLane] Boolean NOT NULL,
// [NoShows_Only] Boolean NOT NULL,
// [Scratches_Only] Boolean NOT NULL,
// [DQs_Only] Boolean NOT NULL,
// [Combined_BothMustScore] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Memorizedreports {
    mem__name : Option<String>,
    mem__type : Option<u16>,
    mem__ptr : Option<u64>,
    num__columns : Option<u16>,
    sort__order : Option<u16>,
    date__time : Option<u16>,
    i_d__type : Option<u16>,
    top__how_many : Option<u16>,
    num__relay_names : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show__start_times : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__records : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__time_stds : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__qual_times : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__evt_comments : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    line__for_results : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__no_entries : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__prior_results : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__rnd1_alt : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__empty_lanes : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show__seed_times : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    sep__a_b_final : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    one_event__per_page : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    ref__format : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    one_heat__per_page : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    dbl__space : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show__ranks : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    multi_age__split : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__qualified_alts : bool,
    scr_alt_exh_spec__filters : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__scratches : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    ignore__psych : bool,
    sess__row : Option<u64>,
    evt__gender : Option<u16>,
    evt__low_age : Option<u16>,
    evt__high_age : Option<u16>,
    team__abbr : Option<String>,
    evt__round : Option<u16>,
    evt__indiv_or_relay : Option<u16>,
    report__type : Option<u16>,
    sort__order_ath_age : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__ath_no_entries : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__ath_no_entries4_col : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    add_apost__class_year : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__comp_no : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__comp_no4_col : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    addr_sort__by_team : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    addr_sort__by_zip : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__scr_in_entry_count : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__alt_in_entry_count : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__birth_date : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__team_addr : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__coaches : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    ath_use_abbr__for_team : bool,
    div__abbr : Option<String>,
    report__format : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__heat_lane : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    add__line_space : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__reg_i_d : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show__check_in : bool,
    num_ath__per_page : Option<u16>,
    splits__choice : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    results__by_heat : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    page__break : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__spec_pts : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__time_trials : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__no_shows : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__team_pts : bool,
    low__lane : Option<u16>,
    high__lane : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    score__female : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    score__male : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    score__combined : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    score__combined_both : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    BAG_CATS : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    flat__h_t_m_l : bool,
    dot_matrix__label_choice : Option<String>,
    laser__label_choice : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__team_score : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__female_team_score : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__male_team_score : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    combine_divisions__for_team_points : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__d_q_codes : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__reaction_times : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__backups : bool,
    use_laser__label : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_d_q_timesfor__combined_events : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__entry_times : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__prior_results_splits : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    incl__logosin_footer : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane_timer__pads : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_best_times__all_rounds : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    qual__club : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    qual_club__scorers : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    pt_break_out__high_pt : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    r_t_f_export : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    results__by_heat_incl_lane : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    no_shows__only : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    scratches__only : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    d_qs__only : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    combined__both_must_score : bool
}
impl Memorizedreports {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Memorizedreports>, Box<dyn Error>> {
       let mut vec: Vec<Memorizedreports> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Memorizedreports = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Memorizedreports>, Error> {
        let mut vec: Vec<Memorizedreports> = Vec::new();
        match conn.execute("SELECT * FROM memorized reports", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let mem__name = read_string(_batch.at(0, row_index));
                        let mem__type = read_u16(_batch.at(1, row_index));
                        let mem__ptr = read_u64(_batch.at(2, row_index));
                        let num__columns = read_u16(_batch.at(3, row_index));
                        let sort__order = read_u16(_batch.at(4, row_index));
                        let date__time = read_u16(_batch.at(5, row_index));
                        let i_d__type = read_u16(_batch.at(6, row_index));
                        let top__how_many = read_u16(_batch.at(7, row_index));
                        let num__relay_names = read_u16(_batch.at(8, row_index));
                        let show__start_times = read_bool(_batch.at(9, row_index));
                        let incl__records = read_bool(_batch.at(10, row_index));
                        let incl__time_stds = read_bool(_batch.at(11, row_index));
                        let incl__qual_times = read_bool(_batch.at(12, row_index));
                        let incl__evt_comments = read_bool(_batch.at(13, row_index));
                        let line__for_results = read_bool(_batch.at(14, row_index));
                        let incl__no_entries = read_bool(_batch.at(15, row_index));
                        let incl__prior_results = read_bool(_batch.at(16, row_index));
                        let incl__rnd1_alt = read_bool(_batch.at(17, row_index));
                        let incl__empty_lanes = read_bool(_batch.at(18, row_index));
                        let show__seed_times = read_bool(_batch.at(19, row_index));
                        let sep__a_b_final = read_bool(_batch.at(20, row_index));
                        let one_event__per_page = read_bool(_batch.at(21, row_index));
                        let ref__format = read_bool(_batch.at(22, row_index));
                        let one_heat__per_page = read_bool(_batch.at(23, row_index));
                        let dbl__space = read_bool(_batch.at(24, row_index));
                        let show__ranks = read_bool(_batch.at(25, row_index));
                        let multi_age__split = read_bool(_batch.at(26, row_index));
                        let incl__qualified_alts = read_bool(_batch.at(27, row_index));
                        let scr_alt_exh_spec__filters = read_u16(_batch.at(28, row_index));
                        let incl__scratches = read_bool(_batch.at(29, row_index));
                        let ignore__psych = read_bool(_batch.at(30, row_index));
                        let sess__row = read_u64(_batch.at(31, row_index));
                        let evt__gender = read_u16(_batch.at(32, row_index));
                        let evt__low_age = read_u16(_batch.at(33, row_index));
                        let evt__high_age = read_u16(_batch.at(34, row_index));
                        let team__abbr = read_string(_batch.at(35, row_index));
                        let evt__round = read_u16(_batch.at(36, row_index));
                        let evt__indiv_or_relay = read_u16(_batch.at(37, row_index));
                        let report__type = read_u16(_batch.at(38, row_index));
                        let sort__order_ath_age = read_u16(_batch.at(39, row_index));
                        let incl__ath_no_entries = read_bool(_batch.at(40, row_index));
                        let incl__ath_no_entries4_col = read_bool(_batch.at(41, row_index));
                        let add_apost__class_year = read_bool(_batch.at(42, row_index));
                        let incl__comp_no = read_bool(_batch.at(43, row_index));
                        let incl__comp_no4_col = read_bool(_batch.at(44, row_index));
                        let addr_sort__by_team = read_bool(_batch.at(45, row_index));
                        let addr_sort__by_zip = read_bool(_batch.at(46, row_index));
                        let incl__scr_in_entry_count = read_bool(_batch.at(47, row_index));
                        let incl__alt_in_entry_count = read_bool(_batch.at(48, row_index));
                        let incl__birth_date = read_bool(_batch.at(49, row_index));
                        let incl__team_addr = read_bool(_batch.at(50, row_index));
                        let incl__coaches = read_bool(_batch.at(51, row_index));
                        let ath_use_abbr__for_team = read_bool(_batch.at(52, row_index));
                        let div__abbr = read_string(_batch.at(53, row_index));
                        let report__format = read_u16(_batch.at(54, row_index));
                        let incl__heat_lane = read_bool(_batch.at(55, row_index));
                        let add__line_space = read_bool(_batch.at(56, row_index));
                        let incl__reg_i_d = read_bool(_batch.at(57, row_index));
                        let show__check_in = read_bool(_batch.at(58, row_index));
                        let num_ath__per_page = read_u16(_batch.at(59, row_index));
                        let splits__choice = read_u16(_batch.at(60, row_index));
                        let results__by_heat = read_bool(_batch.at(61, row_index));
                        let page__break = read_bool(_batch.at(62, row_index));
                        let incl__spec_pts = read_bool(_batch.at(63, row_index));
                        let incl__time_trials = read_bool(_batch.at(64, row_index));
                        let incl__no_shows = read_bool(_batch.at(65, row_index));
                        let incl__team_pts = read_bool(_batch.at(66, row_index));
                        let low__lane = read_u16(_batch.at(67, row_index));
                        let high__lane = read_u16(_batch.at(68, row_index));
                        let score__female = read_bool(_batch.at(69, row_index));
                        let score__male = read_bool(_batch.at(70, row_index));
                        let score__combined = read_bool(_batch.at(71, row_index));
                        let score__combined_both = read_bool(_batch.at(72, row_index));
                        let BAG_CATS = read_bool(_batch.at(73, row_index));
                        let flat__h_t_m_l = read_bool(_batch.at(74, row_index));
                        let dot_matrix__label_choice = read_string(_batch.at(75, row_index));
                        let laser__label_choice = read_string(_batch.at(76, row_index));
                        let incl__team_score = read_bool(_batch.at(77, row_index));
                        let incl__female_team_score = read_bool(_batch.at(78, row_index));
                        let incl__male_team_score = read_bool(_batch.at(79, row_index));
                        let combine_divisions__for_team_points = read_bool(_batch.at(80, row_index));
                        let incl__d_q_codes = read_bool(_batch.at(81, row_index));
                        let incl__reaction_times = read_bool(_batch.at(82, row_index));
                        let incl__backups = read_bool(_batch.at(83, row_index));
                        let use_laser__label = read_u16(_batch.at(84, row_index));
                        let use_d_q_timesfor__combined_events = read_bool(_batch.at(85, row_index));
                        let incl__entry_times = read_bool(_batch.at(86, row_index));
                        let incl__prior_results_splits = read_bool(_batch.at(87, row_index));
                        let incl__logosin_footer = read_bool(_batch.at(88, row_index));
                        let lane_timer__pads = read_bool(_batch.at(89, row_index));
                        let use_best_times__all_rounds = read_bool(_batch.at(90, row_index));
                        let qual__club = read_bool(_batch.at(91, row_index));
                        let qual_club__scorers = read_bool(_batch.at(92, row_index));
                        let pt_break_out__high_pt = read_bool(_batch.at(93, row_index));
                        let r_t_f_export = read_bool(_batch.at(94, row_index));
                        let results__by_heat_incl_lane = read_bool(_batch.at(95, row_index));
                        let no_shows__only = read_bool(_batch.at(96, row_index));
                        let scratches__only = read_bool(_batch.at(97, row_index));
                        let d_qs__only = read_bool(_batch.at(98, row_index));
                        let combined__both_must_score = read_bool(_batch.at(99, row_index));
                        let obj = Memorizedreports {
                            mem__name,
                            mem__type,
                            mem__ptr,
                            num__columns,
                            sort__order,
                            date__time,
                            i_d__type,
                            top__how_many,
                            num__relay_names,
                            show__start_times,
                            incl__records,
                            incl__time_stds,
                            incl__qual_times,
                            incl__evt_comments,
                            line__for_results,
                            incl__no_entries,
                            incl__prior_results,
                            incl__rnd1_alt,
                            incl__empty_lanes,
                            show__seed_times,
                            sep__a_b_final,
                            one_event__per_page,
                            ref__format,
                            one_heat__per_page,
                            dbl__space,
                            show__ranks,
                            multi_age__split,
                            incl__qualified_alts,
                            scr_alt_exh_spec__filters,
                            incl__scratches,
                            ignore__psych,
                            sess__row,
                            evt__gender,
                            evt__low_age,
                            evt__high_age,
                            team__abbr,
                            evt__round,
                            evt__indiv_or_relay,
                            report__type,
                            sort__order_ath_age,
                            incl__ath_no_entries,
                            incl__ath_no_entries4_col,
                            add_apost__class_year,
                            incl__comp_no,
                            incl__comp_no4_col,
                            addr_sort__by_team,
                            addr_sort__by_zip,
                            incl__scr_in_entry_count,
                            incl__alt_in_entry_count,
                            incl__birth_date,
                            incl__team_addr,
                            incl__coaches,
                            ath_use_abbr__for_team,
                            div__abbr,
                            report__format,
                            incl__heat_lane,
                            add__line_space,
                            incl__reg_i_d,
                            show__check_in,
                            num_ath__per_page,
                            splits__choice,
                            results__by_heat,
                            page__break,
                            incl__spec_pts,
                            incl__time_trials,
                            incl__no_shows,
                            incl__team_pts,
                            low__lane,
                            high__lane,
                            score__female,
                            score__male,
                            score__combined,
                            score__combined_both,
                            BAG_CATS,
                            flat__h_t_m_l,
                            dot_matrix__label_choice,
                            laser__label_choice,
                            incl__team_score,
                            incl__female_team_score,
                            incl__male_team_score,
                            combine_divisions__for_team_points,
                            incl__d_q_codes,
                            incl__reaction_times,
                            incl__backups,
                            use_laser__label,
                            use_d_q_timesfor__combined_events,
                            incl__entry_times,
                            incl__prior_results_splits,
                            incl__logosin_footer,
                            lane_timer__pads,
                            use_best_times__all_rounds,
                            qual__club,
                            qual_club__scorers,
                            pt_break_out__high_pt,
                            r_t_f_export,
                            results__by_heat_incl_lane,
                            no_shows__only,
                            scratches__only,
                            d_qs__only,
                            combined__both_must_score
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
