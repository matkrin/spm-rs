use mulfile_rs::read_mul;

fn main() {
    let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    let ids: Vec<String> = mulfile.iter().map(|x| x.img_id.clone()).collect();
    println!("Image IDs: {:#?}", ids);
}
