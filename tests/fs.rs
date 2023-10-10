use std::path::Path;

use spreet::error::Error;
use spreet::fs::get_svg_input_paths;

#[test]
fn get_svg_input_paths_returns_non_recursive_results() {
    assert_eq!(
        get_svg_input_paths(Path::new("tests/fixtures/svgs"), false).unwrap(),
        vec![
            Path::new("tests/fixtures/svgs/circle.svg"),
            Path::new("tests/fixtures/svgs/another_bicycle.svg"),
            Path::new("tests/fixtures/svgs/bicycle.svg"),
        ]
    );
}

#[test]
fn get_svg_input_paths_returns_recursive_results() {
    assert_eq!(
        get_svg_input_paths(Path::new("tests/fixtures/svgs"), true).unwrap(),
        vec![
            Path::new("tests/fixtures/svgs/circle.svg"),
            Path::new("tests/fixtures/svgs/another_bicycle.svg"),
            Path::new("tests/fixtures/svgs/recursive/bear.svg"),
            Path::new("tests/fixtures/svgs/bicycle.svg"),
        ]
    );
}

#[test]
fn get_svg_input_paths_returns_error_when_path_does_not_exist() {
    assert_eq!(
        get_svg_input_paths(Path::new("fake"), false),
        Err(Error::IoError),
    );
}

#[test]
fn get_svg_input_paths_returns_error_when_path_is_file() {
    assert_eq!(
        get_svg_input_paths(Path::new("tests/fixtures/svgs/bicycle.svg"), false),
        Err(Error::IoError),
    );
}