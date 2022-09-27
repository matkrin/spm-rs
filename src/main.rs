use std::fs::{File, read};
use std::io::BufReader;
use std::io::Read;
// use std::path::Path;


fn main() {
    // let file = File::open("stm-aarhus-mul-a.mul").expect("Not able to open file");
    // let mut reader = BufReader::new(file);
    // let capacity = reader.capacity();

    const MUL_BLOCK: i32 = 128;
    // println!("{:?}", reader.bytes());
    let bytes = read("stm-aarhus-mul-a.mul").unwrap();
    println!("{}", bytes.len());
    // println!("{:?}", bytes);
    // let a: [u8; 2] = bytes[0..2].try_into().expect("to be ok");
    // let b: [u8; 4] = bytes[2..6].try_into().expect("to be ok");
    // let c: [u8; 2] = bytes[384..386].try_into().expect("h");
    // let d: [u8; 2] = bytes[386..388].try_into().expect("h");
    // let e: [u8; 2] = bytes[388..390].try_into().expect("h");
    // let f: [u8; 2] = bytes[390..392].try_into().expect("h");
    // let g: [u8; 2] = bytes[392..394].try_into().expect("h");
    // let h: [u8; 2] = bytes[394..396].try_into().expect("h");
    // // println!("{:?}", a);
    let nr = i16::from_le_bytes(bytes[0..2].try_into().unwrap());
    let adr = i32::from_le_bytes(bytes[2..6].try_into().unwrap());
    // let adr = i32::from_le_bytes(b);
    // let img_num = i16::from_le_bytes(c);
    // let size = i16::from_le_bytes(d);
    // let xres = i16::from_le_bytes(e);
    // let yres = i16::from_le_bytes(f);
    // let zres = i16::from_le_bytes(g);
    // let year = i16::from_le_bytes(h);
    //
    // println!("nr: {}", nr);
    // println!("adr: {}", adr);
    // println!("img_num: {}", img_num);
    // println!("size: {}", size);
    // println!("xres: {}", xres);
    // println!("yres: {}", yres);
    // println!("zres: {}", zres);
    // println!("year: {}", year);
    // for b in reader.bytes() {
    //     println!("{}", b.unwrap());
    // }

    let block_counter = 0;
    while (block_counter * MUL_BLOCK) < bytes.len().try_into().unwrap() {
        let mut start: usize = (adr * MUL_BLOCK).try_into().unwrap();
        let img_num = i16::from_le_bytes(bytes[start..start+2].try_into().unwrap());
        println!("{}", img_num);


        break;        
        // block_counter += size;
    }

}
