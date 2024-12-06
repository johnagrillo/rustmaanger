// 
// 
// CREATE TABLE [Team]
// (
// [Team_no] Long Integer,
// [Team_name] Text (60),
// [Team_short] Text (32),
// [Team_abbr] Text (10),
// [Team_stat] Text (2),
// [Team_div] Integer,
// [Team_region] Integer,
// [Team_head] Text (60),
// [Team_asst] Text (60),
// [Team_addr1] Text (60),
// [Team_addr2] Text (60),
// [Team_city] Text (60),
// [Team_prov] Text (60),
// [Team_statenew] Text (6),
// [Team_zip] Text (20),
// [Team_cntry] Text (6),
// [Team_daytele] Text (40),
// [Team_evetele] Text (40),
// [Team_faxtele] Text (40),
// [Team_email] Text (72),
// [Team_c3] Text (60),
// [Team_c4] Text (60),
// [Team_c5] Text (60),
// [Team_c6] Text (60),
// [Team_c7] Text (60),
// [Team_c8] Text (60),
// [Team_c9] Text (60),
// [Team_c10] Text (60),
// [Team_altabbr] Text (10),
// [Team_NoPoints] Boolean NOT NULL,
// [Team_Selected] Boolean NOT NULL,
// [Team_altname] Text (32),
// [InvitedTeam_AgencyID] Long Integer,
// [InvitedTeam_Email] Text (110),
// [Invited_InviteCode] Text (30),
// [InvitedEntry_status] Text (50),
// [InvitedPayment_status] Text (50),
// [Invited_notes] Text (510),
// [Invited_GovBody] Text (100),
// [Invited_TeamID] Long Integer,
// [InvitedHas_notes] Boolean NOT NULL,
// [InvitedLeague_name] Text (50),
// [InvitedFile_status] Text (50),
// [InvitedFile_importdatetime] Text (50),
// [InvitedFile_TMuploaddatetime] Text (50),
// [InvitedFile_source] Text (50),
// [Invited_TeamEntryID] Long Integer,
// [Invited_Athletecount] Text (30),
// [team_lsc] Text (6),
// [NoTeam_surcharge] Boolean NOT NULL,
// [NoFacility_surcharge] Boolean NOT NULL,
// [NoAthlete_surcharge] Boolean NOT NULL,
// [Team_cell] Text (40),
// [NoRelayOnly_surcharge] Boolean NOT NULL,
// [Team_Code] Text (60),
// [Team_Gender] Text (2)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Team {
    team_no : Option<u64>,
    team_name : Option<String>,
    team_short : Option<String>,
    team_abbr : Option<String>,
    team_stat : Option<String>,
    team_div : Option<u16>,
    team_region : Option<u16>,
    team_head : Option<String>,
    team_asst : Option<String>,
    team_addr1 : Option<String>,
    team_addr2 : Option<String>,
    team_city : Option<String>,
    team_prov : Option<String>,
    team_statenew : Option<String>,
    team_zip : Option<String>,
    team_cntry : Option<String>,
    team_daytele : Option<String>,
    team_evetele : Option<String>,
    team_faxtele : Option<String>,
    team_email : Option<String>,
    team_c3 : Option<String>,
    team_c4 : Option<String>,
    team_c5 : Option<String>,
    team_c6 : Option<String>,
    team_c7 : Option<String>,
    team_c8 : Option<String>,
    team_c9 : Option<String>,
    team_c1_0 : Option<String>,
    team_altabbr : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    team__no_points : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    team__selected : bool,
    team_altname : Option<String>,
    invited_team__agency_i_d : Option<u64>,
    invited_team__email : Option<String>,
    invited__invite_code : Option<String>,
    invited_entry_status : Option<String>,
    invited_payment_status : Option<String>,
    invited_notes : Option<String>,
    invited__gov_body : Option<String>,
    invited__team_i_d : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    invited_has_notes : bool,
    invited_league_name : Option<String>,
    invited_file_status : Option<String>,
    invited_file_importdatetime : Option<String>,
    invited_file__t_muploaddatetime : Option<String>,
    invited_file_source : Option<String>,
    invited__team_entry_i_d : Option<u64>,
    invited__athletecount : Option<String>,
    team_lsc : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    no_team_surcharge : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    no_facility_surcharge : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    no_athlete_surcharge : bool,
    team_cell : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    no_relay_only_surcharge : bool,
    team__code : Option<String>,
    team__gender : Option<String>
}
impl Team {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Team>, Box<dyn Error>> {
       let mut vec: Vec<Team> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Team = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Team>, Error> {
        let mut vec: Vec<Team> = Vec::new();
        match conn.execute("SELECT * FROM team", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let team_no = read_u64(_batch.at(0, row_index));
                        let team_name = read_string(_batch.at(1, row_index));
                        let team_short = read_string(_batch.at(2, row_index));
                        let team_abbr = read_string(_batch.at(3, row_index));
                        let team_stat = read_string(_batch.at(4, row_index));
                        let team_div = read_u16(_batch.at(5, row_index));
                        let team_region = read_u16(_batch.at(6, row_index));
                        let team_head = read_string(_batch.at(7, row_index));
                        let team_asst = read_string(_batch.at(8, row_index));
                        let team_addr1 = read_string(_batch.at(9, row_index));
                        let team_addr2 = read_string(_batch.at(10, row_index));
                        let team_city = read_string(_batch.at(11, row_index));
                        let team_prov = read_string(_batch.at(12, row_index));
                        let team_statenew = read_string(_batch.at(13, row_index));
                        let team_zip = read_string(_batch.at(14, row_index));
                        let team_cntry = read_string(_batch.at(15, row_index));
                        let team_daytele = read_string(_batch.at(16, row_index));
                        let team_evetele = read_string(_batch.at(17, row_index));
                        let team_faxtele = read_string(_batch.at(18, row_index));
                        let team_email = read_string(_batch.at(19, row_index));
                        let team_c3 = read_string(_batch.at(20, row_index));
                        let team_c4 = read_string(_batch.at(21, row_index));
                        let team_c5 = read_string(_batch.at(22, row_index));
                        let team_c6 = read_string(_batch.at(23, row_index));
                        let team_c7 = read_string(_batch.at(24, row_index));
                        let team_c8 = read_string(_batch.at(25, row_index));
                        let team_c9 = read_string(_batch.at(26, row_index));
                        let team_c1_0 = read_string(_batch.at(27, row_index));
                        let team_altabbr = read_string(_batch.at(28, row_index));
                        let team__no_points = read_bool(_batch.at(29, row_index));
                        let team__selected = read_bool(_batch.at(30, row_index));
                        let team_altname = read_string(_batch.at(31, row_index));
                        let invited_team__agency_i_d = read_u64(_batch.at(32, row_index));
                        let invited_team__email = read_string(_batch.at(33, row_index));
                        let invited__invite_code = read_string(_batch.at(34, row_index));
                        let invited_entry_status = read_string(_batch.at(35, row_index));
                        let invited_payment_status = read_string(_batch.at(36, row_index));
                        let invited_notes = read_string(_batch.at(37, row_index));
                        let invited__gov_body = read_string(_batch.at(38, row_index));
                        let invited__team_i_d = read_u64(_batch.at(39, row_index));
                        let invited_has_notes = read_bool(_batch.at(40, row_index));
                        let invited_league_name = read_string(_batch.at(41, row_index));
                        let invited_file_status = read_string(_batch.at(42, row_index));
                        let invited_file_importdatetime = read_string(_batch.at(43, row_index));
                        let invited_file__t_muploaddatetime = read_string(_batch.at(44, row_index));
                        let invited_file_source = read_string(_batch.at(45, row_index));
                        let invited__team_entry_i_d = read_u64(_batch.at(46, row_index));
                        let invited__athletecount = read_string(_batch.at(47, row_index));
                        let team_lsc = read_string(_batch.at(48, row_index));
                        let no_team_surcharge = read_bool(_batch.at(49, row_index));
                        let no_facility_surcharge = read_bool(_batch.at(50, row_index));
                        let no_athlete_surcharge = read_bool(_batch.at(51, row_index));
                        let team_cell = read_string(_batch.at(52, row_index));
                        let no_relay_only_surcharge = read_bool(_batch.at(53, row_index));
                        let team__code = read_string(_batch.at(54, row_index));
                        let team__gender = read_string(_batch.at(55, row_index));
                        let obj = Team {
                            team_no,
                            team_name,
                            team_short,
                            team_abbr,
                            team_stat,
                            team_div,
                            team_region,
                            team_head,
                            team_asst,
                            team_addr1,
                            team_addr2,
                            team_city,
                            team_prov,
                            team_statenew,
                            team_zip,
                            team_cntry,
                            team_daytele,
                            team_evetele,
                            team_faxtele,
                            team_email,
                            team_c3,
                            team_c4,
                            team_c5,
                            team_c6,
                            team_c7,
                            team_c8,
                            team_c9,
                            team_c1_0,
                            team_altabbr,
                            team__no_points,
                            team__selected,
                            team_altname,
                            invited_team__agency_i_d,
                            invited_team__email,
                            invited__invite_code,
                            invited_entry_status,
                            invited_payment_status,
                            invited_notes,
                            invited__gov_body,
                            invited__team_i_d,
                            invited_has_notes,
                            invited_league_name,
                            invited_file_status,
                            invited_file_importdatetime,
                            invited_file__t_muploaddatetime,
                            invited_file_source,
                            invited__team_entry_i_d,
                            invited__athletecount,
                            team_lsc,
                            no_team_surcharge,
                            no_facility_surcharge,
                            no_athlete_surcharge,
                            team_cell,
                            no_relay_only_surcharge,
                            team__code,
                            team__gender
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
