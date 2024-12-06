// 
// 
// CREATE TABLE [MTEVENTE]
// (
// [Meet] Long Integer,
// [MtEv] Integer,
// [MtEvX] Text (2),
// [Lo_Hi] Integer,
// [FastCut] Long Integer,
// [SlowCut] Long Integer,
// [Course] Text (2),
// [MtEvent] Long Integer,
// [Fast_L] Long Integer,
// [Slow_L] Long Integer,
// [Fast_Y] Long Integer,
// [Slow_Y] Long Integer,
// [TstdFile] Text (16),
// [TstDesig] Text (8),
// [Distance] Integer,
// [Stroke] Integer,
// [Sex] Text (2),
// [I_R] Text (2),
// [Session] Byte,
// [Division] Text (6),
// [Fee] Single,
// [ExportEvent] Text (8),
// [GenderMix] Byte,
// [SessX] Text (2)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Mtevente {
    meet : Option<u64>,
    mt_ev : Option<u16>,
    mt_ev_x : Option<String>,
    lo__hi : Option<u16>,
    fast_cut : Option<u64>,
    slow_cut : Option<u64>,
    course : Option<String>,
    mt_event : Option<u64>,
    fast__l : Option<u64>,
    slow__l : Option<u64>,
    fast__y : Option<u64>,
    slow__y : Option<u64>,
    tstd_file : Option<String>,
    tst_desig : Option<String>,
    distance : Option<u16>,
    stroke : Option<u16>,
    sex : Option<String>,
    I_R : Option<String>,
    session : Option<u8>,
    division : Option<String>,
    fee : Option<f32>,
    export_event : Option<String>,
    gender_mix : Option<u8>,
    sess_x : Option<String>
}
impl Mtevente {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Mtevente>, Box<dyn Error>> {
       let mut vec: Vec<Mtevente> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Mtevente = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Mtevente>, Box<dyn Error>> {
        let mut vec: Vec<Mtevente> = Vec::new();
        match conn.execute("SELECT * FROM MTEVENTE", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let meet = read_u64(_batch.at(0, row_index))?;
                        let mt_ev = read_u16(_batch.at(1, row_index))?;
                        let mt_ev_x = read_string(_batch.at(2, row_index))?;
                        let lo__hi = read_u16(_batch.at(3, row_index))?;
                        let fast_cut = read_u64(_batch.at(4, row_index))?;
                        let slow_cut = read_u64(_batch.at(5, row_index))?;
                        let course = read_string(_batch.at(6, row_index))?;
                        let mt_event = read_u64(_batch.at(7, row_index))?;
                        let fast__l = read_u64(_batch.at(8, row_index))?;
                        let slow__l = read_u64(_batch.at(9, row_index))?;
                        let fast__y = read_u64(_batch.at(10, row_index))?;
                        let slow__y = read_u64(_batch.at(11, row_index))?;
                        let tstd_file = read_string(_batch.at(12, row_index))?;
                        let tst_desig = read_string(_batch.at(13, row_index))?;
                        let distance = read_u16(_batch.at(14, row_index))?;
                        let stroke = read_u16(_batch.at(15, row_index))?;
                        let sex = read_string(_batch.at(16, row_index))?;
                        let I_R = read_string(_batch.at(17, row_index))?;
                        let session = read_u8(_batch.at(18, row_index))?;
                        let division = read_string(_batch.at(19, row_index))?;
                        let fee = read_f32(_batch.at(20, row_index))?;
                        let export_event = read_string(_batch.at(21, row_index))?;
                        let gender_mix = read_u8(_batch.at(22, row_index))?;
                        let sess_x = read_string(_batch.at(23, row_index))?;
                        let obj = Mtevente {
                            meet,
                            mt_ev,
                            mt_ev_x,
                            lo__hi,
                            fast_cut,
                            slow_cut,
                            course,
                            mt_event,
                            fast__l,
                            slow__l,
                            fast__y,
                            slow__y,
                            tstd_file,
                            tst_desig,
                            distance,
                            stroke,
                            sex,
                            I_R,
                            session,
                            division,
                            fee,
                            export_event,
                            gender_mix,
                            sess_x
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
