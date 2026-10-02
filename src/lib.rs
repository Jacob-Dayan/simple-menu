pub use macroquad::prelude::*;

/// I mean if you needed a documentation for that enum,
/// the problem is not the developer didn't document it;
/// it's you.
pub enum CurrentState {
    Game,
    MainMenu,
    Settings,
}

#[derive(Clone, Copy, Debug)]
pub enum BgColor {
    Red,
    Green,
    Blue,
    Pink,
    Purple,
}

/// Some boilerplate for changing color A -> B
impl BgColor {
    pub fn next(self) -> BgColor {
        match self {
            BgColor::Red => BgColor::Green,
            BgColor::Green => BgColor::Blue,
            BgColor::Blue => BgColor::Pink,
            BgColor::Pink => BgColor::Purple,
            BgColor::Purple => BgColor::Red,
        }
    }
}

/// Just a more idiomatic way to convert one data type to another
/// In that case: BgColor -> Color
impl From<BgColor> for Color {
    fn from(value: BgColor) -> Self {
        match value {
            BgColor::Red => RED,
            BgColor::Green => GREEN,
            BgColor::Blue => BLUE,
            BgColor::Pink => PINK,
            BgColor::Purple => PURPLE,
        }
    }
}

/// simple struct to represent a cute dot
pub struct CuteDot {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: Color,
}
impl Default for CuteDot {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            vel: vec2(50.0, 50.0),
            radius: 5.0,
            color: YELLOW,
        }
    }
}

impl CuteDot {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
        draw_circle_lines(self.pos.x, self.pos.y, self.radius, 2.0, BLACK);
    }
}
