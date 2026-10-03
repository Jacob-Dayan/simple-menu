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
#[derive(Clone, Debug)]
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

    /// Spawns a dot randomly inside the given boundary box.
    /// Because doing math to place dots by hand was getting old.
    pub fn spawn_in_bounds(bounds: Rect) -> Self {
        let radius = 5.0;
        let x = macroquad::rand::gen_range(bounds.left() + radius, bounds.right() - radius);
        let y = macroquad::rand::gen_range(bounds.top() + radius, bounds.bottom() - radius);
        Self {
            pos: vec2(x, y),
            radius,
            ..Default::default()
        }
    }

    /// Updates the dot's position and bounces off the boundary walls.
    /// If you find a bug here, no you didn't.
    pub fn update(&mut self, dt: f32, bounds: Rect) {
        self.pos += self.vel * dt;

        // each of these variables represents the boundaries of the area the dot can move in
        let left = bounds.left() + self.radius;
        let right = bounds.right() - self.radius;
        let top = bounds.top() + self.radius;
        let bottom = bounds.bottom() - self.radius;

        // below is simple collision detection with the area boundaries
        if self.pos.x < left {
            self.pos.x = left;
            self.vel.x *= -1.0;
        } else if self.pos.x > right {
            self.pos.x = right;
            self.vel.x *= -1.0;
        }

        if self.pos.y < top {
            self.pos.y = top;
            self.vel.y *= -1.0;
        } else if self.pos.y > bottom {
            self.pos.y = bottom;
            self.vel.y *= -1.0;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
        draw_circle_lines(self.pos.x, self.pos.y, self.radius, 2.0, BLACK);
    }
}

/// Helper to spawn a dot and push it directly into a collection
pub fn spawn_dot_in_box(dots: &mut Vec<CuteDot>, bounds: Rect) {
    dots.push(CuteDot::spawn_in_bounds(bounds));
}
