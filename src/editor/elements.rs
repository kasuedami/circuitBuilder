use eframe::{epaint::{Pos2, Color32, Stroke, Vec2}, egui::{Painter, Rect}};

pub trait Position {
    fn set_position(&mut self, position: Pos2);
    fn get_position(&self) -> Pos2;
}

pub trait Draw {
    fn draw(&self, painter: &Painter, area: Rect);
}

pub struct EditorComponent {
    position: Pos2,
}

pub struct EditorLine {
    start: Pos2,
    end: Pos2,
}


pub struct EditorInput {
    position: Pos2,
}

impl EditorInput {
    pub fn new(position: Pos2) -> Self {
        Self { position }
    }

    const STROKE_COLOR: Color32 = Color32::WHITE;
    const STROKE_WIDTH: f32 = 5.0;
    const SHAPE_SIZE: f32 = 40.0;
}

impl Draw for EditorInput {
    fn draw(&self, painter: &Painter, _area: Rect) {
        let shape_stroke = Stroke::new(Self::STROKE_WIDTH, Self::STROKE_COLOR);

        painter.rect_stroke(Rect::from_center_size(self.position, Vec2::splat(Self::SHAPE_SIZE)), 0.0, shape_stroke);
        painter.circle_stroke(self.position, Self::SHAPE_SIZE / 3.0, shape_stroke);
        painter.circle_filled(self.position + Vec2::new(Self::SHAPE_SIZE / 2.0, 0.0), Self::STROKE_WIDTH, Color32::RED);
    }
}

pub struct EditorOutput {
    position: Pos2,
}

impl EditorOutput {
    pub fn new(position: Pos2) -> Self {
        Self { position }
    }

    const STROKE_COLOR: Color32 = Color32::WHITE;
    const STROKE_WIDTH: f32 = 5.0;
    const SHAPE_SIZE: f32 = 40.0;
}

impl Draw for EditorOutput {
    fn draw(&self, painter: &Painter, _area: Rect) {
        let shape_stroke = Stroke::new(Self::STROKE_WIDTH, Self::STROKE_COLOR);

        painter.circle_stroke(self.position, Self::SHAPE_SIZE / 2.0, shape_stroke);
        painter.circle_stroke(self.position, Self::SHAPE_SIZE / 3.0, shape_stroke);
        painter.circle_filled(self.position - Vec2::new(Self::SHAPE_SIZE / 2.0, 0.0), Self::STROKE_WIDTH, Color32::RED);
    }
}

impl EditorComponent {
    pub fn new(position: Pos2) -> Self {
        Self { position }
    }
}

impl EditorLine {
    pub fn new(start: Pos2, end: Pos2) -> Self {
        Self { start, end }
    }
}