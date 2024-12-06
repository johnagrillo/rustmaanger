// 
// 
// CREATE TABLE [ScoreLanes]
// (
// [lane_00] Boolean NOT NULL,
// [lane_01] Boolean NOT NULL,
// [lane_02] Boolean NOT NULL,
// [lane_03] Boolean NOT NULL,
// [lane_04] Boolean NOT NULL,
// [lane_05] Boolean NOT NULL,
// [lane_06] Boolean NOT NULL,
// [lane_07] Boolean NOT NULL,
// [lane_08] Boolean NOT NULL,
// [lane_09] Boolean NOT NULL,
// [lane_10] Boolean NOT NULL,
// [lane_11] Boolean NOT NULL,
// [lane_12] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Scorelanes {
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane00 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane01 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane02 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane03 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane04 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane05 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane06 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane07 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane08 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane09 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane10 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane11 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    lane12 : bool
}
impl Scorelanes {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Scorelanes>, Box<dyn Error>> {
       let mut vec: Vec<Scorelanes> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Scorelanes = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Scorelanes>, Error> {
        let mut vec: Vec<Scorelanes> = Vec::new();
        match conn.execute("SELECT * FROM score lanes", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let lane00 = read_bool(_batch.at(0, row_index));
                        let lane01 = read_bool(_batch.at(1, row_index));
                        let lane02 = read_bool(_batch.at(2, row_index));
                        let lane03 = read_bool(_batch.at(3, row_index));
                        let lane04 = read_bool(_batch.at(4, row_index));
                        let lane05 = read_bool(_batch.at(5, row_index));
                        let lane06 = read_bool(_batch.at(6, row_index));
                        let lane07 = read_bool(_batch.at(7, row_index));
                        let lane08 = read_bool(_batch.at(8, row_index));
                        let lane09 = read_bool(_batch.at(9, row_index));
                        let lane10 = read_bool(_batch.at(10, row_index));
                        let lane11 = read_bool(_batch.at(11, row_index));
                        let lane12 = read_bool(_batch.at(12, row_index));
                        let obj = Scorelanes {
                            lane00,
                            lane01,
                            lane02,
                            lane03,
                            lane04,
                            lane05,
                            lane06,
                            lane07,
                            lane08,
                            lane09,
                            lane10,
                            lane11,
                            lane12
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
