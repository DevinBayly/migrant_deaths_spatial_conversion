use std::fs::read;
use crate::MeshPT;
#[derive(Debug,Clone)]
pub struct MigrantEle {
    pub pt:MeshPT,
    pub info:String,
}

pub fn read_csv() -> Vec<MigrantEle> {
    let mut res = vec![];
    let binary_contents = read("./death_points_n33_w113.csv").unwrap();
    let string_contents = String::from_utf8(binary_contents).unwrap();
    // 
    // skip the first line since its the header
    for line in string_contents.split('\n').skip(1) {
        let mut individual_columns:Vec<&str> = line.split(',').collect();
        // its column (21 norm_lat z) (22 norm_lng x) that have normalized data
        let x = individual_columns.get(22).unwrap().parse::<f32>().unwrap();
        let z = individual_columns.get(21).unwrap().parse::<f32>().unwrap();
        let m_pt = MeshPT::new(x,0.0,z);
        let m_e = MigrantEle{
            pt:m_pt,
            info:line.to_string()
        };
        res.push(m_e);

    }
    res
}