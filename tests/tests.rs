use assert_cmd::{assert::Assert, Command};

#[test]
fn noargs() {
    assert("", &[]).success().stdout("");
}

#[test]
fn duplicate_line() {
    assert("a\na\na\n", &[]).success().stdout("a\n");
}

#[test]
fn unique_lines() {
    assert("a\nb\nc\n", &[]).success().stdout("a\nb\nc\n");
}

#[test]
fn count_sort() {
    assert("a\na\nb\n", &["-c", "-s"]).success().stdout("1 b\n2 a\n");
}

#[test]
fn count_sort_descending() {
    assert("a\na\nb\n", &["-c", "-S"]).success().stdout("2 a\n1 b\n");
}

fn assert(input: &str, args: &[&str]) -> Assert {
    let mut cmd = Command::cargo_bin("huniq").unwrap();
    cmd.args(args).write_stdin(input).assert()
}
