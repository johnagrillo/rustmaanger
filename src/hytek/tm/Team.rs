// 
// 
// CREATE TABLE [TEAM]
// (
// [Team] Long Integer,
// [TCode] Text (10),
// [TName] Text (60),
// [Short] Text (32),
// [LSC] Text (6),
// [MailTo] Text (80),
// [TAddr] Text (60),
// [TCity] Text (60),
// [TState] Text (6),
// [TZip] Text (20),
// [TCntry] Text (6),
// [Day] Text (40),
// [Eve] Text (40),
// [Fax] Text (40),
// [TType] Text (6),
// [Regn] Text (2),
// [Reg] Text (8),
// [MinAge] Integer,
// [NumAth] Long Integer,
// [EMail] Text (72),
// [TM50] Boolean NOT NULL,
// [TDivision] Text (6)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Team {
    team : u64,
    t_code : Option<String>,
    t_name : Option<String>,
    cshort : Option<String>,
    LSC : Option<String>,
    mail_to : Option<String>,
    t_addr : Option<String>,
    t_city : Option<String>,
    t_state : Option<String>,
    t_zip : Option<String>,
    t_cntry : Option<String>,
    day : Option<String>,
    eve : Option<String>,
    fax : Option<String>,
    t_type : Option<String>,
    regn : Option<String>,
    reg : Option<String>,
    min_age : Option<u16>,
    num_ath : Option<u64>,
    e_mail : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    TM50 : bool,
    t_division : Option<String>
}
impl Team {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Team>, Box<dyn Error>> {
       let mut vec: Vec<Team> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Team = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Team>, Box<dyn Error>> {
        let mut vec: Vec<Team> = Vec::new();
        match conn.execute("SELECT * FROM TEAM", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let team = read_u64(_batch.at(0, row_index))?;
                        let t_code = read_string(_batch.at(1, row_index))?;
                        let t_name = read_string(_batch.at(2, row_index))?;
                        let cshort = read_string(_batch.at(3, row_index))?;
                        let LSC = read_string(_batch.at(4, row_index))?;
                        let mail_to = read_string(_batch.at(5, row_index))?;
                        let t_addr = read_string(_batch.at(6, row_index))?;
                        let t_city = read_string(_batch.at(7, row_index))?;
                        let t_state = read_string(_batch.at(8, row_index))?;
                        let t_zip = read_string(_batch.at(9, row_index))?;
                        let t_cntry = read_string(_batch.at(10, row_index))?;
                        let day = read_string(_batch.at(11, row_index))?;
                        let eve = read_string(_batch.at(12, row_index))?;
                        let fax = read_string(_batch.at(13, row_index))?;
                        let t_type = read_string(_batch.at(14, row_index))?;
                        let regn = read_string(_batch.at(15, row_index))?;
                        let reg = read_string(_batch.at(16, row_index))?;
                        let min_age = read_u16(_batch.at(17, row_index))?;
                        let num_ath = read_u64(_batch.at(18, row_index))?;
                        let e_mail = read_string(_batch.at(19, row_index))?;
                        let TM50 = read_bool(_batch.at(20, row_index))?;
                        let t_division = read_string(_batch.at(21, row_index))?;
                        let obj = Team {
                            team,
                            t_code,
                            t_name,
                            cshort,
                            LSC,
                            mail_to,
                            t_addr,
                            t_city,
                            t_state,
                            t_zip,
                            t_cntry,
                            day,
                            eve,
                            fax,
                            t_type,
                            regn,
                            reg,
                            min_age,
                            num_ath,
                            e_mail,
                            TM50,
                            t_division
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
