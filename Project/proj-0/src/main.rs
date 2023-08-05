mod utils;
mod grid;

use nannou::{prelude::*, color};

fn main() {
    nannou::app(model)
        .update(update) // rather than `.event(event)`, now we only subscribe to updates
        .simple_window(view)
        .run();
}

struct Model {
    pixel: f32,
    size: u32,
    pos: Vec<f32>,
}

//                ----- Access to the `App` passed as an input to the function.
//               /
//              v
fn model(_app: &App, size: u32) -> Model {
    Model {
        pixel: 25.0,
        size: size,
        pos: Vec::new()
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background().color(GRAY);
    draw.rect()
        .x_y(-200.0, 0.0)
        .w_h(_model.pixel, _model.pixel)
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}