use crate::MeshPT;
use csv;
use std::io::Read;
use std::fs::File;
#[derive(Debug, Clone)]
pub struct MigrantEle {
    pub pt: MeshPT,
    pub info: Vec<String>,
}

pub fn read_csv() -> Vec<MigrantEle> {
    let mut res = vec![];
    // create a csv reader
    let f = File::open("./death_points_n33_w113.csv").unwrap();
    //
    let mut rdr = csv::Reader::from_reader(f);

    // skip the first line since its the header
    for record in rdr.records() {
        let record = record.unwrap();
        //println!("line is {:?}",line);
        //let mut individual_columns: Vec<&str> = line.split(',').collect();
        //// its column (21 norm_lat z) (22 norm_lng x) that have normalized data
        //println!("column {:?}", individual_columns);
        let x_str = record.get(22).unwrap();
        let z_str = record.get(21).unwrap();
        let x = x_str.parse::<f32>().unwrap();
        let z = z_str.parse::<f32>().unwrap();

        let record_string = record.iter().map(|e| e.to_string()).collect::<Vec<String>>();
        let m_pt = MeshPT::new(x, 0.0, z);
        let m_e = MigrantEle {
            pt: m_pt,
            info: record_string,
        };
        res.push(m_e);
    }
    res
}
