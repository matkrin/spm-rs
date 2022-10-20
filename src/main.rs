use clap::Parser;
use spm_rs::mulfile::read_mul;
use spm_rs::omicron_matrix::read_omicron_matrix;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg()]
    filename: String,
}

fn main() {
    let args = Args::parse();
    if args.filename.ends_with(".mul") || args.filename.ends_with(".flm") {
        let mulfile = read_mul(&args.filename);
        for mut i in mulfile {
            i.img_data.correct_plane();
            i.img_data.correct_lines();
            i.img_data.save_png();
        }
    } else if args.filename.ends_with(".Z_mtrx") {
        let mut omicron_matrix = read_omicron_matrix(&args.filename);
        omicron_matrix.img_data_fw.correct_plane();
        omicron_matrix.img_data_bw.correct_lines();
        omicron_matrix.img_data_fw.save_png();
        // omicron_matrix.img_data_bw.save_png();
    }
}
