use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 6"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "12");
    assert!(output.stderr_str().is_empty());
}



