// 
// 
// CREATE TABLE [HYTEKAGEGROUP]
// (
// [AgType] Integer,
// [Lo_Hi] Integer,
// [Course] Text (2),
// [F125] Boolean NOT NULL,
// [F150] Boolean NOT NULL,
// [F1100] Boolean NOT NULL,
// [F1200] Boolean NOT NULL,
// [F1400] Boolean NOT NULL,
// [F1500] Boolean NOT NULL,
// [F1800] Boolean NOT NULL,
// [F11000] Boolean NOT NULL,
// [F11500] Boolean NOT NULL,
// [F11650] Boolean NOT NULL,
// [F225] Boolean NOT NULL,
// [F250] Boolean NOT NULL,
// [F2100] Boolean NOT NULL,
// [F2200] Boolean NOT NULL,
// [F325] Boolean NOT NULL,
// [F350] Boolean NOT NULL,
// [F3100] Boolean NOT NULL,
// [F3200] Boolean NOT NULL,
// [F425] Boolean NOT NULL,
// [F450] Boolean NOT NULL,
// [F4100] Boolean NOT NULL,
// [F4200] Boolean NOT NULL,
// [F5100] Boolean NOT NULL,
// [F5200] Boolean NOT NULL,
// [F5400] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Hytekagegroup {
    ag_type : Option<u16>,
    lo__hi : Option<u16>,
    course : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F125 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F150 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F1100 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F1200 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F1400 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F1500 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F1800 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F11000 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F11500 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F11650 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F225 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F250 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F2100 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F2200 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F325 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F350 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F3100 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F3200 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F425 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F450 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F4100 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F4200 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F5100 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F5200 : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    F5400 : bool
}
impl Hytekagegroup {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Hytekagegroup>, Box<dyn Error>> {
       let mut vec: Vec<Hytekagegroup> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Hytekagegroup = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Hytekagegroup>, Box<dyn Error>> {
        let mut vec: Vec<Hytekagegroup> = Vec::new();
        match conn.execute("SELECT * FROM HYTEKAGEGROUP", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let ag_type = read_u16(_batch.at(0, row_index))?;
                        let lo__hi = read_u16(_batch.at(1, row_index))?;
                        let course = read_string(_batch.at(2, row_index))?;
                        let F125 = read_bool(_batch.at(3, row_index))?;
                        let F150 = read_bool(_batch.at(4, row_index))?;
                        let F1100 = read_bool(_batch.at(5, row_index))?;
                        let F1200 = read_bool(_batch.at(6, row_index))?;
                        let F1400 = read_bool(_batch.at(7, row_index))?;
                        let F1500 = read_bool(_batch.at(8, row_index))?;
                        let F1800 = read_bool(_batch.at(9, row_index))?;
                        let F11000 = read_bool(_batch.at(10, row_index))?;
                        let F11500 = read_bool(_batch.at(11, row_index))?;
                        let F11650 = read_bool(_batch.at(12, row_index))?;
                        let F225 = read_bool(_batch.at(13, row_index))?;
                        let F250 = read_bool(_batch.at(14, row_index))?;
                        let F2100 = read_bool(_batch.at(15, row_index))?;
                        let F2200 = read_bool(_batch.at(16, row_index))?;
                        let F325 = read_bool(_batch.at(17, row_index))?;
                        let F350 = read_bool(_batch.at(18, row_index))?;
                        let F3100 = read_bool(_batch.at(19, row_index))?;
                        let F3200 = read_bool(_batch.at(20, row_index))?;
                        let F425 = read_bool(_batch.at(21, row_index))?;
                        let F450 = read_bool(_batch.at(22, row_index))?;
                        let F4100 = read_bool(_batch.at(23, row_index))?;
                        let F4200 = read_bool(_batch.at(24, row_index))?;
                        let F5100 = read_bool(_batch.at(25, row_index))?;
                        let F5200 = read_bool(_batch.at(26, row_index))?;
                        let F5400 = read_bool(_batch.at(27, row_index))?;
                        let obj = Hytekagegroup {
                            ag_type,
                            lo__hi,
                            course,
                            F125,
                            F150,
                            F1100,
                            F1200,
                            F1400,
                            F1500,
                            F1800,
                            F11000,
                            F11500,
                            F11650,
                            F225,
                            F250,
                            F2100,
                            F2200,
                            F325,
                            F350,
                            F3100,
                            F3200,
                            F425,
                            F450,
                            F4100,
                            F4200,
                            F5100,
                            F5200,
                            F5400
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
