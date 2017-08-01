extern crate direct2d;

use direct2d::Factory;
use direct2d::geometry::{Geometry, FillMode, FigureBegin, FigureEnd};
use direct2d::math::*;

const EPSILON: f32 = 0.0001;

#[test]
fn rectangle_area() {
    let factory = Factory::new().unwrap();
    
    let rect = RectF::new(0.0, 0.0, 1.0, 1.0);
    let rectangle = factory.create_rectangle_geometry(&rect).unwrap();
    
    let area = rectangle.compute_area(None).unwrap();
    assert!((area - 1.0).abs() <= EPSILON);
}

#[test]
fn rectangle_length() {
    let factory = Factory::new().unwrap();
    
    let rect = RectF::new(0.0, 0.0, 1.0, 1.0);
    let rectangle = factory.create_rectangle_geometry(&rect).unwrap();
    
    let area = rectangle.compute_length(None).unwrap();
    assert!((area - 4.0).abs() <= EPSILON);
}

#[test]
fn combined_area() {
    let factory = Factory::new().unwrap();
    
    let rect1 = RectF::new(0.0, 0.0, 1.0, 1.0);
    let rectangle1 = factory.create_rectangle_geometry(&rect1).unwrap();
    
    let rect2 = RectF::new(0.0, 1.0, 1.0, 2.0);
    let rectangle2 = factory.create_rectangle_geometry(&rect2).unwrap();
    
    let combined = factory.create_geometry_group(FillMode::Winding, &[&rectangle1, &rectangle2]).unwrap();
    
    let area = combined.compute_area(None).unwrap();
    assert!((area - 2.0).abs() <= EPSILON);
}

#[test]
fn transformed_area() {
    let factory = Factory::new().unwrap();
    
    let rect = RectF::new(0.0, 0.0, 1.0, 1.0);
    let rectangle = factory.create_rectangle_geometry(&rect).unwrap();
    
    for x in 1..5 {
        for y in 1..5 {
            for t in -10..10 {
                let x = x as f32;
                let y = y as f32;
                let t = t as f32;
                let real_area = x * y;
                
                let transform = Matrix3x2F::new([
                    [  x, 0.0], // Scale along the X axis by `x`
                    [0.0,   y], // Scale along the Y axis by `y`
                    [  t,  -t], // this value shouldn't affect area (translation component)
                ]);
                
                // Apply the transformation to get the area
                let area = rectangle.compute_area(Some(&transform)).unwrap();
                assert!((area - real_area).abs() <= EPSILON);
                
                // Create a permanently transformed geometry and test its base area
                let transformed = rectangle.transformed(&transform).unwrap();
                let area = transformed.compute_area(None).unwrap();
                assert!((area - real_area).abs() <= EPSILON);
                
                // Double-transform
                let area = transformed.compute_area(Some(&transform)).unwrap();
                assert!((area - real_area*real_area).abs() <= EPSILON);
            }
        }
    }
}

#[test]
fn path_geometry() {
    let factory = Factory::new().unwrap();
    
    /* It looks something like this:
        - -
      - - - -
    - - - - - -
    - -     - -
    - -     - -
    - - - - - -
      - - - -
        - -
    */
    
    let mut path = factory.create_path_geometry().unwrap();
    path.open().unwrap()
        .fill_mode(FillMode::Winding)
        // Square with a triangle base
        .begin_figure(Point2F::new(0.0, 0.0), FigureBegin::Filled, FigureEnd::Closed)
            .add_line(Point2F::new(1.0, 0.0))
            .add_line(Point2F::new(1.0, 1.0))
            .add_line(Point2F::new(0.5, 1.5))
            .add_line(Point2F::new(0.0, 1.0))
            .add_line(Point2F::new(0.0, 0.0))
        .end()
        // Add a triangle hat
        .begin_figure(Point2F::new(0.0, 0.0), FigureBegin::Filled, FigureEnd::Closed)
            .add_line(Point2F::new(0.5, -0.5))
            .add_line(Point2F::new(1.0, 0.0))
        .end()
        // Cut a hole in the middle
        .fill_mode(FillMode::Alternate)
        .begin_figure(Point2F::new(0.25, 0.25), FigureBegin::Filled, FigureEnd::Closed)
            .add_line(Point2F::new(0.75, 0.25))
            .add_line(Point2F::new(0.75, 0.75))
            .add_line(Point2F::new(0.25, 0.75))
        .end()
    .close();
    
    assert!(path.open().is_err());
    
    let real_area = 1.25;
    let area = path.compute_area(None).unwrap();
    assert!((area - real_area).abs() <= EPSILON);
}

#[test]
fn to_generic_and_back() {
    let factory = Factory::new().unwrap();
    
    let rect = RectF::new(0.0, 0.0, 1.0, 1.0);
    let rectangle = factory.create_rectangle_geometry(&rect).unwrap();
    let generic = rectangle.to_generic();
    let rectangle = generic.as_rectangle().unwrap();
    assert_eq!(rectangle.get_rect(), rect);
}

