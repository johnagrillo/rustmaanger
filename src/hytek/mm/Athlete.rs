// 
// 
// CREATE TABLE [Athlete]
// (
// [Ath_no] Long Integer,
// [Last_name] Text (40),
// [First_name] Text (40),
// [Initial] Text (2),
// [Ath_Sex] Text (2),
// [Birth_date] DateTime,
// [Team_no] Long Integer,
// [Schl_yr] Text (4),
// [Ath_age] Integer,
// [Reg_no] Text (28),
// [Ath_stat] Text (2),
// [Div_no] Long Integer,
// [Comp_no] Long Integer,
// [Pref_name] Text (40),
// [Home_addr1] Text (60),
// [Home_addr2] Text (60),
// [Home_city] Text (60),
// [Home_prov] Text (60),
// [Home_statenew] Text (6),
// [Home_zip] Text (20),
// [Home_cntry] Text (6),
// [Home_daytele] Text (40),
// [Home_evetele] Text (40),
// [Home_faxtele] Text (40),
// [Citizen_of] Text (6),
// [Picture_bmp] Text (60),
// [second_club] Text (32),
// [home_email] Text (100),
// [Home_celltele] Text (40),
// [bcssa_type] Text (4),
// [Home_emergcontact] Text (60),
// [Home_emergtele] Text (40),
// [Disab_Scode] Integer,
// [Disab_SBcode] Integer,
// [Disab_SMcode] Integer,
// [Disab_SDMSID] Text (14),
// [Disab_Exeptioncodes] Text (28),
// [Masters_RegVerified] Boolean NOT NULL,
// [PC_Hide] Boolean NOT NULL,
// [Ath_Sex_BS] Text (2)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Athlete {
    ath_no : Option<u64>,
    last_name : Option<String>,
    first_name : Option<String>,
    initial : Option<String>,
    ath__sex : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    birth_date : Option<NaiveDateTime>,
    team_no : Option<u64>,
    schl_yr : Option<String>,
    ath_age : Option<u16>,
    reg_no : Option<String>,
    ath_stat : Option<String>,
    div_no : Option<u64>,
    comp_no : Option<u64>,
    pref_name : Option<String>,
    home_addr1 : Option<String>,
    home_addr2 : Option<String>,
    home_city : Option<String>,
    home_prov : Option<String>,
    home_statenew : Option<String>,
    home_zip : Option<String>,
    home_cntry : Option<String>,
    home_daytele : Option<String>,
    home_evetele : Option<String>,
    home_faxtele : Option<String>,
    citizen_of : Option<String>,
    picture_bmp : Option<String>,
    second_club : Option<String>,
    home_email : Option<String>,
    home_celltele : Option<String>,
    bcssa_type : Option<String>,
    home_emergcontact : Option<String>,
    home_emergtele : Option<String>,
    disab__scode : Option<u16>,
    disab__s_bcode : Option<u16>,
    disab__s_mcode : Option<u16>,
    disab__s_d_m_s_i_d : Option<String>,
    disab__exeptioncodes : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    masters__reg_verified : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    p_c__hide : bool,
    ath__sex__b_s : Option<String>
}
impl Athlete {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Athlete>, Box<dyn Error>> {
       let mut vec: Vec<Athlete> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Athlete = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Athlete>, Error> {
        let mut vec: Vec<Athlete> = Vec::new();
        match conn.execute("SELECT * FROM athlete", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let ath_no = read_u64(_batch.at(0, row_index));
                        let last_name = read_string(_batch.at(1, row_index));
                        let first_name = read_string(_batch.at(2, row_index));
                        let initial = read_string(_batch.at(3, row_index));
                        let ath__sex = read_string(_batch.at(4, row_index));
                        let birth_date = read_datetime(_batch.at(5, row_index));
                        let team_no = read_u64(_batch.at(6, row_index));
                        let schl_yr = read_string(_batch.at(7, row_index));
                        let ath_age = read_u16(_batch.at(8, row_index));
                        let reg_no = read_string(_batch.at(9, row_index));
                        let ath_stat = read_string(_batch.at(10, row_index));
                        let div_no = read_u64(_batch.at(11, row_index));
                        let comp_no = read_u64(_batch.at(12, row_index));
                        let pref_name = read_string(_batch.at(13, row_index));
                        let home_addr1 = read_string(_batch.at(14, row_index));
                        let home_addr2 = read_string(_batch.at(15, row_index));
                        let home_city = read_string(_batch.at(16, row_index));
                        let home_prov = read_string(_batch.at(17, row_index));
                        let home_statenew = read_string(_batch.at(18, row_index));
                        let home_zip = read_string(_batch.at(19, row_index));
                        let home_cntry = read_string(_batch.at(20, row_index));
                        let home_daytele = read_string(_batch.at(21, row_index));
                        let home_evetele = read_string(_batch.at(22, row_index));
                        let home_faxtele = read_string(_batch.at(23, row_index));
                        let citizen_of = read_string(_batch.at(24, row_index));
                        let picture_bmp = read_string(_batch.at(25, row_index));
                        let second_club = read_string(_batch.at(26, row_index));
                        let home_email = read_string(_batch.at(27, row_index));
                        let home_celltele = read_string(_batch.at(28, row_index));
                        let bcssa_type = read_string(_batch.at(29, row_index));
                        let home_emergcontact = read_string(_batch.at(30, row_index));
                        let home_emergtele = read_string(_batch.at(31, row_index));
                        let disab__scode = read_u16(_batch.at(32, row_index));
                        let disab__s_bcode = read_u16(_batch.at(33, row_index));
                        let disab__s_mcode = read_u16(_batch.at(34, row_index));
                        let disab__s_d_m_s_i_d = read_string(_batch.at(35, row_index));
                        let disab__exeptioncodes = read_string(_batch.at(36, row_index));
                        let masters__reg_verified = read_bool(_batch.at(37, row_index));
                        let p_c__hide = read_bool(_batch.at(38, row_index));
                        let ath__sex__b_s = read_string(_batch.at(39, row_index));
                        let obj = Athlete {
                            ath_no,
                            last_name,
                            first_name,
                            initial,
                            ath__sex,
                            birth_date,
                            team_no,
                            schl_yr,
                            ath_age,
                            reg_no,
                            ath_stat,
                            div_no,
                            comp_no,
                            pref_name,
                            home_addr1,
                            home_addr2,
                            home_city,
                            home_prov,
                            home_statenew,
                            home_zip,
                            home_cntry,
                            home_daytele,
                            home_evetele,
                            home_faxtele,
                            citizen_of,
                            picture_bmp,
                            second_club,
                            home_email,
                            home_celltele,
                            bcssa_type,
                            home_emergcontact,
                            home_emergtele,
                            disab__scode,
                            disab__s_bcode,
                            disab__s_mcode,
                            disab__s_d_m_s_i_d,
                            disab__exeptioncodes,
                            masters__reg_verified,
                            p_c__hide,
                            ath__sex__b_s
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
