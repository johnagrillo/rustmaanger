// 
// 
// CREATE TABLE [CUSTOMAGEGROUP]
// (
// [Lo_Hi] Integer,
// [EventNum] Integer,
// [Stroke] Integer,
// [Distance] Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Customagegroup {
    lo__hi : Option<u16>,
    event_num : Option<u16>,
    stroke : Option<u16>,
    distance : Option<u16>
}
impl Customagegroup {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Customagegroup>, Box<dyn Error>> {
       let mut vec: Vec<Customagegroup> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Customagegroup = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Customagegroup>, Box<dyn Error>> {
        let mut vec: Vec<Customagegroup> = Vec::new();
        match conn.execute("SELECT * FROM CUSTOMAGEGROUP", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let lo__hi = read_u16(_batch.at(0, row_index))?;
                        let event_num = read_u16(_batch.at(1, row_index))?;
                        let stroke = read_u16(_batch.at(2, row_index))?;
                        let distance = read_u16(_batch.at(3, row_index))?;
                        let obj = Customagegroup {
                            lo__hi,
                            event_num,
                            stroke,
                            distance
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
