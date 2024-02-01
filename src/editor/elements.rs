use eframe::{epaint::{vec2, Color32, FontId, Pos2, Shape, Stroke, Vec2}, egui::{Painter, Rect}, emath::{Align2, RectTransform}};
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

pub struct EditorInput {
    pub position: Pos2,
}

impl EditorInput {
    pub fn new(position: Pos2) -> Self {
        Self {
            position,
        }
    }

    pub fn shape(&self, transform: RectTransform, dragged: bool) -> Shape {
        let transformed_position = transform.transform_pos(self.position);
        
        let border_stroke = if dragged {
            Stroke::new(4.0, Color32::WHITE)
        } else {
            Stroke::new(3.0, Color32::WHITE)
        };

        let border = Shape::rect_stroke(Rect::from_center_size(transformed_position, Vec2::splat(40.0)), 0.0, border_stroke);
        let inner = Shape::circle_stroke(transformed_position, 16.0, Stroke::new(3.0, Color32::WHITE));
        let connector = Shape::circle_filled(transformed_position + vec2(20.0, 0.0), 5.0, Color32::RED);

        Shape::Vec(vec![border, inner, connector])
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

#[derive(Debug)]
pub struct EditorLine {
    pub start: Pos2,
    pub end: Pos2,
}

impl EditorLine {
    pub fn new(start: Pos2, end: Pos2) -> Self {
        Self {
            start,
            end,
        }
    }
    
    pub fn from_single_pos(pos: Pos2) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }
    
    pub fn shape(&self, transform: RectTransform, start_dragged: bool, end_dragged: bool) -> Shape {
        let real_start = transform.transform_pos(self.start);
        let real_end = transform.transform_pos(self.end);

        let start_shape = Shape::circle_filled(real_start, 5.0, Color32::GREEN);
        let end_shape = Shape::circle_filled(real_end, 5.0, Color32::GREEN);
        let line_shape = Shape::line(vec![real_start, real_end], Stroke::new(5.0, Color32::GREEN));
        
        if start_dragged {
            let hover_shape = Shape::circle_stroke(real_start, 7.0, Stroke::new(1.0, Color32::GREEN));

            Shape::Vec(vec![start_shape, end_shape, line_shape, hover_shape])
        } else if end_dragged {
            let hover_shape = Shape::circle_stroke(real_end, 7.0, Stroke::new(1.0, Color32::GREEN));

            Shape::Vec(vec![start_shape, end_shape, line_shape, hover_shape])
        } else {
            Shape::Vec(vec![start_shape, end_shape, line_shape])
        }
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