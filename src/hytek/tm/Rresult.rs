// 
// 
// CREATE TABLE [RESULT]
// (
// [MEET] Long Integer,
// [ATHLETE] Long Integer,
// [I_R] Text (2),
// [TEAM] Long Integer,
// [SCORE] Long Integer,
// [F_P] Text (2),
// [SPLIT] Long Integer,
// [EX] Text (2),
// [ORIGIN] Text (8),
// [NT] Byte,
// [RESULT] Long Integer,
// [MISC] Text (2),
// [AGE] Integer,
// [DISTANCE] Integer,
// [STROKE] Integer,
// [MTEVENT] Long Integer,
// [POINTS] Integer,
// [PLACE] Integer,
// [RANK] Integer,
// [TRANK] Integer,
// [COURSE] Text (2),
// [REACTION] Text (10),
// [DQCODE] Text (4),
// [DQDESCRIPT] Text (180),
// [DQCODESecondary] Text (4),
// [DQDESCRIPTSecondary] Text (180)
// );
#[derive(Serialize,Deserialize,Debug)]
struct RResult {
    MEET : Option<u64>,
    ATHLETE : Option<u64>,
    I_R : Option<String>,
    TEAM : Option<u64>,
    SCORE : Option<u64>,
    F_P : Option<String>,
    SPLIT : Option<u64>,
    EX : Option<String>,
    ORIGIN : Option<String>,
    NT : Option<u8>,
    RESULT : u64,
    MISC : Option<String>,
    AGE : Option<u16>,
    DISTANCE : Option<u16>,
    STROKE : Option<u16>,
    MTEVENT : Option<u64>,
    POINTS : Option<u16>,
    PLACE : Option<u16>,
    RANK : Option<u16>,
    TRANK : Option<u16>,
    COURSE : Option<String>,
    REACTION : Option<String>,
    DQCODE : Option<String>,
    DQDESCRIPT : Option<String>,
    d_q_c_o_d_e_secondary : Option<String>,
    d_q_d_e_s_c_r_i_p_t_secondary : Option<String>
}
impl RResult {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<RResult>, Box<dyn Error>> {
       let mut vec: Vec<RResult> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: RResult = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<RResult>, Box<dyn Error>> {
        let mut vec: Vec<RResult> = Vec::new();
        match conn.execute("SELECT * FROM RESULT", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let MEET = read_u64(_batch.at(0, row_index))?;
                        let ATHLETE = read_u64(_batch.at(1, row_index))?;
                        let I_R = read_string(_batch.at(2, row_index))?;
                        let TEAM = read_u64(_batch.at(3, row_index))?;
                        let SCORE = read_u64(_batch.at(4, row_index))?;
                        let F_P = read_string(_batch.at(5, row_index))?;
                        let SPLIT = read_u64(_batch.at(6, row_index))?;
                        let EX = read_string(_batch.at(7, row_index))?;
                        let ORIGIN = read_string(_batch.at(8, row_index))?;
                        let NT = read_u8(_batch.at(9, row_index))?;
                        let RESULT = read_u64(_batch.at(10, row_index))?;
                        let MISC = read_string(_batch.at(11, row_index))?;
                        let AGE = read_u16(_batch.at(12, row_index))?;
                        let DISTANCE = read_u16(_batch.at(13, row_index))?;
                        let STROKE = read_u16(_batch.at(14, row_index))?;
                        let MTEVENT = read_u64(_batch.at(15, row_index))?;
                        let POINTS = read_u16(_batch.at(16, row_index))?;
                        let PLACE = read_u16(_batch.at(17, row_index))?;
                        let RANK = read_u16(_batch.at(18, row_index))?;
                        let TRANK = read_u16(_batch.at(19, row_index))?;
                        let COURSE = read_string(_batch.at(20, row_index))?;
                        let REACTION = read_string(_batch.at(21, row_index))?;
                        let DQCODE = read_string(_batch.at(22, row_index))?;
                        let DQDESCRIPT = read_string(_batch.at(23, row_index))?;
                        let d_q_c_o_d_e_secondary = read_string(_batch.at(24, row_index))?;
                        let d_q_d_e_s_c_r_i_p_t_secondary = read_string(_batch.at(25, row_index))?;
                        let obj = RResult {
                          MEET : Some(MEET),
                          ATHLETE : Some(ATHLETE),
                          I_R : Some(I_R),
                          TEAM : Some(TEAM),
                          SCORE : Some(SCORE),
                          F_P : Some(F_P),
                          SPLIT : Some(SPLIT),
                          EX : Some(EX),
                          ORIGIN : Some(ORIGIN),
                          NT : Some(NT),
                            RESULT,
                          MISC : Some(MISC),
                          AGE : Some(AGE),
                          DISTANCE : Some(DISTANCE),
                          STROKE : Some(STROKE),
                          MTEVENT : Some(MTEVENT),
                          POINTS : Some(POINTS),
                          PLACE : Some(PLACE),
                          RANK : Some(RANK),
                          TRANK : Some(TRANK),
                          COURSE : Some(COURSE),
                          REACTION : Some(REACTION),
                          DQCODE : Some(DQCODE),
                          DQDESCRIPT : Some(DQDESCRIPT),
                          d_q_c_o_d_e_secondary : Some(d_q_c_o_d_e_secondary),
                          d_q_d_e_s_c_r_i_p_t_secondary : Some(d_q_d_e_s_c_r_i_p_t_secondary)
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
