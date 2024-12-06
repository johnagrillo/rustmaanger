// 
// 
// CREATE TABLE [Athlete]
// (
// [Athlete] Long Integer,
// [Team1] Long Integer,
// [Team2] Long Integer,
// [Team3] Long Integer,
// [Group] Text (6),
// [SubGr] Text (6),
// [Last] Text (40),
// [First] Text (40),
// [Initial] Text (2),
// [Sex] Text (2),
// [Birth] DateTime,
// [Age] Integer,
// [Class] Text (6),
// [ID_NO] Text (34),
// [Citizen] Text (6),
// [Inactive] Boolean NOT NULL,
// [Pref] Text (40),
// [Batch] Integer,
// [WMGroup] Text (6),
// [WMSubGr] Text (6),
// [BCSSASwimmer] Text (4),
// [BCSSADiver] Text (4),
// [BCSSASyncro] Text (4),
// [BCSSAPolo] Text (4),
// [TheSort] Long Integer,
// [DiveCertified] Boolean NOT NULL,
// [DateClubJoined] DateTime,
// [DateGroupJoined] DateTime,
// [AWRegType] Text (2),
// [RegYear] Integer,
// [Foreign] Boolean NOT NULL,
// [ForeignCitizenOf] Text (6),
// [LastUpdated] DateTime,
// [PC_Hide] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Athlete {
    athlete : u64,
    team1 : Option<u64>,
    team2 : Option<u64>,
    team3 : Option<u64>,
    group : Option<String>,
    sub_gr : Option<String>,
    last : Option<String>,
    first : Option<String>,
    initial : Option<String>,
    sex : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    birth : Option<NaiveDateTime>,
    age : Option<u16>,
    cclass : Option<String>,
    ID_NO : Option<String>,
    citizen : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    inactive : bool,
    pref : Option<String>,
    batch : Option<u16>,
    w_m_group : Option<String>,
    w_m_sub_gr : Option<String>,
    b_c_s_s_a_swimmer : Option<String>,
    b_c_s_s_a_diver : Option<String>,
    b_c_s_s_a_syncro : Option<String>,
    b_c_s_s_a_polo : Option<String>,
    the_sort : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    dive_certified : bool,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    date_club_joined : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    date_group_joined : Option<NaiveDateTime>,
    a_w_reg_type : Option<String>,
    reg_year : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    foreign : bool,
    foreign_citizen_of : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    last_updated : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    p_c__hide : bool
}
impl Athlete {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Athlete>, Box<dyn Error>> {
       let mut vec: Vec<Athlete> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Athlete = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Athlete>, Box<dyn Error>> {
        let mut vec: Vec<Athlete> = Vec::new();
        match conn.execute("SELECT * FROM athlete", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let athlete = read_u64(_batch.at(0, row_index))?;
                        let team1 = read_u64(_batch.at(1, row_index))?;
                        let team2 = read_u64(_batch.at(2, row_index))?;
                        let team3 = read_u64(_batch.at(3, row_index))?;
                        let group = read_string(_batch.at(4, row_index))?;
                        let sub_gr = read_string(_batch.at(5, row_index))?;
                        let last = read_string(_batch.at(6, row_index))?;
                        let first = read_string(_batch.at(7, row_index))?;
                        let initial = read_string(_batch.at(8, row_index))?;
                        let sex = read_string(_batch.at(9, row_index))?;
                        let birth = read_datetime(_batch.at(10, row_index))?;
                        let age = read_u16(_batch.at(11, row_index))?;
                        let cclass = read_string(_batch.at(12, row_index))?;
                        let ID_NO = read_string(_batch.at(13, row_index))?;
                        let citizen = read_string(_batch.at(14, row_index))?;
                        let inactive = read_bool(_batch.at(15, row_index))?;
                        let pref = read_string(_batch.at(16, row_index))?;
                        let batch = read_u16(_batch.at(17, row_index))?;
                        let w_m_group = read_string(_batch.at(18, row_index))?;
                        let w_m_sub_gr = read_string(_batch.at(19, row_index))?;
                        let b_c_s_s_a_swimmer = read_string(_batch.at(20, row_index))?;
                        let b_c_s_s_a_diver = read_string(_batch.at(21, row_index))?;
                        let b_c_s_s_a_syncro = read_string(_batch.at(22, row_index))?;
                        let b_c_s_s_a_polo = read_string(_batch.at(23, row_index))?;
                        let the_sort = read_u64(_batch.at(24, row_index))?;
                        let dive_certified = read_bool(_batch.at(25, row_index))?;
                        let date_club_joined = read_datetime(_batch.at(26, row_index))?;
                        let date_group_joined = read_datetime(_batch.at(27, row_index))?;
                        let a_w_reg_type = read_string(_batch.at(28, row_index))?;
                        let reg_year = read_u16(_batch.at(29, row_index))?;
                        let foreign = read_bool(_batch.at(30, row_index))?;
                        let foreign_citizen_of = read_string(_batch.at(31, row_index))?;
                        let last_updated = read_datetime(_batch.at(32, row_index))?;
                        let p_c__hide = read_bool(_batch.at(33, row_index))?;
                        let obj = Athlete {
                            athlete,
                          team1 : Some(team1),
                          team2 : Some(team2),
                          team3 : Some(team3),
                          group : Some(group),
                          sub_gr : Some(sub_gr),
                          last : Some(last),
                          first : Some(first),
                          initial : Some(initial),
                          sex : Some(sex),
                          birth : Some(birth),
                          age : Some(age),
                          cclass : Some(cclass),
                          ID_NO : Some(ID_NO),
                          citizen : Some(citizen),
                            inactive,
                          pref : Some(pref),
                          batch : Some(batch),
                          w_m_group : Some(w_m_group),
                          w_m_sub_gr : Some(w_m_sub_gr),
                          b_c_s_s_a_swimmer : Some(b_c_s_s_a_swimmer),
                          b_c_s_s_a_diver : Some(b_c_s_s_a_diver),
                          b_c_s_s_a_syncro : Some(b_c_s_s_a_syncro),
                          b_c_s_s_a_polo : Some(b_c_s_s_a_polo),
                          the_sort : Some(the_sort),
                            dive_certified,
                          date_club_joined : Some(date_club_joined),
                          date_group_joined : Some(date_group_joined),
                          a_w_reg_type : Some(a_w_reg_type),
                          reg_year : Some(reg_year),
                            foreign,
                          foreign_citizen_of : Some(foreign_citizen_of),
                          last_updated : Some(last_updated),
                            p_c__hide
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
