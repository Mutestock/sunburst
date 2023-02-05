
use std::env::consts;

use scraper::scrape_tools::{construct_user_agent, ExtractionHelper};

const TEST_HTML: &'static str = "
<body>
    <div class=\"munchables\">
        <p>Cake</p>
    </div>
    <div class=\"munchables\">
        <p>Pie</p>
    </div>
    <div class=\"munchables\">
        <p>Gateau</p>
    </div>
</body>
";

#[test]
fn test_construct_user_agent() {
    let os = consts::OS;
    assert_eq!(!os.is_empty(), true);
    let user_agent = construct_user_agent();
    assert_eq!(user_agent.contains(os), true);
}

#[test]
fn test_extraction_helper_before() {
    assert_eq!(TEST_HTML.to_owned().before("munchables").contains("Cake"), false);
    assert_eq!(TEST_HTML.to_owned().before("munchables").contains("Pie"), false);
    assert_eq!(TEST_HTML.to_owned().before("munchables").contains("Gateau"), false);

    let expected = "
<body>
    <div class=\"";
    assert_eq!(TEST_HTML.to_owned().before("munchables"), expected);
}

#[test]
fn test_extraction_helper_after() {
    assert_eq!(TEST_HTML.to_owned().after("munchables").contains("Cake"), false);
    assert_eq!(TEST_HTML.to_owned().after("munchables").contains("Pie"), false);
    assert_eq!(TEST_HTML.to_owned().after("munchables").contains("Gateau"), true);
}

#[test]
fn test_extraction_helper_at() {
    assert_eq!(TEST_HTML.to_owned().at("munchables", 0).contains("Cake"), false);
    assert_eq!(TEST_HTML.to_owned().at("munchables", 0).contains("Pie"), false);
    assert_eq!(TEST_HTML.to_owned().at("munchables", 0).contains("Gateau"), false);
    
    assert_eq!(TEST_HTML.to_owned().at("munchables", 1).contains("Cake"), true);
    assert_eq!(TEST_HTML.to_owned().at("munchables", 1).contains("Pie"), false);
    assert_eq!(TEST_HTML.to_owned().at("munchables", 1).contains("Gateau"), false);
    
    assert_eq!(TEST_HTML.to_owned().at("munchables", 2).contains("Cake"), false);
    assert_eq!(TEST_HTML.to_owned().at("munchables", 2).contains("Pie"), true);
    assert_eq!(TEST_HTML.to_owned().at("munchables", 2).contains("Gateau"), false);
}

#[test]
#[should_panic]
fn test_at_overflow_panics() {
    assert_eq!(TEST_HTML.to_owned().at("munchables", 3).contains("Cake"), false);
    assert_eq!(TEST_HTML.to_owned().at("munchables", 3).contains("Pie"), true);
    assert_eq!(TEST_HTML.to_owned().at("munchables", 3).contains("Gateau"), false);
}