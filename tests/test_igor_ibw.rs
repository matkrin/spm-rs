use spm_rs::igor_ibw::read_ibw;

const IBW_MATRIX: &str = "tests/test_files/test_matrix.ibw";

#[test]
fn test_npnts() {
    let ibw = read_ibw(IBW_MATRIX).unwrap();
    assert_eq!(ibw.npnts, 16);
}

#[test]
fn test_bname() {
    let ibw = read_ibw(IBW_MATRIX).unwrap();
    assert_eq!(ibw.bname, "test_matrix".to_string());
}

#[test]
fn test_n_dim() {
    let ibw = read_ibw(IBW_MATRIX).unwrap();
    assert_eq!(ibw.n_dim, [4, 4, 0, 0]);
}

#[test]
fn test_x_step() {
    let ibw = read_ibw(IBW_MATRIX).unwrap();
    assert_eq!(ibw.x_step, [1., 1., 1., 1.]);
}

#[test]
fn test_x_start() {
    let ibw = read_ibw(IBW_MATRIX).unwrap();
    assert_eq!(ibw.x_start, [0., 0., 0., 0.]);
}

// #[test]
// fn test_data() {
//     let ibw = read_ibw(IBW_MATRIX).unwrap();
//     assert_eq!(ibw.data, );
// }

#[test]
fn test_note() {
    let ibw = read_ibw(IBW_MATRIX).unwrap();
    assert_eq!(ibw.note, "test matrix 4x4".to_string());
}

#[test]
fn test_extended_data_units() {
    let ibw = read_ibw(IBW_MATRIX).unwrap();
    assert_eq!(ibw.extended_data_units, Some("data_units".to_string()));
}

#[test]
fn test_dim_e_units() {
    let ibw = read_ibw(IBW_MATRIX).unwrap();
    assert_eq!(
        ibw.dim_e_units,
        Some(
            vec!["row_units", "col_units", "", ""]
                .iter()
                .map(|it| it.to_string())
                .collect()
        )
    );
}

#[test]
fn test_dim_labels() {
    let ibw = read_ibw(IBW_MATRIX).unwrap();
    assert_eq!(
        ibw.dim_labels,
        Some(
            vec!["", "", "", ""]
                .iter()
                .map(|it| it.to_string())
                .collect()
        )
    );
}
