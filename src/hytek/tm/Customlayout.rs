// 
// 
// CREATE TABLE [CustomLayout]
// (
// [ID] Long Integer,
// [Name] Text (40),
// [Portrait] Boolean NOT NULL,
// [Title] Text (100)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Customlayout {
    ID : Option<u64>,
    name : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    portrait : bool,
    title : Option<String>
}
impl Customlayout {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Customlayout>, Box<dyn Error>> {
       let mut vec: Vec<Customlayout> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Customlayout = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Customlayout>, Box<dyn Error>> {
        let mut vec: Vec<Customlayout> = Vec::new();
        match conn.execute("SELECT * FROM custom layout", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let ID = read_u64(_batch.at(0, row_index))?;
                        let name = read_string(_batch.at(1, row_index))?;
                        let portrait = read_bool(_batch.at(2, row_index))?;
                        let title = read_string(_batch.at(3, row_index))?;
                        let obj = Customlayout {
                            ID,
                            name,
                            portrait,
                            title
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
