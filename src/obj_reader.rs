use std::fs::read;
use crate::PT;
pub fn read_obj() -> Vec<PT> {
    let binary_contents = read("./landscape_full.obj").unwrap();
    // convert to string from utf8`
    let string_contents = String::from_utf8(binary_contents).unwrap();
    // split on the newlines
    let mut point_holder = vec![];
    for line in string_contents.split('\n') {
        if line.contains("v ") {
            // split on the spaces, but skip the first
            let mut coord_iter = line.split(' ').skip(1);
            // get x and convert
            let x = coord_iter.next().unwrap().parse::<f32>().unwrap();
            let y = coord_iter.next().unwrap().parse::<f32>().unwrap();
            let z = coord_iter.next().unwrap().parse::<f32>().unwrap();
            // add z later
            point_holder.push(PT::new(x,z));
        }

    }
    println!("point holder {:?}",point_holder);
    return point_holder;
}