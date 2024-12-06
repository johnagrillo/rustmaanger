// 
// 
// CREATE TABLE [2023_ALL]
// (
// [Std] Long Integer,
// [Lo_Age] Integer,
// [Hi_age] Integer,
// [F(0)] Long Integer,
// [F(1)] Long Integer,
// [F(2)] Long Integer,
// [F(3)] Long Integer,
// [F(4)] Long Integer,
// [F(5)] Long Integer,
// [F(6)] Long Integer,
// [F(7)] Long Integer,
// [F(8)] Long Integer,
// [F(9)] Long Integer,
// [F(10)] Long Integer,
// [F(11)] Long Integer,
// [F(12)] Long Integer,
// [F(13)] Long Integer,
// [F(14)] Long Integer,
// [F(15)] Long Integer,
// [F(16)] Long Integer,
// [F(17)] Long Integer,
// [F(18)] Long Integer,
// [F(19)] Long Integer,
// [F(20)] Long Integer,
// [F(21)] Long Integer,
// [F(22)] Long Integer,
// [F(23)] Long Integer,
// [F(24)] Long Integer,
// [F(25)] Long Integer,
// [F(26)] Long Integer,
// [F(27)] Long Integer,
// [F(28)] Long Integer,
// [F(29)] Long Integer,
// [F(30)] Long Integer,
// [F(31)] Long Integer,
// [F(32)] Long Integer,
// [F(33)] Long Integer,
// [F(34)] Long Integer,
// [F(35)] Long Integer,
// [S(0)] Long Integer,
// [S(1)] Long Integer,
// [S(2)] Long Integer,
// [S(3)] Long Integer,
// [S(4)] Long Integer,
// [S(5)] Long Integer,
// [S(6)] Long Integer,
// [S(7)] Long Integer,
// [S(8)] Long Integer,
// [S(9)] Long Integer,
// [S(10)] Long Integer,
// [S(11)] Long Integer,
// [S(12)] Long Integer,
// [S(13)] Long Integer,
// [S(14)] Long Integer,
// [S(15)] Long Integer,
// [S(16)] Long Integer,
// [S(17)] Long Integer,
// [S(18)] Long Integer,
// [S(19)] Long Integer,
// [S(20)] Long Integer,
// [S(21)] Long Integer,
// [S(22)] Long Integer,
// [S(23)] Long Integer,
// [S(24)] Long Integer,
// [S(25)] Long Integer,
// [S(26)] Long Integer,
// [S(27)] Long Integer,
// [S(28)] Long Integer,
// [S(29)] Long Integer,
// [S(30)] Long Integer,
// [S(31)] Long Integer,
// [S(32)] Long Integer,
// [S(33)] Long Integer,
// [S(34)] Long Integer,
// [S(35)] Long Integer,
// [Distance] Integer,
// [Stroke] Integer,
// [Sex] Text (2),
// [I_R] Text (2),
// [Division] Text (4)
// );
#[derive(Serialize,Deserialize,Debug)]
struct 2023_all {
    std : Option<u64>,
    lo__age : Option<u16>,
    hi_age : Option<u16>,
    F_0_ : Option<u64>,
    F_1_ : Option<u64>,
    F_2_ : Option<u64>,
    F_3_ : Option<u64>,
    F_4_ : Option<u64>,
    F_5_ : Option<u64>,
    F_6_ : Option<u64>,
    F_7_ : Option<u64>,
    F_8_ : Option<u64>,
    F_9_ : Option<u64>,
    F_10_ : Option<u64>,
    F_11_ : Option<u64>,
    F_12_ : Option<u64>,
    F_13_ : Option<u64>,
    F_14_ : Option<u64>,
    F_15_ : Option<u64>,
    F_16_ : Option<u64>,
    F_17_ : Option<u64>,
    F_18_ : Option<u64>,
    F_19_ : Option<u64>,
    F_20_ : Option<u64>,
    F_21_ : Option<u64>,
    F_22_ : Option<u64>,
    F_23_ : Option<u64>,
    F_24_ : Option<u64>,
    F_25_ : Option<u64>,
    F_26_ : Option<u64>,
    F_27_ : Option<u64>,
    F_28_ : Option<u64>,
    F_29_ : Option<u64>,
    F_30_ : Option<u64>,
    F_31_ : Option<u64>,
    F_32_ : Option<u64>,
    F_33_ : Option<u64>,
    F_34_ : Option<u64>,
    F_35_ : Option<u64>,
    S_0_ : Option<u64>,
    S_1_ : Option<u64>,
    S_2_ : Option<u64>,
    S_3_ : Option<u64>,
    S_4_ : Option<u64>,
    S_5_ : Option<u64>,
    S_6_ : Option<u64>,
    S_7_ : Option<u64>,
    S_8_ : Option<u64>,
    S_9_ : Option<u64>,
    S_10_ : Option<u64>,
    S_11_ : Option<u64>,
    S_12_ : Option<u64>,
    S_13_ : Option<u64>,
    S_14_ : Option<u64>,
    S_15_ : Option<u64>,
    S_16_ : Option<u64>,
    S_17_ : Option<u64>,
    S_18_ : Option<u64>,
    S_19_ : Option<u64>,
    S_20_ : Option<u64>,
    S_21_ : Option<u64>,
    S_22_ : Option<u64>,
    S_23_ : Option<u64>,
    S_24_ : Option<u64>,
    S_25_ : Option<u64>,
    S_26_ : Option<u64>,
    S_27_ : Option<u64>,
    S_28_ : Option<u64>,
    S_29_ : Option<u64>,
    S_30_ : Option<u64>,
    S_31_ : Option<u64>,
    S_32_ : Option<u64>,
    S_33_ : Option<u64>,
    S_34_ : Option<u64>,
    S_35_ : Option<u64>,
    distance : Option<u16>,
    stroke : Option<u16>,
    sex : Option<String>,
    I_R : Option<String>,
    division : Option<String>
}
impl 2023_all {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<2023_all>, Box<dyn Error>> {
       let mut vec: Vec<2023_all> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: 2023_all = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<2023_all>, Box<dyn Error>> {
        let mut vec: Vec<2023_all> = Vec::new();
        match conn.execute("SELECT * FROM 2023_ALL", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let std = read_u64(_batch.at(0, row_index))?;
                        let lo__age = read_u16(_batch.at(1, row_index))?;
                        let hi_age = read_u16(_batch.at(2, row_index))?;
                        let F_0_ = read_u64(_batch.at(3, row_index))?;
                        let F_1_ = read_u64(_batch.at(4, row_index))?;
                        let F_2_ = read_u64(_batch.at(5, row_index))?;
                        let F_3_ = read_u64(_batch.at(6, row_index))?;
                        let F_4_ = read_u64(_batch.at(7, row_index))?;
                        let F_5_ = read_u64(_batch.at(8, row_index))?;
                        let F_6_ = read_u64(_batch.at(9, row_index))?;
                        let F_7_ = read_u64(_batch.at(10, row_index))?;
                        let F_8_ = read_u64(_batch.at(11, row_index))?;
                        let F_9_ = read_u64(_batch.at(12, row_index))?;
                        let F_10_ = read_u64(_batch.at(13, row_index))?;
                        let F_11_ = read_u64(_batch.at(14, row_index))?;
                        let F_12_ = read_u64(_batch.at(15, row_index))?;
                        let F_13_ = read_u64(_batch.at(16, row_index))?;
                        let F_14_ = read_u64(_batch.at(17, row_index))?;
                        let F_15_ = read_u64(_batch.at(18, row_index))?;
                        let F_16_ = read_u64(_batch.at(19, row_index))?;
                        let F_17_ = read_u64(_batch.at(20, row_index))?;
                        let F_18_ = read_u64(_batch.at(21, row_index))?;
                        let F_19_ = read_u64(_batch.at(22, row_index))?;
                        let F_20_ = read_u64(_batch.at(23, row_index))?;
                        let F_21_ = read_u64(_batch.at(24, row_index))?;
                        let F_22_ = read_u64(_batch.at(25, row_index))?;
                        let F_23_ = read_u64(_batch.at(26, row_index))?;
                        let F_24_ = read_u64(_batch.at(27, row_index))?;
                        let F_25_ = read_u64(_batch.at(28, row_index))?;
                        let F_26_ = read_u64(_batch.at(29, row_index))?;
                        let F_27_ = read_u64(_batch.at(30, row_index))?;
                        let F_28_ = read_u64(_batch.at(31, row_index))?;
                        let F_29_ = read_u64(_batch.at(32, row_index))?;
                        let F_30_ = read_u64(_batch.at(33, row_index))?;
                        let F_31_ = read_u64(_batch.at(34, row_index))?;
                        let F_32_ = read_u64(_batch.at(35, row_index))?;
                        let F_33_ = read_u64(_batch.at(36, row_index))?;
                        let F_34_ = read_u64(_batch.at(37, row_index))?;
                        let F_35_ = read_u64(_batch.at(38, row_index))?;
                        let S_0_ = read_u64(_batch.at(39, row_index))?;
                        let S_1_ = read_u64(_batch.at(40, row_index))?;
                        let S_2_ = read_u64(_batch.at(41, row_index))?;
                        let S_3_ = read_u64(_batch.at(42, row_index))?;
                        let S_4_ = read_u64(_batch.at(43, row_index))?;
                        let S_5_ = read_u64(_batch.at(44, row_index))?;
                        let S_6_ = read_u64(_batch.at(45, row_index))?;
                        let S_7_ = read_u64(_batch.at(46, row_index))?;
                        let S_8_ = read_u64(_batch.at(47, row_index))?;
                        let S_9_ = read_u64(_batch.at(48, row_index))?;
                        let S_10_ = read_u64(_batch.at(49, row_index))?;
                        let S_11_ = read_u64(_batch.at(50, row_index))?;
                        let S_12_ = read_u64(_batch.at(51, row_index))?;
                        let S_13_ = read_u64(_batch.at(52, row_index))?;
                        let S_14_ = read_u64(_batch.at(53, row_index))?;
                        let S_15_ = read_u64(_batch.at(54, row_index))?;
                        let S_16_ = read_u64(_batch.at(55, row_index))?;
                        let S_17_ = read_u64(_batch.at(56, row_index))?;
                        let S_18_ = read_u64(_batch.at(57, row_index))?;
                        let S_19_ = read_u64(_batch.at(58, row_index))?;
                        let S_20_ = read_u64(_batch.at(59, row_index))?;
                        let S_21_ = read_u64(_batch.at(60, row_index))?;
                        let S_22_ = read_u64(_batch.at(61, row_index))?;
                        let S_23_ = read_u64(_batch.at(62, row_index))?;
                        let S_24_ = read_u64(_batch.at(63, row_index))?;
                        let S_25_ = read_u64(_batch.at(64, row_index))?;
                        let S_26_ = read_u64(_batch.at(65, row_index))?;
                        let S_27_ = read_u64(_batch.at(66, row_index))?;
                        let S_28_ = read_u64(_batch.at(67, row_index))?;
                        let S_29_ = read_u64(_batch.at(68, row_index))?;
                        let S_30_ = read_u64(_batch.at(69, row_index))?;
                        let S_31_ = read_u64(_batch.at(70, row_index))?;
                        let S_32_ = read_u64(_batch.at(71, row_index))?;
                        let S_33_ = read_u64(_batch.at(72, row_index))?;
                        let S_34_ = read_u64(_batch.at(73, row_index))?;
                        let S_35_ = read_u64(_batch.at(74, row_index))?;
                        let distance = read_u16(_batch.at(75, row_index))?;
                        let stroke = read_u16(_batch.at(76, row_index))?;
                        let sex = read_string(_batch.at(77, row_index))?;
                        let I_R = read_string(_batch.at(78, row_index))?;
                        let division = read_string(_batch.at(79, row_index))?;
                        let obj = 2023_all {
                            std,
                            lo__age,
                            hi_age,
                            F_0_,
                            F_1_,
                            F_2_,
                            F_3_,
                            F_4_,
                            F_5_,
                            F_6_,
                            F_7_,
                            F_8_,
                            F_9_,
                            F_10_,
                            F_11_,
                            F_12_,
                            F_13_,
                            F_14_,
                            F_15_,
                            F_16_,
                            F_17_,
                            F_18_,
                            F_19_,
                            F_20_,
                            F_21_,
                            F_22_,
                            F_23_,
                            F_24_,
                            F_25_,
                            F_26_,
                            F_27_,
                            F_28_,
                            F_29_,
                            F_30_,
                            F_31_,
                            F_32_,
                            F_33_,
                            F_34_,
                            F_35_,
                            S_0_,
                            S_1_,
                            S_2_,
                            S_3_,
                            S_4_,
                            S_5_,
                            S_6_,
                            S_7_,
                            S_8_,
                            S_9_,
                            S_10_,
                            S_11_,
                            S_12_,
                            S_13_,
                            S_14_,
                            S_15_,
                            S_16_,
                            S_17_,
                            S_18_,
                            S_19_,
                            S_20_,
                            S_21_,
                            S_22_,
                            S_23_,
                            S_24_,
                            S_25_,
                            S_26_,
                            S_27_,
                            S_28_,
                            S_29_,
                            S_30_,
                            S_31_,
                            S_32_,
                            S_33_,
                            S_34_,
                            S_35_,
                            distance,
                            stroke,
                            sex,
                            I_R,
                            division
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
