use eframe::{epaint::{Pos2, Color32, Stroke, Vec2, FontId}, egui::{Painter, Rect}, emath::Align2};
use simulator::function::Function;

const STROKE_WIDTH: f32 = 5.0;
const STROKE_COLOR: Color32 = Color32::WHITE;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl RealPosition for Position {
    fn set_position(&mut self, position: Position) {
        *self = position
    }

    fn get_position(&self) -> Position {
        *self
    }
}

pub struct EditorInput {
    position: Position,
}

impl EditorInput {
    pub fn new(position: Position) -> Self {
        Self {
            position,
        }
    }
}

impl RealPosition for EditorInput {
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn get_position(&self) -> Position {
        self.position
    }
}

impl Draw for EditorInput {
    fn draw(&self, painter: &Painter, scaling: f32, area: Rect) {
        let shape_stroke = Stroke::new(STROKE_WIDTH, STROKE_COLOR);
        let real_position = Self::get_real_position(&self, scaling, area);

        painter.rect_stroke(Rect::from_center_size(real_position, Vec2::splat(scaling * 2.0)), 0.0, shape_stroke);
        painter.circle_stroke(real_position, (scaling * 2.0) / 3.0, shape_stroke);
        painter.circle_filled(real_position + Vec2::new(scaling, 0.0), STROKE_WIDTH, Color32::RED);
    }
}

pub struct EditorOutput {
    position: Position,
}

impl EditorOutput {
    pub fn new(position: Position) -> Self {
        Self {
            position,
        }
    }
}

impl RealPosition for EditorOutput {
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn get_position(&self) -> Position {
        self.position
    }
}

impl Draw for EditorOutput {
    fn draw(&self, painter: &Painter, scaling: f32, area: Rect) {
        let shape_stroke = Stroke::new(STROKE_WIDTH, STROKE_COLOR);
        let real_position = Self::get_real_position(&self, scaling, area);

        painter.circle_stroke(real_position, scaling, shape_stroke);
        painter.circle_stroke(real_position, (scaling * 2.0) / 3.0, shape_stroke);
        painter.circle_filled(real_position - Vec2::new(scaling, 0.0), STROKE_WIDTH, Color32::RED);
    }
}


pub struct EditorComponent {
    position: Position,
    function: Function,
}

impl EditorComponent {
    pub fn new(position: Position, function: Function) -> Self {
        Self {
            position,
            function,
        }
    }
}

impl RealPosition for EditorComponent {
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn get_position(&self) -> Position {
        self.position
    }
}

impl Draw for EditorComponent {
    fn draw(&self, painter: &Painter, scaling: f32, area: Rect) {
        let shape_stroke = Stroke::new(STROKE_WIDTH, STROKE_COLOR);
        let real_position = Self::get_real_position(&self, scaling, area);

        painter.rect_stroke(Rect::from_center_size(real_position, Vec2::splat(scaling * 2.0)), 0.0, shape_stroke);
        painter.text(real_position, Align2::CENTER_CENTER, "comp", FontId::monospace(12.0), Color32::WHITE);
    }
}

pub struct EditorLine {
    start: Position,
    end: Position,
}

impl EditorLine {
    pub fn new(start: Position, end: Position) -> Self {
        Self {
            start,
            end,
        }
    }
}

impl Draw for EditorLine {
    fn draw(&self, painter: &Painter, scaling: f32, area: Rect) {
        let real_start = self.start.get_real_position(scaling, area);
        let real_end = self.end.get_real_position(scaling, area);

        painter.line_segment([real_start, real_end], Stroke::new(STROKE_WIDTH, Color32::GREEN));
    }
}

pub trait RealPosition {
    fn set_position(&mut self, position: Position);
    fn get_position(&self) -> Position;

    fn get_real_position(&self, scaling: f32, area: Rect) -> Pos2 {
        Pos2::new(self.get_position().x as f32, self.get_position().y as f32) * scaling + area.left_top().to_vec2()
    }
}

pub trait Draw {
    fn draw(&self, painter: &Painter, scaling: f32, area: Rect);
}