// 
// 
// CREATE TABLE [ModelParam]
// (
// [wk_ptr] Long Integer,
// [wk_num] Long Integer,
// [wk_date] DateTime,
// [start_time] Long Integer,
// [rest_period] Integer,
// [group_abbr] Text (6),
// [subgroup_abbr] Text (6),
// [course_yls] Text (2),
// [tot_yards] Long Integer,
// [tot_stress] Long Integer,
// [ck_selected] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Modelparam {
    wk_ptr : Option<u64>,
    wk_num : Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    wk_date : Option<NaiveDateTime>,
    start_time : Option<u64>,
    rest_period : Option<u16>,
    group_abbr : Option<String>,
    subgroup_abbr : Option<String>,
    course_yls : Option<String>,
    tot_yards : Option<u64>,
    tot_stress : Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    ck_selected : bool
}
impl Modelparam {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Modelparam>, Box<dyn Error>> {
       let mut vec: Vec<Modelparam> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Modelparam = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Modelparam>, Box<dyn Error>> {
        let mut vec: Vec<Modelparam> = Vec::new();
        match conn.execute("SELECT * FROM model param", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let wk_ptr = read_u64(_batch.at(0, row_index))?;
                        let wk_num = read_u64(_batch.at(1, row_index))?;
                        let wk_date = read_datetime(_batch.at(2, row_index))?;
                        let start_time = read_u64(_batch.at(3, row_index))?;
                        let rest_period = read_u16(_batch.at(4, row_index))?;
                        let group_abbr = read_string(_batch.at(5, row_index))?;
                        let subgroup_abbr = read_string(_batch.at(6, row_index))?;
                        let course_yls = read_string(_batch.at(7, row_index))?;
                        let tot_yards = read_u64(_batch.at(8, row_index))?;
                        let tot_stress = read_u64(_batch.at(9, row_index))?;
                        let ck_selected = read_bool(_batch.at(10, row_index))?;
                        let obj = Modelparam {
                            wk_ptr,
                            wk_num,
                            wk_date,
                            start_time,
                            rest_period,
                            group_abbr,
                            subgroup_abbr,
                            course_yls,
                            tot_yards,
                            tot_stress,
                            ck_selected
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
