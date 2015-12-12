extern crate direct2d;

use direct2d::Factory;
use direct2d::geometry::{Geometry, FillMode};
use direct2d::math::*;

const EPSILON: f32 = 0.0001;

#[test]
fn rectangle_area() {
    let factory = Factory::create().unwrap();
    
    let rect = RectF::new(0.0, 0.0, 1.0, 1.0);
    let rectangle = factory.create_rectangle_geometry(&rect).unwrap();
    
    let area = rectangle.compute_area(None).unwrap();
    assert!((area - 1.0).abs() <= EPSILON);
}

#[test]
fn combined_area() {
    let factory = Factory::create().unwrap();
    
    let rect1 = RectF::new(0.0, 0.0, 1.0, 1.0);
    let rectangle1 = factory.create_rectangle_geometry(&rect1).unwrap();
    
    let rect2 = RectF::new(0.0, 1.0, 1.0, 2.0);
    let rectangle2 = factory.create_rectangle_geometry(&rect2).unwrap();
    
    let combined = factory.create_geometry_group(FillMode::Winding, &[&rectangle1, &rectangle2]).unwrap();
    
    let area = combined.compute_area(None).unwrap();
    assert!((area - 2.0).abs() <= EPSILON);
}

