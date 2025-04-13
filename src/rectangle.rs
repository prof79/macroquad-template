// rectangle.rs
// Custom Rectangle

use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CustomRectangle {
    pub point1: Vec2,
    pub point2: Vec2,
    pub color: Color,
    pub thickness: f32,
}

impl CustomRectangle {

    pub fn new(
                x: f32,
                y: f32,
                width: f32,
                height: f32,
                thickness: f32,
                color: Color,
            ) -> Self {

        CustomRectangle {
            point1: (x, y).into(),
            point2: (x + width, y + height).into(),
            thickness: thickness,
            color: color,
        }
    }

    pub fn width(&self) -> f32 {
        self.point2.x - self.point1.x
    }

    pub fn height(&self) -> f32 {
        self.point2.y - self.point1.y
    }

    pub fn draw(&self) {
        let point_a: Vec2 = (self.point2.x, self.point1.y).into();
        let point_b: Vec2 = (self.point1.x, self.point2.y).into();

        draw_line(
            self.point1.x,
            self.point1.y,
            point_a.x,
            point_a.y,
            self.thickness,
            self.color,
        );
        draw_line(
            point_a.x,
            point_a.y,
            self.point2.x,
            self.point2.y,
            self.thickness,
            self.color,
        );
        draw_line(
            self.point2.x,
            self.point2.y,
            point_b.x,
            point_b.y,
            self.thickness,
            self.color,
        );
        draw_line(
            point_b.x,
            point_b.y,
            self.point1.x,
            self.point1.y,
            self.thickness,
            self.color,
        )
    }
}
