// 
// 
// CREATE TABLE [ScoringImprovement]
// (
// [list_no] Integer,
// [diff_lowtime] Single,
// [diff_hightime] Single,
// [pt_score] Single,
// [swim_score] Single
// );
#[derive(Serialize,Deserialize,Debug)]
struct Scoringimprovement {
    list_no : Option<u16>,
    diff_lowtime : Option<f32>,
    diff_hightime : Option<f32>,
    pt_score : Option<f32>,
    swim_score : Option<f32>
}
impl Scoringimprovement {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Scoringimprovement>, Box<dyn Error>> {
       let mut vec: Vec<Scoringimprovement> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Scoringimprovement = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Scoringimprovement>, Error> {
        let mut vec: Vec<Scoringimprovement> = Vec::new();
        match conn.execute("SELECT * FROM scoring improvement", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let list_no = read_u16(_batch.at(0, row_index));
                        let diff_lowtime = read_f32(_batch.at(1, row_index));
                        let diff_hightime = read_f32(_batch.at(2, row_index));
                        let pt_score = read_f32(_batch.at(3, row_index));
                        let swim_score = read_f32(_batch.at(4, row_index));
                        let obj = Scoringimprovement {
                            list_no,
                            diff_lowtime,
                            diff_hightime,
                            pt_score,
                            swim_score
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
