// 
// 
// CREATE TABLE [Dualteams]
// (
// [team_gender] Text (2),
// [ateam_no] Long Integer,
// [bteam_no] Long Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Dualteams {
    team_gender : Option<String>,
    ateam_no : Option<u64>,
    bteam_no : Option<u64>
}
impl Dualteams {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Dualteams>, Box<dyn Error>> {
       let mut vec: Vec<Dualteams> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Dualteams = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Dualteams>, Error> {
        let mut vec: Vec<Dualteams> = Vec::new();
        match conn.execute("SELECT * FROM dualteams", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let team_gender = read_string(_batch.at(0, row_index));
                        let ateam_no = read_u64(_batch.at(1, row_index));
                        let bteam_no = read_u64(_batch.at(2, row_index));
                        let obj = Dualteams {
                            team_gender,
                            ateam_no,
                            bteam_no
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
