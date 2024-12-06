// 
// 
// CREATE TABLE [COACHES]
// (
// [Coaches] Long Integer,
// [Team] Long Integer,
// [CPhone] Text (28),
// [CEMail] Text (72),
// [CName] Text (60),
// [CTitle] Text (30),
// [Cert1] Text (30),
// [Cert2] Text (30),
// [Cert3] Text (30),
// [Head] Boolean NOT NULL,
// [CoachCell] Text (40),
// [Export] Boolean NOT NULL,
// [Cert4] Text (30),
// [Cert5] Text (30),
// [Cert1Expire] DateTime,
// [Cert2Expire] DateTime,
// [Cert3Expire] DateTime,
// [Cert4Expire] DateTime,
// [Cert5Expire] DateTime,
// [PrimaryMale] Boolean NOT NULL,
// [PrimaryFemale] Boolean NOT NULL,
// [MaleOnly] Boolean NOT NULL,
// [FemaleOnly] Boolean NOT NULL,
// [cLastName] Text (40),
// [cFirstName] Text (40),
// [CFirst] Text (30),
// [CLast] Text (30)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Coaches {
    coaches : u64,
    team : Option<u64>,
    c_phone : Option<String>,
    c_e_mail : Option<String>,
    c_name : Option<String>,
    c_title : Option<String>,
    cert1 : Option<String>,
    cert2 : Option<String>,
    cert3 : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    head : bool,
    coach_cell : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    cexport : bool,
    cert4 : Option<String>,
    cert5 : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    cert1_expire : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    cert2_expire : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    cert3_expire : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    cert4_expire : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    cert5_expire : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    primary_male : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    primary_female : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    male_only : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    female_only : bool,
    c_last_name : Option<String>,
    c_first_name : Option<String>,
    c_first : Option<String>,
    c_last : Option<String>
}
impl Coaches {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Coaches>, Box<dyn Error>> {
       let mut vec: Vec<Coaches> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Coaches = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Coaches>, Box<dyn Error>> {
        let mut vec: Vec<Coaches> = Vec::new();
        match conn.execute("SELECT * FROM COACHES", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let coaches = read_u64(_batch.at(0, row_index))?;
                        let team = read_u64(_batch.at(1, row_index))?;
                        let c_phone = read_string(_batch.at(2, row_index))?;
                        let c_e_mail = read_string(_batch.at(3, row_index))?;
                        let c_name = read_string(_batch.at(4, row_index))?;
                        let c_title = read_string(_batch.at(5, row_index))?;
                        let cert1 = read_string(_batch.at(6, row_index))?;
                        let cert2 = read_string(_batch.at(7, row_index))?;
                        let cert3 = read_string(_batch.at(8, row_index))?;
                        let head = read_bool(_batch.at(9, row_index))?;
                        let coach_cell = read_string(_batch.at(10, row_index))?;
                        let cexport = read_bool(_batch.at(11, row_index))?;
                        let cert4 = read_string(_batch.at(12, row_index))?;
                        let cert5 = read_string(_batch.at(13, row_index))?;
                        let cert1_expire = read_datetime(_batch.at(14, row_index))?;
                        let cert2_expire = read_datetime(_batch.at(15, row_index))?;
                        let cert3_expire = read_datetime(_batch.at(16, row_index))?;
                        let cert4_expire = read_datetime(_batch.at(17, row_index))?;
                        let cert5_expire = read_datetime(_batch.at(18, row_index))?;
                        let primary_male = read_bool(_batch.at(19, row_index))?;
                        let primary_female = read_bool(_batch.at(20, row_index))?;
                        let male_only = read_bool(_batch.at(21, row_index))?;
                        let female_only = read_bool(_batch.at(22, row_index))?;
                        let c_last_name = read_string(_batch.at(23, row_index))?;
                        let c_first_name = read_string(_batch.at(24, row_index))?;
                        let c_first = read_string(_batch.at(25, row_index))?;
                        let c_last = read_string(_batch.at(26, row_index))?;
                        let obj = Coaches {
                            coaches,
                            team,
                            c_phone,
                            c_e_mail,
                            c_name,
                            c_title,
                            cert1,
                            cert2,
                            cert3,
                            head,
                            coach_cell,
                            cexport,
                            cert4,
                            cert5,
                            cert1_expire,
                            cert2_expire,
                            cert3_expire,
                            cert4_expire,
                            cert5_expire,
                            primary_male,
                            primary_female,
                            male_only,
                            female_only,
                            c_last_name,
                            c_first_name,
                            c_first,
                            c_last
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
