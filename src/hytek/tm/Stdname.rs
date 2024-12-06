// 
// 
// CREATE TABLE [STDNAME]
// (
// [StdFile] Text (16),
// [Year] Text (8),
// [Descript] Text (50),
// [D1] Text (8),
// [D2] Text (8),
// [D3] Text (8),
// [D4] Text (8),
// [D5] Text (8),
// [D6] Text (8),
// [D7] Text (8),
// [D8] Text (8),
// [D9] Text (8),
// [D10] Text (8),
// [D11] Text (8),
// [D12] Text (8),
// [D1Des] Text (30),
// [D2Des] Text (30),
// [D3Des] Text (30),
// [D4Des] Text (30),
// [D5Des] Text (30),
// [D6Des] Text (30),
// [D7Des] Text (30),
// [D8Des] Text (30),
// [D9Des] Text (30),
// [D10Des] Text (30),
// [D11Des] Text (30),
// [D12Des] Text (30)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Stdname {
    std_file : Option<String>,
    year : Option<String>,
    descript : Option<String>,
    D1 : Option<String>,
    D2 : Option<String>,
    D3 : Option<String>,
    D4 : Option<String>,
    D5 : Option<String>,
    D6 : Option<String>,
    D7 : Option<String>,
    D8 : Option<String>,
    D9 : Option<String>,
    D10 : Option<String>,
    D11 : Option<String>,
    D12 : Option<String>,
    d1_des : Option<String>,
    d2_des : Option<String>,
    d3_des : Option<String>,
    d4_des : Option<String>,
    d5_des : Option<String>,
    d6_des : Option<String>,
    d7_des : Option<String>,
    d8_des : Option<String>,
    d9_des : Option<String>,
    d1_0_des : Option<String>,
    d1_1_des : Option<String>,
    d1_2_des : Option<String>
}
impl Stdname {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Stdname>, Box<dyn Error>> {
       let mut vec: Vec<Stdname> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Stdname = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Stdname>, Box<dyn Error>> {
        let mut vec: Vec<Stdname> = Vec::new();
        match conn.execute("SELECT * FROM STDNAME", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let std_file = read_string(_batch.at(0, row_index))?;
                        let year = read_string(_batch.at(1, row_index))?;
                        let descript = read_string(_batch.at(2, row_index))?;
                        let D1 = read_string(_batch.at(3, row_index))?;
                        let D2 = read_string(_batch.at(4, row_index))?;
                        let D3 = read_string(_batch.at(5, row_index))?;
                        let D4 = read_string(_batch.at(6, row_index))?;
                        let D5 = read_string(_batch.at(7, row_index))?;
                        let D6 = read_string(_batch.at(8, row_index))?;
                        let D7 = read_string(_batch.at(9, row_index))?;
                        let D8 = read_string(_batch.at(10, row_index))?;
                        let D9 = read_string(_batch.at(11, row_index))?;
                        let D10 = read_string(_batch.at(12, row_index))?;
                        let D11 = read_string(_batch.at(13, row_index))?;
                        let D12 = read_string(_batch.at(14, row_index))?;
                        let d1_des = read_string(_batch.at(15, row_index))?;
                        let d2_des = read_string(_batch.at(16, row_index))?;
                        let d3_des = read_string(_batch.at(17, row_index))?;
                        let d4_des = read_string(_batch.at(18, row_index))?;
                        let d5_des = read_string(_batch.at(19, row_index))?;
                        let d6_des = read_string(_batch.at(20, row_index))?;
                        let d7_des = read_string(_batch.at(21, row_index))?;
                        let d8_des = read_string(_batch.at(22, row_index))?;
                        let d9_des = read_string(_batch.at(23, row_index))?;
                        let d1_0_des = read_string(_batch.at(24, row_index))?;
                        let d1_1_des = read_string(_batch.at(25, row_index))?;
                        let d1_2_des = read_string(_batch.at(26, row_index))?;
                        let obj = Stdname {
                            std_file,
                            year,
                            descript,
                            D1,
                            D2,
                            D3,
                            D4,
                            D5,
                            D6,
                            D7,
                            D8,
                            D9,
                            D10,
                            D11,
                            D12,
                            d1_des,
                            d2_des,
                            d3_des,
                            d4_des,
                            d5_des,
                            d6_des,
                            d7_des,
                            d8_des,
                            d9_des,
                            d1_0_des,
                            d1_1_des,
                            d1_2_des
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
