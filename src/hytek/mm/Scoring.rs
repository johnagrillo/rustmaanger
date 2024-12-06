// 
// 
// CREATE TABLE [Scoring]
// (
// [score_divno] Integer,
// [score_sex] Text (2),
// [score_place] Integer,
// [ind_score] Single,
// [rel_score] Single
// );
#[derive(Serialize,Deserialize,Debug)]
struct Scoring {
    score_divno : Option<u16>,
    score_sex : Option<String>,
    score_place : Option<u16>,
    ind_score : Option<f32>,
    rel_score : Option<f32>
}
impl Scoring {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Scoring>, Box<dyn Error>> {
       let mut vec: Vec<Scoring> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Scoring = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Scoring>, Error> {
        let mut vec: Vec<Scoring> = Vec::new();
        match conn.execute("SELECT * FROM scoring", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let score_divno = read_u16(_batch.at(0, row_index));
                        let score_sex = read_string(_batch.at(1, row_index));
                        let score_place = read_u16(_batch.at(2, row_index));
                        let ind_score = read_f32(_batch.at(3, row_index));
                        let rel_score = read_f32(_batch.at(4, row_index));
                        let obj = Scoring {
                            score_divno,
                            score_sex,
                            score_place,
                            ind_score,
                            rel_score
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
