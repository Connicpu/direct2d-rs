extern crate direct2d;
extern crate math2d;

use direct2d::enums::FillMode;
use direct2d::factory::Factory;
use direct2d::geometry::{GroupGeometry, IGeometry, PathGeometry, RectangleGeometry};
use math2d::*;

const EPSILON: f32 = 0.0001;

#[test]
fn rectangle_area() {
    let factory = Factory::new().unwrap();

    let rect = Rectf::new(0.0, 0.0, 1.0, 1.0);
    let rectangle = RectangleGeometry::create(&factory, &rect).unwrap();

    let area = rectangle.compute_area(None).unwrap();
    assert!((area - 1.0).abs() <= EPSILON);
}

#[test]
fn rectangle_length() {
    let factory = Factory::new().unwrap();

    let rect = Rectf::new(0.0, 0.0, 1.0, 1.0);
    let rectangle = RectangleGeometry::create(&factory, &rect).unwrap();

    let area = rectangle.compute_length(None).unwrap();
    assert!((area - 4.0).abs() <= EPSILON);
}

#[test]
fn combined_area() {
    use direct2d::enums::FillMode::*;

    let factory = Factory::new().unwrap();

    let rect1 = Rectf::new(0.0, 0.0, 1.0, 1.0);
    let rectangle1 = RectangleGeometry::create(&factory, &rect1).unwrap();

    let rect2 = Rectf::new(0.0, 1.0, 1.0, 2.0);
    let rectangle2 = RectangleGeometry::create(&factory, &rect2).unwrap();

    let list = (rectangle1, rectangle2);
    let combined = GroupGeometry::create(&factory, Winding, &list).unwrap();

    let area = combined.compute_area(None).unwrap();
    assert!((area - 2.0).abs() <= EPSILON);
}

#[test]
fn transformed_area() {
    let factory = Factory::new().unwrap();

    let rect = Rectf::new(0.0, 0.0, 1.0, 1.0);
    let rectangle = RectangleGeometry::create(&factory, &rect).unwrap();

    for x in 1..5 {
        for y in 1..5 {
            for t in -10..10 {
                let x = x as f32;
                let y = y as f32;
                let t = t as f32;
                let real_area = x * y;

                let transform = Matrix3x2f::new([
                    [x, 0.0], // Scale along the X axis by `x`
                    [0.0, y], // Scale along the Y axis by `y`
                    [t, -t],  // this value shouldn't affect area (translation component)
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
                assert!((area - real_area * real_area).abs() <= EPSILON);
            }
        }
    }
}

#[test]
fn path_geometry() {
    use direct2d::enums::{FigureBegin::*, FigureEnd::*};

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

    let path = PathGeometry::create(&factory)
        .unwrap()
        .fill_mode(FillMode::Winding)
        // Square with a triangle base
        .with_line_figure(
            Filled,
            Closed,
            &[
                (0.0, 0.0).into(),
                (1.0, 0.0).into(),
                (1.0, 1.0).into(),
                (0.5, 1.5).into(),
                (0.0, 1.0).into(),
            ],
        )
        // Add a triangle hat
        .with_line_figure(
            Filled,
            Closed,
            &[(0.0, 0.0).into(), (0.5, -0.5).into(), (1.0, 0.0).into()],
        )
        // Cut a hole in the middle
        .fill_mode(FillMode::Alternate)
        .with_line_figure(
            Filled,
            Closed,
            &[
                (0.25, 0.25).into(),
                (0.75, 0.25).into(),
                (0.75, 0.75).into(),
                (0.25, 0.75).into(),
            ],
        )
        .finish()
        .unwrap();

    let real_area = 1.25;
    let area = path.compute_area(None).unwrap();
    assert!((area - real_area).abs() <= EPSILON);
}
