extern crate direct2d;

#[test]
fn factory_test() {
    assert!(direct2d::Factory::new().is_ok());
}

