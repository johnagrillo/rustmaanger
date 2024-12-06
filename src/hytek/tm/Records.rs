// 
// 
// CREATE TABLE [RECORDS]
// (
// [Record] Long Integer,
// [Lo_Age] Integer,
// [Hi_Age] Integer,
// [RecText] Text (160),
// [RecDate] DateTime,
// [RecTime] Long Integer,
// [RecLSC] Text (6),
// [RecTeam] Text (10),
// [Ex] Text (2),
// [Distance] Integer,
// [Stroke] Integer,
// [Sex] Text (2),
// [I_R] Text (2),
// [Rec] Long Integer,
// [Course] Text (2),
// [Division] Text (4)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Records {
    record : Option<u64>,
    lo__age : Option<u16>,
    hi__age : Option<u16>,
    rec_text : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    rec_date : Option<NaiveDateTime>,
    rec_time : Option<u64>,
    rec_l_s_c : Option<String>,
    rec_team : Option<String>,
    ex : Option<String>,
    distance : Option<u16>,
    stroke : Option<u16>,
    sex : Option<String>,
    I_R : Option<String>,
    rec : Option<u64>,
    course : Option<String>,
    division : Option<String>
}
impl Records {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Records>, Box<dyn Error>> {
       let mut vec: Vec<Records> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Records = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Records>, Box<dyn Error>> {
        let mut vec: Vec<Records> = Vec::new();
        match conn.execute("SELECT * FROM RECORDS", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let record = read_u64(_batch.at(0, row_index))?;
                        let lo__age = read_u16(_batch.at(1, row_index))?;
                        let hi__age = read_u16(_batch.at(2, row_index))?;
                        let rec_text = read_string(_batch.at(3, row_index))?;
                        let rec_date = read_datetime(_batch.at(4, row_index))?;
                        let rec_time = read_u64(_batch.at(5, row_index))?;
                        let rec_l_s_c = read_string(_batch.at(6, row_index))?;
                        let rec_team = read_string(_batch.at(7, row_index))?;
                        let ex = read_string(_batch.at(8, row_index))?;
                        let distance = read_u16(_batch.at(9, row_index))?;
                        let stroke = read_u16(_batch.at(10, row_index))?;
                        let sex = read_string(_batch.at(11, row_index))?;
                        let I_R = read_string(_batch.at(12, row_index))?;
                        let rec = read_u64(_batch.at(13, row_index))?;
                        let course = read_string(_batch.at(14, row_index))?;
                        let division = read_string(_batch.at(15, row_index))?;
                        let obj = Records {
                            record,
                            lo__age,
                            hi__age,
                            rec_text,
                            rec_date,
                            rec_time,
                            rec_l_s_c,
                            rec_team,
                            ex,
                            distance,
                            stroke,
                            sex,
                            I_R,
                            rec,
                            course,
                            division
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
