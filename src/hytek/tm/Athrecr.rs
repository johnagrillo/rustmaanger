// 
// 
// CREATE TABLE [ATHRECR]
// (
// [Athlete] Long Integer,
// [Grad] Integer,
// [PicFile] Text (22),
// [HS_Name] Text (60),
// [HS_Phone] Text (40),
// [HS_Addr] Text (60),
// [HS_City] Text (40),
// [HS_State] Text (6),
// [HS_ZIP] Text (20),
// [HS_Coach] Text (60),
// [Oth_Team] Text (60),
// [Oth_Coach] Text (60),
// [HS_Study] Text (40),
// [HS_Rank] Integer,
// [HS_Num] Integer,
// [HS_GPA] Single,
// [Activities] Text (100),
// [Awards] Text (40),
// [Add_Info] Text (100),
// [Camps] Text (100),
// [T1Score] Integer,
// [Remarks] Text (100),
// [T3Score1] Integer,
// [T2Score] Integer,
// [Height] Integer,
// [Weight] Integer,
// [Oth_Phone] Text (40),
// [Picture] Text (120),
// [Video] Boolean NOT NULL,
// [Comments] Text (510),
// [T1Name] Text (16),
// [T2Name] Text (16),
// [T3Name] Text (16),
// [T3SName1] Text (16),
// [T3SName2] Text (16),
// [T3Score2] Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Athrecr {
    athlete : Option<u64>,
    grad : Option<u16>,
    pic_file : Option<String>,
    h_s__name : Option<String>,
    h_s__phone : Option<String>,
    h_s__addr : Option<String>,
    h_s__city : Option<String>,
    h_s__state : Option<String>,
    HS_ZIP : Option<String>,
    h_s__coach : Option<String>,
    oth__team : Option<String>,
    oth__coach : Option<String>,
    h_s__study : Option<String>,
    h_s__rank : Option<u16>,
    h_s__num : Option<u16>,
    HS_GPA : Option<f32>,
    activities : Option<String>,
    awards : Option<String>,
    add__info : Option<String>,
    camps : Option<String>,
    t1_score : Option<u16>,
    remarks : Option<String>,
    t3_score1 : Option<u16>,
    t2_score : Option<u16>,
    height : Option<u16>,
    weight : Option<u16>,
    oth__phone : Option<String>,
    picture : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    video : bool,
    comments : Option<String>,
    t1_name : Option<String>,
    t2_name : Option<String>,
    t3_name : Option<String>,
    t3_s_name1 : Option<String>,
    t3_s_name2 : Option<String>,
    t3_score2 : Option<u16>
}
impl Athrecr {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Athrecr>, Box<dyn Error>> {
       let mut vec: Vec<Athrecr> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Athrecr = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Athrecr>, Box<dyn Error>> {
        let mut vec: Vec<Athrecr> = Vec::new();
        match conn.execute("SELECT * FROM ATHRECR", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let athlete = read_u64(_batch.at(0, row_index))?;
                        let grad = read_u16(_batch.at(1, row_index))?;
                        let pic_file = read_string(_batch.at(2, row_index))?;
                        let h_s__name = read_string(_batch.at(3, row_index))?;
                        let h_s__phone = read_string(_batch.at(4, row_index))?;
                        let h_s__addr = read_string(_batch.at(5, row_index))?;
                        let h_s__city = read_string(_batch.at(6, row_index))?;
                        let h_s__state = read_string(_batch.at(7, row_index))?;
                        let HS_ZIP = read_string(_batch.at(8, row_index))?;
                        let h_s__coach = read_string(_batch.at(9, row_index))?;
                        let oth__team = read_string(_batch.at(10, row_index))?;
                        let oth__coach = read_string(_batch.at(11, row_index))?;
                        let h_s__study = read_string(_batch.at(12, row_index))?;
                        let h_s__rank = read_u16(_batch.at(13, row_index))?;
                        let h_s__num = read_u16(_batch.at(14, row_index))?;
                        let HS_GPA = read_f32(_batch.at(15, row_index))?;
                        let activities = read_string(_batch.at(16, row_index))?;
                        let awards = read_string(_batch.at(17, row_index))?;
                        let add__info = read_string(_batch.at(18, row_index))?;
                        let camps = read_string(_batch.at(19, row_index))?;
                        let t1_score = read_u16(_batch.at(20, row_index))?;
                        let remarks = read_string(_batch.at(21, row_index))?;
                        let t3_score1 = read_u16(_batch.at(22, row_index))?;
                        let t2_score = read_u16(_batch.at(23, row_index))?;
                        let height = read_u16(_batch.at(24, row_index))?;
                        let weight = read_u16(_batch.at(25, row_index))?;
                        let oth__phone = read_string(_batch.at(26, row_index))?;
                        let picture = read_string(_batch.at(27, row_index))?;
                        let video = read_bool(_batch.at(28, row_index))?;
                        let comments = read_string(_batch.at(29, row_index))?;
                        let t1_name = read_string(_batch.at(30, row_index))?;
                        let t2_name = read_string(_batch.at(31, row_index))?;
                        let t3_name = read_string(_batch.at(32, row_index))?;
                        let t3_s_name1 = read_string(_batch.at(33, row_index))?;
                        let t3_s_name2 = read_string(_batch.at(34, row_index))?;
                        let t3_score2 = read_u16(_batch.at(35, row_index))?;
                        let obj = Athrecr {
                            athlete,
                            grad,
                            pic_file,
                            h_s__name,
                            h_s__phone,
                            h_s__addr,
                            h_s__city,
                            h_s__state,
                            HS_ZIP,
                            h_s__coach,
                            oth__team,
                            oth__coach,
                            h_s__study,
                            h_s__rank,
                            h_s__num,
                            HS_GPA,
                            activities,
                            awards,
                            add__info,
                            camps,
                            t1_score,
                            remarks,
                            t3_score1,
                            t2_score,
                            height,
                            weight,
                            oth__phone,
                            picture,
                            video,
                            comments,
                            t1_name,
                            t2_name,
                            t3_name,
                            t3_s_name1,
                            t3_s_name2,
                            t3_score2
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
