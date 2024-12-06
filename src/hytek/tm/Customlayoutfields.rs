// 
// 
// CREATE TABLE [CustomLayoutFields]
// (
// [FieldName] Text (40),
// [Description] Text (60),
// [Size] Integer,
// [Abbr] Text (30)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Customlayoutfields {
    field_name : Option<String>,
    description : Option<String>,
    size : Option<u16>,
    abbr : Option<String>
}
impl Customlayoutfields {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Customlayoutfields>, Box<dyn Error>> {
       let mut vec: Vec<Customlayoutfields> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Customlayoutfields = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Customlayoutfields>, Box<dyn Error>> {
        let mut vec: Vec<Customlayoutfields> = Vec::new();
        match conn.execute("SELECT * FROM custom layout fields", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let field_name = read_string(_batch.at(0, row_index))?;
                        let description = read_string(_batch.at(1, row_index))?;
                        let size = read_u16(_batch.at(2, row_index))?;
                        let abbr = read_string(_batch.at(3, row_index))?;
                        let obj = Customlayoutfields {
                            field_name,
                            description,
                            size,
                            abbr
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
