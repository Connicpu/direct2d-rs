extern crate direct2d;

#[test]
fn factory_test() {
    assert!(direct2d::Factory::create().is_ok());
}

