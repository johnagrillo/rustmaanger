// 
// 
// CREATE TABLE [CustomLayoutValues]
// (
// [LayoutID] Long Integer,
// [Line] Byte,
// [Sequence] Byte,
// [FieldName] Text (40),
// [FieldSize] Byte,
// [Abbr] Text (30),
// [RJustified] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Customlayoutvalues {
    layout_i_d : Option<u64>,
    line : Option<u8>,
    sequence : Option<u8>,
    field_name : Option<String>,
    field_size : Option<u8>,
    abbr : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    r_justified : bool
}
impl Customlayoutvalues {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Customlayoutvalues>, Box<dyn Error>> {
       let mut vec: Vec<Customlayoutvalues> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Customlayoutvalues = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Customlayoutvalues>, Box<dyn Error>> {
        let mut vec: Vec<Customlayoutvalues> = Vec::new();
        match conn.execute("SELECT * FROM custom layout values", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let layout_i_d = read_u64(_batch.at(0, row_index))?;
                        let line = read_u8(_batch.at(1, row_index))?;
                        let sequence = read_u8(_batch.at(2, row_index))?;
                        let field_name = read_string(_batch.at(3, row_index))?;
                        let field_size = read_u8(_batch.at(4, row_index))?;
                        let abbr = read_string(_batch.at(5, row_index))?;
                        let r_justified = read_bool(_batch.at(6, row_index))?;
                        let obj = Customlayoutvalues {
                            layout_i_d,
                            line,
                            sequence,
                            field_name,
                            field_size,
                            abbr,
                            r_justified
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
