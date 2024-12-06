// 
// 
// CREATE TABLE [TeamCoaches]
// (
// [Coach_no] Long Integer,
// [Team_no] Long Integer,
// [Coach_LastName] Text (40),
// [Coach_FirstName] Text (40),
// [Coach_Title] Text (30),
// [Primary_MaleHeadCoach] Boolean NOT NULL,
// [Primary_FemaleHeadCoach] Boolean NOT NULL,
// [MaleCoach_Only] Boolean NOT NULL,
// [FemaleCoach_Only] Boolean NOT NULL,
// [Coach_Phone] Text (40),
// [Coach_Cell] Text (40),
// [Coach_EMail] Text (72),
// [Coach_Cert1] Text (30),
// [Coach_Cert2] Text (30),
// [Coach_Cert3] Text (30),
// [Coach_Cert4] Text (30),
// [Coach_ExpDate1] DateTime,
// [Coach_ExpDate2] DateTime,
// [Coach_ExpDate3] DateTime,
// [Coach_ExpDate4] DateTime
// );
#[derive(Serialize,Deserialize,Debug)]
struct Teamcoaches {
    coach_no : Option<u64>,
    team_no : Option<u64>,
    coach__last_name : Option<String>,
    coach__first_name : Option<String>,
    coach__title : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    primary__male_head_coach : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    primary__female_head_coach : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    male_coach__only : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    female_coach__only : bool,
    coach__phone : Option<String>,
    coach__cell : Option<String>,
    coach__e_mail : Option<String>,
    coach__cert1 : Option<String>,
    coach__cert2 : Option<String>,
    coach__cert3 : Option<String>,
    coach__cert4 : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    coach__exp_date1 : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    coach__exp_date2 : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    coach__exp_date3 : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    coach__exp_date4 : Option<NaiveDateTime>
}
impl Teamcoaches {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Teamcoaches>, Box<dyn Error>> {
       let mut vec: Vec<Teamcoaches> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Teamcoaches = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Teamcoaches>, Error> {
        let mut vec: Vec<Teamcoaches> = Vec::new();
        match conn.execute("SELECT * FROM team coaches", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let coach_no = read_u64(_batch.at(0, row_index));
                        let team_no = read_u64(_batch.at(1, row_index));
                        let coach__last_name = read_string(_batch.at(2, row_index));
                        let coach__first_name = read_string(_batch.at(3, row_index));
                        let coach__title = read_string(_batch.at(4, row_index));
                        let primary__male_head_coach = read_bool(_batch.at(5, row_index));
                        let primary__female_head_coach = read_bool(_batch.at(6, row_index));
                        let male_coach__only = read_bool(_batch.at(7, row_index));
                        let female_coach__only = read_bool(_batch.at(8, row_index));
                        let coach__phone = read_string(_batch.at(9, row_index));
                        let coach__cell = read_string(_batch.at(10, row_index));
                        let coach__e_mail = read_string(_batch.at(11, row_index));
                        let coach__cert1 = read_string(_batch.at(12, row_index));
                        let coach__cert2 = read_string(_batch.at(13, row_index));
                        let coach__cert3 = read_string(_batch.at(14, row_index));
                        let coach__cert4 = read_string(_batch.at(15, row_index));
                        let coach__exp_date1 = read_datetime(_batch.at(16, row_index));
                        let coach__exp_date2 = read_datetime(_batch.at(17, row_index));
                        let coach__exp_date3 = read_datetime(_batch.at(18, row_index));
                        let coach__exp_date4 = read_datetime(_batch.at(19, row_index));
                        let obj = Teamcoaches {
                            coach_no,
                            team_no,
                            coach__last_name,
                            coach__first_name,
                            coach__title,
                            primary__male_head_coach,
                            primary__female_head_coach,
                            male_coach__only,
                            female_coach__only,
                            coach__phone,
                            coach__cell,
                            coach__e_mail,
                            coach__cert1,
                            coach__cert2,
                            coach__cert3,
                            coach__cert4,
                            coach__exp_date1,
                            coach__exp_date2,
                            coach__exp_date3,
                            coach__exp_date4
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
