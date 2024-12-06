// 
// 
// CREATE TABLE [WorkCategory]
// (
// [stroke_abbr] Text (2),
// [stroke_name] Text (40)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Workcategory {
    stroke_abbr : Option<String>,
    stroke_name : Option<String>
}
impl Workcategory {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Workcategory>, Box<dyn Error>> {
       let mut vec: Vec<Workcategory> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Workcategory = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Workcategory>, Box<dyn Error>> {
        let mut vec: Vec<Workcategory> = Vec::new();
        match conn.execute("SELECT * FROM work category", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let stroke_abbr = read_string(_batch.at(0, row_index))?;
                        let stroke_name = read_string(_batch.at(1, row_index))?;
                        let obj = Workcategory {
                            stroke_abbr,
                            stroke_name
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
