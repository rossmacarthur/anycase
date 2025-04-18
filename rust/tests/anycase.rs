use std::fs;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use serde_json as json;

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    input: String,
    snake: String,
    camel: String,
    pascal: String,
    screaming_snake: String,
    kebab: String,
    screaming_kebab: String,
    train: String,
    lower: String,
    title: String,
    upper: String,
}

#[allow(clippy::incompatible_msrv)]
static CASES: LazyLock<Vec<TestCase>> = LazyLock::new(|| {
    let path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "..", "testdata", "common.json"]);
    let data = fs::read_to_string(path).unwrap();
    json::from_str(&data).unwrap()
});

#[test]
fn common_to_string() {
    for case in CASES.iter() {
        assert_eq!(
            anycase::to_snake(&case.input),
            case.snake,
            "in: {:?}, conversion: snake",
            case.input
        );
        assert_eq!(
            anycase::to_camel(&case.input),
            case.camel,
            "in: {:?}, conversion: camel",
            case.input
        );
        assert_eq!(
            anycase::to_pascal(&case.input),
            case.pascal,
            "in: {:?}, conversion: pascal",
            case.input
        );
        assert_eq!(
            anycase::to_screaming_snake(&case.input),
            case.screaming_snake,
            "in: {:?}, conversion: screaming_snake",
            case.input
        );
        assert_eq!(
            anycase::to_kebab(&case.input),
            case.kebab,
            "in: {:?}, conversion: kebab",
            case.input
        );
        assert_eq!(
            anycase::to_screaming_kebab(&case.input),
            case.screaming_kebab,
            "in: {:?}, conversion: screaming_kebab",
            case.input
        );
        assert_eq!(
            anycase::to_train(&case.input),
            case.train,
            "in: {:?}, conversion: train",
            case.input
        );
        assert_eq!(
            anycase::to_lower(&case.input),
            case.lower,
            "in: {:?}, conversion: lower",
            case.input
        );
        assert_eq!(
            anycase::to_title(&case.input),
            case.title,
            "in: {:?}, conversion: title",
            case.input
        );
        assert_eq!(
            anycase::to_upper(&case.input),
            case.upper,
            "in: {:?}, conversion: upper",
            case.input
        );
    }
}
