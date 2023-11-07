// use std::fs::File;
// use std::io::Write;

use xdiff::diff_text;

#[test]
fn diff_text_should_work() {
    let text1 = "hello\nbar";
    let text2 = "hello\nbaz";

    let res = include_str!("../fixtures/diff1.txt");
    let (output, _output1, _output2) = diff_text(text1, text2).unwrap();

    // let mut file = File::create("fixtures/diff1.txt").unwrap();
    // write!(&mut file, "{}", output).unwrap();

    assert_eq!(output, res.to_string());
}