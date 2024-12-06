// 
// 
// CREATE TABLE [CCrank]
// (
// [Event_ptr] Long Integer,
// [Ath_no] Long Integer,
// [Ath_sex] Text (2),
// [Ath_stat] Text (2),
// [Team_no] Long Integer,
// [Chute_rank] Long Integer,
// [Chute_no] Integer,
// [Div_no] Long Integer,
// [Ev_score] Long Integer,
// [JDEv_score] Long Integer,
// [score_stat] Text (2),
// [multi_age] Integer,
// [Fin_hand] Boolean NOT NULL,
// [Fin_stat] Text (2),
// [Fin_Time] Single,
// [Fin_course] Text (2),
// [Fin_place] Integer,
// [Fin_jdplace] Integer,
// [Fin_award] Integer,
// [Fin_exh] Text (2),
// [Team_ltr] Text (2)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Ccrank {
    event_ptr : Option<u64>,
    ath_no : Option<u64>,
    ath_sex : Option<String>,
    ath_stat : Option<String>,
    team_no : Option<u64>,
    chute_rank : Option<u64>,
    chute_no : Option<u16>,
    div_no : Option<u64>,
    ev_score : Option<u64>,
    j_d_ev_score : Option<u64>,
    score_stat : Option<String>,
    multi_age : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    fin_hand : bool,
    fin_stat : Option<String>,
    fin__time : Option<f32>,
    fin_course : Option<String>,
    fin_place : Option<u16>,
    fin_jdplace : Option<u16>,
    fin_award : Option<u16>,
    fin_exh : Option<String>,
    team_ltr : Option<String>
}
impl Ccrank {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Ccrank>, Box<dyn Error>> {
       let mut vec: Vec<Ccrank> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Ccrank = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Ccrank>, Error> {
        let mut vec: Vec<Ccrank> = Vec::new();
        match conn.execute("SELECT * FROM c crank", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let event_ptr = read_u64(_batch.at(0, row_index));
                        let ath_no = read_u64(_batch.at(1, row_index));
                        let ath_sex = read_string(_batch.at(2, row_index));
                        let ath_stat = read_string(_batch.at(3, row_index));
                        let team_no = read_u64(_batch.at(4, row_index));
                        let chute_rank = read_u64(_batch.at(5, row_index));
                        let chute_no = read_u16(_batch.at(6, row_index));
                        let div_no = read_u64(_batch.at(7, row_index));
                        let ev_score = read_u64(_batch.at(8, row_index));
                        let j_d_ev_score = read_u64(_batch.at(9, row_index));
                        let score_stat = read_string(_batch.at(10, row_index));
                        let multi_age = read_u16(_batch.at(11, row_index));
                        let fin_hand = read_bool(_batch.at(12, row_index));
                        let fin_stat = read_string(_batch.at(13, row_index));
                        let fin__time = read_f32(_batch.at(14, row_index));
                        let fin_course = read_string(_batch.at(15, row_index));
                        let fin_place = read_u16(_batch.at(16, row_index));
                        let fin_jdplace = read_u16(_batch.at(17, row_index));
                        let fin_award = read_u16(_batch.at(18, row_index));
                        let fin_exh = read_string(_batch.at(19, row_index));
                        let team_ltr = read_string(_batch.at(20, row_index));
                        let obj = Ccrank {
                            event_ptr,
                            ath_no,
                            ath_sex,
                            ath_stat,
                            team_no,
                            chute_rank,
                            chute_no,
                            div_no,
                            ev_score,
                            j_d_ev_score,
                            score_stat,
                            multi_age,
                            fin_hand,
                            fin_stat,
                            fin__time,
                            fin_course,
                            fin_place,
                            fin_jdplace,
                            fin_award,
                            fin_exh,
                            team_ltr
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
