# direct2d-rs [![Crates.io](https://img.shields.io/crates/v/direct2d.svg)](https://crates.io/crates/direct2d)

Safe abstractions for drawing on Windows using Direct2D

To use it, add this to your Cargo.toml:
```toml
[dependencies]
direct2d = "0.1"
```

## Example

```rs
extern crate direct2d;

use direct2d::{DeviceContext, RenderTarget};
use direct2d::brush::SolidColorBrush;
use direct2d::image::Bitmap;

fn draw(context: &mut DeviceContext, target: &Bitmap) {
    let brush = SolidColorBrush::create(&context)
        .with_color(0xFF_7F_7F)
        .build().unwrap();

    context.begin_draw();
    context.set_target(target);
    context.clear(0xFF_FF_FF);
    
    context.draw_line((10.0, 10.0), (20.0, 20.0), &brush, 2.0, None);
    context.draw_line((10.0, 20.0), (20.0, 10.0), &brush, 2.0, None);

    match context.end_draw() {
        Ok(_) => {/* cool */},
        Err(_) => panic!("Uh oh, rendering failed!"),
    }
}
```
