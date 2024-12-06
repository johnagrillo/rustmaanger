// 
// 
// CREATE TABLE [RELAY]
// (
// [RELAY] Long Integer,
// [MEET] Long Integer,
// [LO_HI] Integer,
// [TEAM] Long Integer,
// [LETTER] Text (2),
// [AGE_RANGE] Integer,
// [SEX] Text (2),
// [ATH(1)] Long Integer,
// [ATH(2)] Long Integer,
// [ATH(3)] Long Integer,
// [ATH(4)] Long Integer,
// [ATH(5)] Long Integer,
// [ATH(6)] Long Integer,
// [ATH(7)] Long Integer,
// [ATH(8)] Long Integer,
// [DISTANCE] Integer,
// [STROKE] Integer,
// [RELAYAGE] Text (6),
// [REACTION1] Text (10),
// [REACTION2] Text (10),
// [REACTION3] Text (10),
// [REACTION4] Text (10)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Relay {
    RELAY : u64,
    MEET : Option<u64>,
    LO_HI : Option<u16>,
    TEAM : Option<u64>,
    LETTER : Option<String>,
    AGE_RANGE : Option<u16>,
    SEX : Option<String>,
    ATH_1_ : Option<u64>,
    ATH_2_ : Option<u64>,
    ATH_3_ : Option<u64>,
    ATH_4_ : Option<u64>,
    ATH_5_ : Option<u64>,
    ATH_6_ : Option<u64>,
    ATH_7_ : Option<u64>,
    ATH_8_ : Option<u64>,
    DISTANCE : Option<u16>,
    STROKE : Option<u16>,
    RELAYAGE : Option<String>,
    REACTION1 : Option<String>,
    REACTION2 : Option<String>,
    REACTION3 : Option<String>,
    REACTION4 : Option<String>
}
impl Relay {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Relay>, Box<dyn Error>> {
       let mut vec: Vec<Relay> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Relay = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Relay>, Box<dyn Error>> {
        let mut vec: Vec<Relay> = Vec::new();
        match conn.execute("SELECT * FROM RELAY", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let RELAY = read_u64(_batch.at(0, row_index))?;
                        let MEET = read_u64(_batch.at(1, row_index))?;
                        let LO_HI = read_u16(_batch.at(2, row_index))?;
                        let TEAM = read_u64(_batch.at(3, row_index))?;
                        let LETTER = read_string(_batch.at(4, row_index))?;
                        let AGE_RANGE = read_u16(_batch.at(5, row_index))?;
                        let SEX = read_string(_batch.at(6, row_index))?;
                        let ATH_1_ = read_u64(_batch.at(7, row_index))?;
                        let ATH_2_ = read_u64(_batch.at(8, row_index))?;
                        let ATH_3_ = read_u64(_batch.at(9, row_index))?;
                        let ATH_4_ = read_u64(_batch.at(10, row_index))?;
                        let ATH_5_ = read_u64(_batch.at(11, row_index))?;
                        let ATH_6_ = read_u64(_batch.at(12, row_index))?;
                        let ATH_7_ = read_u64(_batch.at(13, row_index))?;
                        let ATH_8_ = read_u64(_batch.at(14, row_index))?;
                        let DISTANCE = read_u16(_batch.at(15, row_index))?;
                        let STROKE = read_u16(_batch.at(16, row_index))?;
                        let RELAYAGE = read_string(_batch.at(17, row_index))?;
                        let REACTION1 = read_string(_batch.at(18, row_index))?;
                        let REACTION2 = read_string(_batch.at(19, row_index))?;
                        let REACTION3 = read_string(_batch.at(20, row_index))?;
                        let REACTION4 = read_string(_batch.at(21, row_index))?;
                        let obj = Relay {
                            RELAY,
                            MEET,
                            LO_HI,
                            TEAM,
                            LETTER,
                            AGE_RANGE,
                            SEX,
                            ATH_1_,
                            ATH_2_,
                            ATH_3_,
                            ATH_4_,
                            ATH_5_,
                            ATH_6_,
                            ATH_7_,
                            ATH_8_,
                            DISTANCE,
                            STROKE,
                            RELAYAGE,
                            REACTION1,
                            REACTION2,
                            REACTION3,
                            REACTION4
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
