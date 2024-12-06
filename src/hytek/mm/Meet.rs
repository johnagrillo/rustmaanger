// 
// 
// CREATE TABLE [Meet]
// (
// [Meet_name1] Text (90),
// [Meet_location] Text (90),
// [Meet_start] DateTime,
// [Meet_end] DateTime,
// [Meet_idformat] Integer,
// [Meet_class] Integer,
// [Meet_meettype] Integer,
// [Meet_course] Integer,
// [Enter_ages] Boolean NOT NULL,
// [Enter_birthdate] Boolean NOT NULL,
// [Calc_date] DateTime,
// [Enter_schoolyr] Boolean NOT NULL,
// [A_Relaysonly] Boolean NOT NULL,
// [Use_hometown] Boolean NOT NULL,
// [Show_countrycode] Boolean NOT NULL,
// [Scores_afterevt] Boolean NOT NULL,
// [Lastname_first] Boolean NOT NULL,
// [Punct_names] Boolean NOT NULL,
// [Punct_teams] Boolean NOT NULL,
// [win_mm] Boolean NOT NULL,
// [meet_numlanes] Integer,
// [prelimheats_circle] Integer,
// [timedfinal_circleseed] Boolean NOT NULL,
// [foreign_infinal] Boolean NOT NULL,
// [exh_infinal] Boolean NOT NULL,
// [nonconform_last] Boolean NOT NULL,
// [course_order] Text (510),
// [seed_exhlast] Boolean NOT NULL,
// [dual_evenodd] Boolean NOT NULL,
// [strict_evenodd] Boolean NOT NULL,
// [team_evenlanes] Long Integer,
// [team_oddlanes] Long Integer,
// [masters_bytimeonly] Boolean NOT NULL,
// [masters_agegrpsskip] Integer,
// [timer_port] Integer,
// [scbd_port] Integer,
// [timer_vendor] Text (8),
// [scbd_vendor] Text (8),
// [show_initial] Boolean NOT NULL,
// [ucase_names] Boolean NOT NULL,
// [ucase_teams] Boolean NOT NULL,
// [open_senior_none] Text (2),
// [entryqual_faster] Boolean NOT NULL,
// [indentryfee_surcharge] Currency,
// [anyone_onrelay] Boolean NOT NULL,
// [language_choice] Text (40),
// [military_time] Boolean NOT NULL,
// [check_times] Boolean NOT NULL,
// [enterkey_astab] Boolean NOT NULL,
// [double_endedsplits] Boolean NOT NULL,
// [use_compnumbers] Boolean NOT NULL,
// [flighted_minentries] Integer,
// [diffpts_malefemale] Boolean NOT NULL,
// [diffpts_eachdivision] Boolean NOT NULL,
// [scoreonly_ifexceedqualtime] Boolean NOT NULL,
// [score_fastestheatonly] Boolean NOT NULL,
// [entrylimits_warn] Boolean NOT NULL,
// [pointsbasedon_seedtime] Boolean NOT NULL,
// [pointsfor_overachievers] Boolean NOT NULL,
// [pointsfor_underachievers] Boolean NOT NULL,
// [indmaxscorers_perteam] Integer,
// [relmaxscorers_perteam] Integer,
// [indtopmany_awards] Integer,
// [reltopmany_awards] Integer,
// [entrymax_total] Integer,
// [indmax_perath] Integer,
// [relmax_perath] Integer,
// [foreign_getteampoints] Boolean NOT NULL,
// [include_swimupsinteamscore] Boolean NOT NULL,
// [enter_citizenof] Boolean NOT NULL,
// [meet_meetstyle] Integer,
// [flag_overachievers] Boolean NOT NULL,
// [flag_underachievers] Boolean NOT NULL,
// [scbd_punctuation] Integer,
// [scbd_names] Integer,
// [scbd_relaynames] Integer,
// [scbd_cycle] Boolean NOT NULL,
// [scbd_cycleseconds] Integer,
// [copies_toprinter] Integer,
// [report_headersonly] Boolean NOT NULL,
// [autoinc_compno] Boolean NOT NULL,
// [pentscoring_usedqtime] Boolean NOT NULL,
// [swimmer_surcharge] Currency,
// [directly_toprinter] Boolean NOT NULL,
// [lastname_asinitial] Boolean NOT NULL,
// [under_eventname] Boolean NOT NULL,
// [suppress_Arelay] Boolean NOT NULL,
// [Punct_recholders] Boolean NOT NULL,
// [ucase_recholders] Boolean NOT NULL,
// [suppress_lsc] Boolean NOT NULL,
// [showathlete_status] Boolean NOT NULL,
// [open_lowage] Integer,
// [useeventsex_teamscore] Boolean NOT NULL,
// [suppress_smallx] Boolean NOT NULL,
// [score_Arelayonly] Boolean NOT NULL,
// [thirteenandover_assenior] Boolean NOT NULL,
// [suppress_jd] Boolean NOT NULL,
// [abcfinal_order] Text (6),
// [maxagefor_cfinal] Integer,
// [include_sanction] Boolean NOT NULL,
// [special_points] Integer,
// [countrelay_alt] Boolean NOT NULL,
// [UseNonConforming_PoolFactor] Boolean NOT NULL,
// [NonConforming_PoolFactor] Single,
// [apnews_team] Text (2),
// [PointsAwarded_ForDQ] Single,
// [PointsAwarded_ForScratch] Single,
// [PointsAwarded_ForNT] Single,
// [Enter_AthStat] Boolean NOT NULL,
// [Show_secondclub] Boolean NOT NULL,
// [firstinitial_fulllastname] Boolean NOT NULL,
// [turnon_autobackup] Boolean NOT NULL,
// [autobackup_interval] Integer,
// [PointsAwarded_ForExh] Boolean NOT NULL,
// [Use_AltTeamAbbr] Boolean NOT NULL,
// [IsCanadian_Masters] Boolean NOT NULL,
// [entry_msg] Text (160),
// [timedfinalnonconform_last] Boolean NOT NULL,
// [referee_name] Text (60),
// [referee_homphone] Text (40),
// [referee_offphone] Text (40),
// [Meet_altitude] Long Integer,
// [Read_Only] Boolean NOT NULL,
// [masters_indlowage] Integer,
// [masters_rellowage] Integer,
// [Import_Dir] Text (160),
// [Export_Dir] Text (160),
// [Backup_Dir] Text (160),
// [RestoreFrom_Dir] Text (160),
// [RestoreTo_Dir] Text (160),
// [FlatHtml_Dir] Text (160),
// [APNews_Dir] Text (160),
// [AllowSameEvent_DupRelayNames] Boolean NOT NULL,
// [dualteam_lane1] Long Integer,
// [dualteam_lane2] Long Integer,
// [dualteam_lane3] Long Integer,
// [dualteam_lane4] Long Integer,
// [dualteam_lane5] Long Integer,
// [dualteam_lane6] Long Integer,
// [dualteam_lane7] Long Integer,
// [dualteam_lane8] Long Integer,
// [dualteam_lane9] Long Integer,
// [dualteam_lane10] Long Integer,
// [strict_evenoddfastestheatonly] Boolean NOT NULL,
// [dualseeding_altunusedlane] Boolean NOT NULL,
// [team_surcharge] Currency,
// [display_actualentrytime] Boolean NOT NULL,
// [indmaxadvance_perteam] Integer,
// [relmaxadvance_perteam] Integer,
// [RelayNames_LinkByLSC] Boolean NOT NULL,
// [Flighted_BasedOnResultsTime] Boolean NOT NULL,
// [ShowYear_InPlaceOfAge] Boolean NOT NULL,
// [PenaltyPts_ForNS] Single,
// [EntryEligibility_date] DateTime,
// [Facility_Surcharge] Currency,
// [Suppress_TimeStdAbbr] Boolean NOT NULL,
// [Custom_QualTimes] Boolean NOT NULL,
// [Suppress_SplitsForDQs] Boolean NOT NULL,
// [Suppress_SplitsForDQsRelay] Boolean NOT NULL,
// [DQCodes_Type] Text (2),
// [SuppressTimes_NotMeetQualTime] Boolean NOT NULL,
// [Show_AgeandBirthYear] Boolean NOT NULL,
// [Meet_state] Text (6),
// [Meet_country] Text (6),
// [Meet_lsc] Text (6),
// [BCSSA_DivbyTimeStd] Boolean NOT NULL,
// [Show_HyTekDecimals] Boolean NOT NULL,
// [Suppress_ResultsAdvQ] Boolean NOT NULL,
// [RelaysAs_4x100Style] Boolean NOT NULL,
// [flighted_flightcount] Integer,
// [flighted_inclDQ] Boolean NOT NULL,
// [RelaysAlternate_TwoFastestFirst] Boolean NOT NULL,
// [dualteam_lane11] Long Integer,
// [dualteam_lane12] Long Integer,
// [entry_deadline] DateTime,
// [Meet_addr1] Text (60),
// [Meet_addr2] Text (60),
// [Meet_city] Text (60),
// [Meet_zip] Text (20),
// [Meet_hostlsc] Text (6),
// [Meet_USMastersMeetID] Text (40),
// [ShowFirstName_OverPreferred] Boolean NOT NULL,
// [ExcludeNTEntries_WhenImporting] Boolean NOT NULL,
// [Enter_birthcentury] Boolean NOT NULL,
// [Using_twopools] Boolean NOT NULL,
// [Pool1_name] Text (60),
// [Pool2_name] Text (60),
// [indtopmany_awardsSr] Integer,
// [reltopmany_awardsSr] Integer,
// [maxforeign_infinal] Integer,
// [Flag_FastestRecordOnly] Boolean NOT NULL,
// [athlete_earlysurcharge] Currency,
// [athlete_latesurcharge] Currency,
// [athlete_earlysurchargedate] DateTime,
// [athlete_latesurchargedate] DateTime,
// [entry_OMEopendate] DateTime,
// [DisplayNTfor_TimesUnder5Sec] Boolean NOT NULL,
// [SortTeamCombos_ByTeamName] Boolean NOT NULL,
// [FastestHeat_SomeLanesDoNotScore] Boolean NOT NULL,
// [RankDisabled_ByPoints] Boolean NOT NULL,
// [special_parapoints] Integer,
// [DisabledDoNot_AdvanceToFinals] Boolean NOT NULL,
// [prelimheats_circledist] Integer,
// [DisabledIgnoreQualTime_ForScoring] Boolean NOT NULL,
// [CountTimeTrial_Events] Boolean NOT NULL,
// [Meet_header1] Text (150),
// [Meet_header2] Text (150),
// [QualNonConformCourse_UseMinStd] Boolean NOT NULL,
// [entry_OpenDate] DateTime,
// [TimeAdj_Method] Text (2),
// [Lock_Reseed] Boolean NOT NULL,
// [sanction_number] Text (60),
// [Last_updated] Text (80),
// [MixedRelays_DividedPoints] Boolean NOT NULL,
// [RelayOnly_Surcharge] Currency,
// [DisabledSeedWithAgeGroup_IfTimedFinalSuperSeed] Boolean NOT NULL,
// [Competition_Code] Text (12),
// [PenaltyTimeSec_ForCombEvtDQ] Single
// );
#[derive(Serialize,Deserialize,Debug)]
struct Meet {
    meet_name1 : Option<String>,
    meet_location : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    meet_start : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    meet_end : Option<NaiveDateTime>,
    meet_idformat : Option<u16>,
    meet_class : Option<u16>,
    meet_meettype : Option<u16>,
    meet_course : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    enter_ages : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    enter_birthdate : bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    calc_date : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    enter_schoolyr : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    a__relaysonly : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_hometown : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_countrycode : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    scores_afterevt : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lastname_first : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    punct_names : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    punct_teams : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    win_mm : bool,
    meet_numlanes : Option<u16>,
    prelimheats_circle : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    timedfinal_circleseed : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    foreign_infinal : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    exh_infinal : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    nonconform_last : bool,
    course_order : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    seed_exhlast : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    dual_evenodd : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    strict_evenodd : bool,
    team_evenlanes : Option<u64>,
    team_oddlanes : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    masters_bytimeonly : bool,
    masters_agegrpsskip : Option<u16>,
    timer_port : Option<u16>,
    scbd_port : Option<u16>,
    timer_vendor : Option<String>,
    scbd_vendor : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_initial : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    ucase_names : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    ucase_teams : bool,
    open_senior_none : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    entryqual_faster : bool,
    indentryfee_surcharge : Option<Decimal>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    anyone_onrelay : bool,
    language_choice : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    military_time : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    check_times : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    enterkey_astab : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    double_endedsplits : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_compnumbers : bool,
    flighted_minentries : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    diffpts_malefemale : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    diffpts_eachdivision : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    scoreonly_ifexceedqualtime : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    score_fastestheatonly : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    entrylimits_warn : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    pointsbasedon_seedtime : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    pointsfor_overachievers : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    pointsfor_underachievers : bool,
    indmaxscorers_perteam : Option<u16>,
    relmaxscorers_perteam : Option<u16>,
    indtopmany_awards : Option<u16>,
    reltopmany_awards : Option<u16>,
    entrymax_total : Option<u16>,
    indmax_perath : Option<u16>,
    relmax_perath : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    foreign_getteampoints : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    include_swimupsinteamscore : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    enter_citizenof : bool,
    meet_meetstyle : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    flag_overachievers : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    flag_underachievers : bool,
    scbd_punctuation : Option<u16>,
    scbd_names : Option<u16>,
    scbd_relaynames : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    scbd_cycle : bool,
    scbd_cycleseconds : Option<u16>,
    copies_toprinter : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    report_headersonly : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    autoinc_compno : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    pentscoring_usedqtime : bool,
    swimmer_surcharge : Option<Decimal>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    directly_toprinter : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lastname_asinitial : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    under_eventname : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress__arelay : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    punct_recholders : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    ucase_recholders : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress_lsc : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    showathlete_status : bool,
    open_lowage : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    useeventsex_teamscore : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress_smallx : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    score__arelayonly : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    thirteenandover_assenior : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress_jd : bool,
    abcfinal_order : Option<String>,
    maxagefor_cfinal : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    include_sanction : bool,
    special_points : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    countrelay_alt : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_non_conforming__pool_factor : bool,
    non_conforming__pool_factor : Option<f32>,
    apnews_team : Option<String>,
    points_awarded__for_d_q : Option<f32>,
    points_awarded__for_scratch : Option<f32>,
    points_awarded__for_n_t : Option<f32>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    enter__ath_stat : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_secondclub : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    firstinitial_fulllastname : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    turnon_autobackup : bool,
    autobackup_interval : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    points_awarded__for_exh : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use__alt_team_abbr : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    is_canadian__masters : bool,
    entry_msg : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    timedfinalnonconform_last : bool,
    referee_name : Option<String>,
    referee_homphone : Option<String>,
    referee_offphone : Option<String>,
    meet_altitude : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    read__only : bool,
    masters_indlowage : Option<u16>,
    masters_rellowage : Option<u16>,
    import__dir : Option<String>,
    export__dir : Option<String>,
    backup__dir : Option<String>,
    restore_from__dir : Option<String>,
    restore_to__dir : Option<String>,
    flat_html__dir : Option<String>,
    a_p_news__dir : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    allow_same_event__dup_relay_names : bool,
    dualteam_lane1 : Option<u64>,
    dualteam_lane2 : Option<u64>,
    dualteam_lane3 : Option<u64>,
    dualteam_lane4 : Option<u64>,
    dualteam_lane5 : Option<u64>,
    dualteam_lane6 : Option<u64>,
    dualteam_lane7 : Option<u64>,
    dualteam_lane8 : Option<u64>,
    dualteam_lane9 : Option<u64>,
    dualteam_lane10 : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    strict_evenoddfastestheatonly : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    dualseeding_altunusedlane : bool,
    team_surcharge : Option<Decimal>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    display_actualentrytime : bool,
    indmaxadvance_perteam : Option<u16>,
    relmaxadvance_perteam : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    relay_names__link_by_l_s_c : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    flighted__based_on_results_time : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_year__in_place_of_age : bool,
    penalty_pts__for_n_s : Option<f32>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    entry_eligibility_date : Option<NaiveDateTime>,
    facility__surcharge : Option<Decimal>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress__time_std_abbr : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    custom__qual_times : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress__splits_for_d_qs : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress__splits_for_d_qs_relay : bool,
    d_q_codes__type : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress_times__not_meet_qual_time : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show__ageand_birth_year : bool,
    meet_state : Option<String>,
    meet_country : Option<String>,
    meet_lsc : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    b_c_s_s_a__divby_time_std : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show__hy_tek_decimals : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    suppress__results_adv_q : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    relays_as__4x1_0_0_style : bool,
    flighted_flightcount : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    flighted_incl_d_q : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    relays_alternate__two_fastest_first : bool,
    dualteam_lane11 : Option<u64>,
    dualteam_lane12 : Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    entry_deadline : Option<NaiveDateTime>,
    meet_addr1 : Option<String>,
    meet_addr2 : Option<String>,
    meet_city : Option<String>,
    meet_zip : Option<String>,
    meet_hostlsc : Option<String>,
    meet__u_s_masters_meet_i_d : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    show_first_name__over_preferred : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    exclude_n_t_entries__when_importing : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    enter_birthcentury : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    using_twopools : bool,
    pool1__name : Option<String>,
    pool2__name : Option<String>,
    indtopmany_awards_sr : Option<u16>,
    reltopmany_awards_sr : Option<u16>,
    maxforeign_infinal : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    flag__fastest_record_only : bool,
    athlete_earlysurcharge : Option<Decimal>,
    athlete_latesurcharge : Option<Decimal>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    athlete_earlysurchargedate : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    athlete_latesurchargedate : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    entry__o_m_eopendate : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    display_n_tfor__times_under5_sec : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    sort_team_combos__by_team_name : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    fastest_heat__some_lanes_do_not_score : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    rank_disabled__by_points : bool,
    special_parapoints : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    disabled_do_not__advance_to_finals : bool,
    prelimheats_circledist : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    disabled_ignore_qual_time__for_scoring : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    count_time_trial__events : bool,
    meet_header1 : Option<String>,
    meet_header2 : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    qual_non_conform_course__use_min_std : bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    entry__open_date : Option<NaiveDateTime>,
    time_adj__method : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lock__reseed : bool,
    sanction_number : Option<String>,
    last_updated : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    mixed_relays__divided_points : bool,
    relay_only__surcharge : Option<Decimal>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    disabled_seed_with_age_group__if_timed_final_super_seed : bool,
    competition__code : Option<String>,
    penalty_time_sec__for_comb_evt_d_q : Option<f32>
}
impl Meet {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Meet>, Box<dyn Error>> {
       let mut vec: Vec<Meet> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Meet = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Meet>, Error> {
        let mut vec: Vec<Meet> = Vec::new();
        match conn.execute("SELECT * FROM meet", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let meet_name1 = read_string(_batch.at(0, row_index));
                        let meet_location = read_string(_batch.at(1, row_index));
                        let meet_start = read_datetime(_batch.at(2, row_index));
                        let meet_end = read_datetime(_batch.at(3, row_index));
                        let meet_idformat = read_u16(_batch.at(4, row_index));
                        let meet_class = read_u16(_batch.at(5, row_index));
                        let meet_meettype = read_u16(_batch.at(6, row_index));
                        let meet_course = read_u16(_batch.at(7, row_index));
                        let enter_ages = read_bool(_batch.at(8, row_index));
                        let enter_birthdate = read_bool(_batch.at(9, row_index));
                        let calc_date = read_datetime(_batch.at(10, row_index));
                        let enter_schoolyr = read_bool(_batch.at(11, row_index));
                        let a__relaysonly = read_bool(_batch.at(12, row_index));
                        let use_hometown = read_bool(_batch.at(13, row_index));
                        let show_countrycode = read_bool(_batch.at(14, row_index));
                        let scores_afterevt = read_bool(_batch.at(15, row_index));
                        let lastname_first = read_bool(_batch.at(16, row_index));
                        let punct_names = read_bool(_batch.at(17, row_index));
                        let punct_teams = read_bool(_batch.at(18, row_index));
                        let win_mm = read_bool(_batch.at(19, row_index));
                        let meet_numlanes = read_u16(_batch.at(20, row_index));
                        let prelimheats_circle = read_u16(_batch.at(21, row_index));
                        let timedfinal_circleseed = read_bool(_batch.at(22, row_index));
                        let foreign_infinal = read_bool(_batch.at(23, row_index));
                        let exh_infinal = read_bool(_batch.at(24, row_index));
                        let nonconform_last = read_bool(_batch.at(25, row_index));
                        let course_order = read_string(_batch.at(26, row_index));
                        let seed_exhlast = read_bool(_batch.at(27, row_index));
                        let dual_evenodd = read_bool(_batch.at(28, row_index));
                        let strict_evenodd = read_bool(_batch.at(29, row_index));
                        let team_evenlanes = read_u64(_batch.at(30, row_index));
                        let team_oddlanes = read_u64(_batch.at(31, row_index));
                        let masters_bytimeonly = read_bool(_batch.at(32, row_index));
                        let masters_agegrpsskip = read_u16(_batch.at(33, row_index));
                        let timer_port = read_u16(_batch.at(34, row_index));
                        let scbd_port = read_u16(_batch.at(35, row_index));
                        let timer_vendor = read_string(_batch.at(36, row_index));
                        let scbd_vendor = read_string(_batch.at(37, row_index));
                        let show_initial = read_bool(_batch.at(38, row_index));
                        let ucase_names = read_bool(_batch.at(39, row_index));
                        let ucase_teams = read_bool(_batch.at(40, row_index));
                        let open_senior_none = read_string(_batch.at(41, row_index));
                        let entryqual_faster = read_bool(_batch.at(42, row_index));
                        let indentryfee_surcharge = read_decimal(_batch.at(43, row_index));
                        let anyone_onrelay = read_bool(_batch.at(44, row_index));
                        let language_choice = read_string(_batch.at(45, row_index));
                        let military_time = read_bool(_batch.at(46, row_index));
                        let check_times = read_bool(_batch.at(47, row_index));
                        let enterkey_astab = read_bool(_batch.at(48, row_index));
                        let double_endedsplits = read_bool(_batch.at(49, row_index));
                        let use_compnumbers = read_bool(_batch.at(50, row_index));
                        let flighted_minentries = read_u16(_batch.at(51, row_index));
                        let diffpts_malefemale = read_bool(_batch.at(52, row_index));
                        let diffpts_eachdivision = read_bool(_batch.at(53, row_index));
                        let scoreonly_ifexceedqualtime = read_bool(_batch.at(54, row_index));
                        let score_fastestheatonly = read_bool(_batch.at(55, row_index));
                        let entrylimits_warn = read_bool(_batch.at(56, row_index));
                        let pointsbasedon_seedtime = read_bool(_batch.at(57, row_index));
                        let pointsfor_overachievers = read_bool(_batch.at(58, row_index));
                        let pointsfor_underachievers = read_bool(_batch.at(59, row_index));
                        let indmaxscorers_perteam = read_u16(_batch.at(60, row_index));
                        let relmaxscorers_perteam = read_u16(_batch.at(61, row_index));
                        let indtopmany_awards = read_u16(_batch.at(62, row_index));
                        let reltopmany_awards = read_u16(_batch.at(63, row_index));
                        let entrymax_total = read_u16(_batch.at(64, row_index));
                        let indmax_perath = read_u16(_batch.at(65, row_index));
                        let relmax_perath = read_u16(_batch.at(66, row_index));
                        let foreign_getteampoints = read_bool(_batch.at(67, row_index));
                        let include_swimupsinteamscore = read_bool(_batch.at(68, row_index));
                        let enter_citizenof = read_bool(_batch.at(69, row_index));
                        let meet_meetstyle = read_u16(_batch.at(70, row_index));
                        let flag_overachievers = read_bool(_batch.at(71, row_index));
                        let flag_underachievers = read_bool(_batch.at(72, row_index));
                        let scbd_punctuation = read_u16(_batch.at(73, row_index));
                        let scbd_names = read_u16(_batch.at(74, row_index));
                        let scbd_relaynames = read_u16(_batch.at(75, row_index));
                        let scbd_cycle = read_bool(_batch.at(76, row_index));
                        let scbd_cycleseconds = read_u16(_batch.at(77, row_index));
                        let copies_toprinter = read_u16(_batch.at(78, row_index));
                        let report_headersonly = read_bool(_batch.at(79, row_index));
                        let autoinc_compno = read_bool(_batch.at(80, row_index));
                        let pentscoring_usedqtime = read_bool(_batch.at(81, row_index));
                        let swimmer_surcharge = read_decimal(_batch.at(82, row_index));
                        let directly_toprinter = read_bool(_batch.at(83, row_index));
                        let lastname_asinitial = read_bool(_batch.at(84, row_index));
                        let under_eventname = read_bool(_batch.at(85, row_index));
                        let suppress__arelay = read_bool(_batch.at(86, row_index));
                        let punct_recholders = read_bool(_batch.at(87, row_index));
                        let ucase_recholders = read_bool(_batch.at(88, row_index));
                        let suppress_lsc = read_bool(_batch.at(89, row_index));
                        let showathlete_status = read_bool(_batch.at(90, row_index));
                        let open_lowage = read_u16(_batch.at(91, row_index));
                        let useeventsex_teamscore = read_bool(_batch.at(92, row_index));
                        let suppress_smallx = read_bool(_batch.at(93, row_index));
                        let score__arelayonly = read_bool(_batch.at(94, row_index));
                        let thirteenandover_assenior = read_bool(_batch.at(95, row_index));
                        let suppress_jd = read_bool(_batch.at(96, row_index));
                        let abcfinal_order = read_string(_batch.at(97, row_index));
                        let maxagefor_cfinal = read_u16(_batch.at(98, row_index));
                        let include_sanction = read_bool(_batch.at(99, row_index));
                        let special_points = read_u16(_batch.at(100, row_index));
                        let countrelay_alt = read_bool(_batch.at(101, row_index));
                        let use_non_conforming__pool_factor = read_bool(_batch.at(102, row_index));
                        let non_conforming__pool_factor = read_f32(_batch.at(103, row_index));
                        let apnews_team = read_string(_batch.at(104, row_index));
                        let points_awarded__for_d_q = read_f32(_batch.at(105, row_index));
                        let points_awarded__for_scratch = read_f32(_batch.at(106, row_index));
                        let points_awarded__for_n_t = read_f32(_batch.at(107, row_index));
                        let enter__ath_stat = read_bool(_batch.at(108, row_index));
                        let show_secondclub = read_bool(_batch.at(109, row_index));
                        let firstinitial_fulllastname = read_bool(_batch.at(110, row_index));
                        let turnon_autobackup = read_bool(_batch.at(111, row_index));
                        let autobackup_interval = read_u16(_batch.at(112, row_index));
                        let points_awarded__for_exh = read_bool(_batch.at(113, row_index));
                        let use__alt_team_abbr = read_bool(_batch.at(114, row_index));
                        let is_canadian__masters = read_bool(_batch.at(115, row_index));
                        let entry_msg = read_string(_batch.at(116, row_index));
                        let timedfinalnonconform_last = read_bool(_batch.at(117, row_index));
                        let referee_name = read_string(_batch.at(118, row_index));
                        let referee_homphone = read_string(_batch.at(119, row_index));
                        let referee_offphone = read_string(_batch.at(120, row_index));
                        let meet_altitude = read_u64(_batch.at(121, row_index));
                        let read__only = read_bool(_batch.at(122, row_index));
                        let masters_indlowage = read_u16(_batch.at(123, row_index));
                        let masters_rellowage = read_u16(_batch.at(124, row_index));
                        let import__dir = read_string(_batch.at(125, row_index));
                        let export__dir = read_string(_batch.at(126, row_index));
                        let backup__dir = read_string(_batch.at(127, row_index));
                        let restore_from__dir = read_string(_batch.at(128, row_index));
                        let restore_to__dir = read_string(_batch.at(129, row_index));
                        let flat_html__dir = read_string(_batch.at(130, row_index));
                        let a_p_news__dir = read_string(_batch.at(131, row_index));
                        let allow_same_event__dup_relay_names = read_bool(_batch.at(132, row_index));
                        let dualteam_lane1 = read_u64(_batch.at(133, row_index));
                        let dualteam_lane2 = read_u64(_batch.at(134, row_index));
                        let dualteam_lane3 = read_u64(_batch.at(135, row_index));
                        let dualteam_lane4 = read_u64(_batch.at(136, row_index));
                        let dualteam_lane5 = read_u64(_batch.at(137, row_index));
                        let dualteam_lane6 = read_u64(_batch.at(138, row_index));
                        let dualteam_lane7 = read_u64(_batch.at(139, row_index));
                        let dualteam_lane8 = read_u64(_batch.at(140, row_index));
                        let dualteam_lane9 = read_u64(_batch.at(141, row_index));
                        let dualteam_lane10 = read_u64(_batch.at(142, row_index));
                        let strict_evenoddfastestheatonly = read_bool(_batch.at(143, row_index));
                        let dualseeding_altunusedlane = read_bool(_batch.at(144, row_index));
                        let team_surcharge = read_decimal(_batch.at(145, row_index));
                        let display_actualentrytime = read_bool(_batch.at(146, row_index));
                        let indmaxadvance_perteam = read_u16(_batch.at(147, row_index));
                        let relmaxadvance_perteam = read_u16(_batch.at(148, row_index));
                        let relay_names__link_by_l_s_c = read_bool(_batch.at(149, row_index));
                        let flighted__based_on_results_time = read_bool(_batch.at(150, row_index));
                        let show_year__in_place_of_age = read_bool(_batch.at(151, row_index));
                        let penalty_pts__for_n_s = read_f32(_batch.at(152, row_index));
                        let entry_eligibility_date = read_datetime(_batch.at(153, row_index));
                        let facility__surcharge = read_decimal(_batch.at(154, row_index));
                        let suppress__time_std_abbr = read_bool(_batch.at(155, row_index));
                        let custom__qual_times = read_bool(_batch.at(156, row_index));
                        let suppress__splits_for_d_qs = read_bool(_batch.at(157, row_index));
                        let suppress__splits_for_d_qs_relay = read_bool(_batch.at(158, row_index));
                        let d_q_codes__type = read_string(_batch.at(159, row_index));
                        let suppress_times__not_meet_qual_time = read_bool(_batch.at(160, row_index));
                        let show__ageand_birth_year = read_bool(_batch.at(161, row_index));
                        let meet_state = read_string(_batch.at(162, row_index));
                        let meet_country = read_string(_batch.at(163, row_index));
                        let meet_lsc = read_string(_batch.at(164, row_index));
                        let b_c_s_s_a__divby_time_std = read_bool(_batch.at(165, row_index));
                        let show__hy_tek_decimals = read_bool(_batch.at(166, row_index));
                        let suppress__results_adv_q = read_bool(_batch.at(167, row_index));
                        let relays_as__4x1_0_0_style = read_bool(_batch.at(168, row_index));
                        let flighted_flightcount = read_u16(_batch.at(169, row_index));
                        let flighted_incl_d_q = read_bool(_batch.at(170, row_index));
                        let relays_alternate__two_fastest_first = read_bool(_batch.at(171, row_index));
                        let dualteam_lane11 = read_u64(_batch.at(172, row_index));
                        let dualteam_lane12 = read_u64(_batch.at(173, row_index));
                        let entry_deadline = read_datetime(_batch.at(174, row_index));
                        let meet_addr1 = read_string(_batch.at(175, row_index));
                        let meet_addr2 = read_string(_batch.at(176, row_index));
                        let meet_city = read_string(_batch.at(177, row_index));
                        let meet_zip = read_string(_batch.at(178, row_index));
                        let meet_hostlsc = read_string(_batch.at(179, row_index));
                        let meet__u_s_masters_meet_i_d = read_string(_batch.at(180, row_index));
                        let show_first_name__over_preferred = read_bool(_batch.at(181, row_index));
                        let exclude_n_t_entries__when_importing = read_bool(_batch.at(182, row_index));
                        let enter_birthcentury = read_bool(_batch.at(183, row_index));
                        let using_twopools = read_bool(_batch.at(184, row_index));
                        let pool1__name = read_string(_batch.at(185, row_index));
                        let pool2__name = read_string(_batch.at(186, row_index));
                        let indtopmany_awards_sr = read_u16(_batch.at(187, row_index));
                        let reltopmany_awards_sr = read_u16(_batch.at(188, row_index));
                        let maxforeign_infinal = read_u16(_batch.at(189, row_index));
                        let flag__fastest_record_only = read_bool(_batch.at(190, row_index));
                        let athlete_earlysurcharge = read_decimal(_batch.at(191, row_index));
                        let athlete_latesurcharge = read_decimal(_batch.at(192, row_index));
                        let athlete_earlysurchargedate = read_datetime(_batch.at(193, row_index));
                        let athlete_latesurchargedate = read_datetime(_batch.at(194, row_index));
                        let entry__o_m_eopendate = read_datetime(_batch.at(195, row_index));
                        let display_n_tfor__times_under5_sec = read_bool(_batch.at(196, row_index));
                        let sort_team_combos__by_team_name = read_bool(_batch.at(197, row_index));
                        let fastest_heat__some_lanes_do_not_score = read_bool(_batch.at(198, row_index));
                        let rank_disabled__by_points = read_bool(_batch.at(199, row_index));
                        let special_parapoints = read_u16(_batch.at(200, row_index));
                        let disabled_do_not__advance_to_finals = read_bool(_batch.at(201, row_index));
                        let prelimheats_circledist = read_u16(_batch.at(202, row_index));
                        let disabled_ignore_qual_time__for_scoring = read_bool(_batch.at(203, row_index));
                        let count_time_trial__events = read_bool(_batch.at(204, row_index));
                        let meet_header1 = read_string(_batch.at(205, row_index));
                        let meet_header2 = read_string(_batch.at(206, row_index));
                        let qual_non_conform_course__use_min_std = read_bool(_batch.at(207, row_index));
                        let entry__open_date = read_datetime(_batch.at(208, row_index));
                        let time_adj__method = read_string(_batch.at(209, row_index));
                        let lock__reseed = read_bool(_batch.at(210, row_index));
                        let sanction_number = read_string(_batch.at(211, row_index));
                        let last_updated = read_string(_batch.at(212, row_index));
                        let mixed_relays__divided_points = read_bool(_batch.at(213, row_index));
                        let relay_only__surcharge = read_decimal(_batch.at(214, row_index));
                        let disabled_seed_with_age_group__if_timed_final_super_seed = read_bool(_batch.at(215, row_index));
                        let competition__code = read_string(_batch.at(216, row_index));
                        let penalty_time_sec__for_comb_evt_d_q = read_f32(_batch.at(217, row_index));
                        let obj = Meet {
                            meet_name1,
                            meet_location,
                            meet_start,
                            meet_end,
                            meet_idformat,
                            meet_class,
                            meet_meettype,
                            meet_course,
                            enter_ages,
                            enter_birthdate,
                            calc_date,
                            enter_schoolyr,
                            a__relaysonly,
                            use_hometown,
                            show_countrycode,
                            scores_afterevt,
                            lastname_first,
                            punct_names,
                            punct_teams,
                            win_mm,
                            meet_numlanes,
                            prelimheats_circle,
                            timedfinal_circleseed,
                            foreign_infinal,
                            exh_infinal,
                            nonconform_last,
                            course_order,
                            seed_exhlast,
                            dual_evenodd,
                            strict_evenodd,
                            team_evenlanes,
                            team_oddlanes,
                            masters_bytimeonly,
                            masters_agegrpsskip,
                            timer_port,
                            scbd_port,
                            timer_vendor,
                            scbd_vendor,
                            show_initial,
                            ucase_names,
                            ucase_teams,
                            open_senior_none,
                            entryqual_faster,
                            indentryfee_surcharge,
                            anyone_onrelay,
                            language_choice,
                            military_time,
                            check_times,
                            enterkey_astab,
                            double_endedsplits,
                            use_compnumbers,
                            flighted_minentries,
                            diffpts_malefemale,
                            diffpts_eachdivision,
                            scoreonly_ifexceedqualtime,
                            score_fastestheatonly,
                            entrylimits_warn,
                            pointsbasedon_seedtime,
                            pointsfor_overachievers,
                            pointsfor_underachievers,
                            indmaxscorers_perteam,
                            relmaxscorers_perteam,
                            indtopmany_awards,
                            reltopmany_awards,
                            entrymax_total,
                            indmax_perath,
                            relmax_perath,
                            foreign_getteampoints,
                            include_swimupsinteamscore,
                            enter_citizenof,
                            meet_meetstyle,
                            flag_overachievers,
                            flag_underachievers,
                            scbd_punctuation,
                            scbd_names,
                            scbd_relaynames,
                            scbd_cycle,
                            scbd_cycleseconds,
                            copies_toprinter,
                            report_headersonly,
                            autoinc_compno,
                            pentscoring_usedqtime,
                            swimmer_surcharge,
                            directly_toprinter,
                            lastname_asinitial,
                            under_eventname,
                            suppress__arelay,
                            punct_recholders,
                            ucase_recholders,
                            suppress_lsc,
                            showathlete_status,
                            open_lowage,
                            useeventsex_teamscore,
                            suppress_smallx,
                            score__arelayonly,
                            thirteenandover_assenior,
                            suppress_jd,
                            abcfinal_order,
                            maxagefor_cfinal,
                            include_sanction,
                            special_points,
                            countrelay_alt,
                            use_non_conforming__pool_factor,
                            non_conforming__pool_factor,
                            apnews_team,
                            points_awarded__for_d_q,
                            points_awarded__for_scratch,
                            points_awarded__for_n_t,
                            enter__ath_stat,
                            show_secondclub,
                            firstinitial_fulllastname,
                            turnon_autobackup,
                            autobackup_interval,
                            points_awarded__for_exh,
                            use__alt_team_abbr,
                            is_canadian__masters,
                            entry_msg,
                            timedfinalnonconform_last,
                            referee_name,
                            referee_homphone,
                            referee_offphone,
                            meet_altitude,
                            read__only,
                            masters_indlowage,
                            masters_rellowage,
                            import__dir,
                            export__dir,
                            backup__dir,
                            restore_from__dir,
                            restore_to__dir,
                            flat_html__dir,
                            a_p_news__dir,
                            allow_same_event__dup_relay_names,
                            dualteam_lane1,
                            dualteam_lane2,
                            dualteam_lane3,
                            dualteam_lane4,
                            dualteam_lane5,
                            dualteam_lane6,
                            dualteam_lane7,
                            dualteam_lane8,
                            dualteam_lane9,
                            dualteam_lane10,
                            strict_evenoddfastestheatonly,
                            dualseeding_altunusedlane,
                            team_surcharge,
                            display_actualentrytime,
                            indmaxadvance_perteam,
                            relmaxadvance_perteam,
                            relay_names__link_by_l_s_c,
                            flighted__based_on_results_time,
                            show_year__in_place_of_age,
                            penalty_pts__for_n_s,
                            entry_eligibility_date,
                            facility__surcharge,
                            suppress__time_std_abbr,
                            custom__qual_times,
                            suppress__splits_for_d_qs,
                            suppress__splits_for_d_qs_relay,
                            d_q_codes__type,
                            suppress_times__not_meet_qual_time,
                            show__ageand_birth_year,
                            meet_state,
                            meet_country,
                            meet_lsc,
                            b_c_s_s_a__divby_time_std,
                            show__hy_tek_decimals,
                            suppress__results_adv_q,
                            relays_as__4x1_0_0_style,
                            flighted_flightcount,
                            flighted_incl_d_q,
                            relays_alternate__two_fastest_first,
                            dualteam_lane11,
                            dualteam_lane12,
                            entry_deadline,
                            meet_addr1,
                            meet_addr2,
                            meet_city,
                            meet_zip,
                            meet_hostlsc,
                            meet__u_s_masters_meet_i_d,
                            show_first_name__over_preferred,
                            exclude_n_t_entries__when_importing,
                            enter_birthcentury,
                            using_twopools,
                            pool1__name,
                            pool2__name,
                            indtopmany_awards_sr,
                            reltopmany_awards_sr,
                            maxforeign_infinal,
                            flag__fastest_record_only,
                            athlete_earlysurcharge,
                            athlete_latesurcharge,
                            athlete_earlysurchargedate,
                            athlete_latesurchargedate,
                            entry__o_m_eopendate,
                            display_n_tfor__times_under5_sec,
                            sort_team_combos__by_team_name,
                            fastest_heat__some_lanes_do_not_score,
                            rank_disabled__by_points,
                            special_parapoints,
                            disabled_do_not__advance_to_finals,
                            prelimheats_circledist,
                            disabled_ignore_qual_time__for_scoring,
                            count_time_trial__events,
                            meet_header1,
                            meet_header2,
                            qual_non_conform_course__use_min_std,
                            entry__open_date,
                            time_adj__method,
                            lock__reseed,
                            sanction_number,
                            last_updated,
                            mixed_relays__divided_points,
                            relay_only__surcharge,
                            disabled_seed_with_age_group__if_timed_final_super_seed,
                            competition__code,
                            penalty_time_sec__for_comb_evt_d_q
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
