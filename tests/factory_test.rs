extern crate direct2d;

#[test]
fn factory_test() {
    assert!(direct2d::factory::Factory1::new().is_ok());
}

