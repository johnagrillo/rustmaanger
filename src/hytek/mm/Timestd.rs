// 
// 
// CREATE TABLE [TimeStd]
// (
// [tag_ptr] Long Integer,
// [tag_gender] Text (2),
// [tag_indrel] Text (2),
// [tag_dist] Long Integer,
// [tag_stroke] Text (2),
// [low_age] Integer,
// [high_Age] Integer,
// [tag_time] Single,
// [tag_course] Text (2),
// [div_abbr] Text (20)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Timestd {
    tag_ptr : Option<u64>,
    tag_gender : Option<String>,
    tag_indrel : Option<String>,
    tag_dist : Option<u64>,
    tag_stroke : Option<String>,
    low_age : Option<u16>,
    high__age : Option<u16>,
    tag_time : Option<f32>,
    tag_course : Option<String>,
    div_abbr : Option<String>
}
impl Timestd {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Timestd>, Box<dyn Error>> {
       let mut vec: Vec<Timestd> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Timestd = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Timestd>, Error> {
        let mut vec: Vec<Timestd> = Vec::new();
        match conn.execute("SELECT * FROM time std", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let tag_ptr = read_u64(_batch.at(0, row_index));
                        let tag_gender = read_string(_batch.at(1, row_index));
                        let tag_indrel = read_string(_batch.at(2, row_index));
                        let tag_dist = read_u64(_batch.at(3, row_index));
                        let tag_stroke = read_string(_batch.at(4, row_index));
                        let low_age = read_u16(_batch.at(5, row_index));
                        let high__age = read_u16(_batch.at(6, row_index));
                        let tag_time = read_f32(_batch.at(7, row_index));
                        let tag_course = read_string(_batch.at(8, row_index));
                        let div_abbr = read_string(_batch.at(9, row_index));
                        let obj = Timestd {
                            tag_ptr,
                            tag_gender,
                            tag_indrel,
                            tag_dist,
                            tag_stroke,
                            low_age,
                            high__age,
                            tag_time,
                            tag_course,
                            div_abbr
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
