use mulfile_rs::read_mul;

#[test]
fn test_current() {
    let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    let currents: Vec<f64> = mulfile.iter().map(|x| x.current).collect();
    assert_eq!(currents, vec![0.23, 0.43, 0.91, 0.65]);
}

#[test]
fn test_bias() {
    let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    let biases: Vec<f64> = mulfile.iter().map(|x| x.bias.round()).collect();
    assert_eq!(biases, vec![222.0, 293.0, 293.0, 293.0]);
}

#[test]
fn test_gain() {
    let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    let gains: Vec<i32> = mulfile.iter().map(|x| x.gain).collect();
    assert_eq!(gains, Vec::from([955; 4]));
}

#[test]
fn test_scan_duration() {
    let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    let scan_durations: Vec<f64> = mulfile.iter().map(|x| x.speed.round()).collect();
    assert_eq!(scan_durations, vec![77.0, 82.0, 123.0, 129.0]);
}


#[test]
fn test_lines_time() {
    let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    let line_times: Vec<f64> = mulfile.iter().map(|x| x.line_time.round()).collect();
    assert_eq!(line_times, vec![150.0, 160.0, 240.0, 252.0]);
}

