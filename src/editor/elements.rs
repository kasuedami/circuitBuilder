use eframe::{
    egui::Rect,
    emath::RectTransform,
    epaint::{vec2, Color32, Pos2, Shape, Stroke, Vec2},
};
use simulator::function::Function;

pub const CONNECTION_RADIUS: f32 = 5.0;

pub trait EditorShape {
    type DraggedInfo;

    fn get_shape(&self, transform: RectTransform, dragged_info: Self::DraggedInfo) -> Shape;
}

#[derive(Debug)]
pub struct EditorInput {
    pub position: Pos2,
}

impl EditorInput {
    pub fn new(position: Pos2) -> Self {
        Self { position }
    }
}

impl EditorShape for EditorInput {
    type DraggedInfo = bool;

    fn get_shape(&self, transform: RectTransform, dragged_info: Self::DraggedInfo) -> Shape {
        let transformed_position = transform.transform_pos(self.position);

        let border_stroke = if dragged_info {
            Stroke::new(4.0, Color32::WHITE)
        } else {
            Stroke::new(3.0, Color32::WHITE)
        };

        let border = Shape::rect_stroke(
            Rect::from_center_size(transformed_position, Vec2::splat(40.0)),
            0.0,
            border_stroke,
        );
        let inner =
            Shape::circle_stroke(transformed_position, 16.0, Stroke::new(3.0, Color32::WHITE));
        let connector = Shape::circle_filled(
            transformed_position + vec2(20.0, 0.0),
            CONNECTION_RADIUS,
            Color32::RED,
        );

        Shape::Vec(vec![border, inner, connector])
    }
}

#[derive(Debug)]
pub struct EditorOutput {
    pub position: Pos2,
}

impl EditorOutput {
    pub fn new(position: Pos2) -> Self {
        Self { position }
    }
}

impl EditorShape for EditorOutput {
    type DraggedInfo = bool;

    fn get_shape(&self, transform: RectTransform, dragged_info: Self::DraggedInfo) -> Shape {
        let transformed_position = transform.transform_pos(self.position);

        let border_stroke = if dragged_info {
            Stroke::new(4.0, Color32::WHITE)
        } else {
            Stroke::new(3.0, Color32::WHITE)
        };

        let border = Shape::circle_stroke(transformed_position, 20.0, border_stroke);
        let inner =
            Shape::circle_stroke(transformed_position, 16.0, Stroke::new(3.0, Color32::WHITE));
        let connector = Shape::circle_filled(
            transformed_position + vec2(-20.0, 0.0),
            CONNECTION_RADIUS,
            Color32::RED,
        );

        Shape::Vec(vec![border, inner, connector])
    }
}

#[derive(Debug)]
pub struct EditorComponent {
    pub position: Pos2,
    pub function: Function,
}

impl EditorComponent {
    pub fn new(position: Pos2, function: Function) -> Self {
        Self { position, function }
    }
}

impl EditorShape for EditorComponent {
    type DraggedInfo = bool;

    fn get_shape(&self, transform: RectTransform, dragged_info: Self::DraggedInfo) -> Shape {
        let transformed_position = transform.transform_pos(self.position);

        let border_stroke = if dragged_info {
            Stroke::new(4.0, Color32::WHITE)
        } else {
            Stroke::new(3.0, Color32::WHITE)
        };

        let border = Shape::rect_stroke(
            Rect::from_center_size(transformed_position, Vec2::splat(60.0)),
            0.0,
            border_stroke,
        );
        let in_connector0 = Shape::circle_filled(
            transformed_position + vec2(-30.0, -15.0),
            CONNECTION_RADIUS,
            Color32::RED,
        );
        let in_connector1 = Shape::circle_filled(
            transformed_position + vec2(-30.0, 15.0),
            CONNECTION_RADIUS,
            Color32::RED,
        );
        let out_connector0 = Shape::circle_filled(
            transformed_position + vec2(30.0, 0.0),
            CONNECTION_RADIUS,
            Color32::RED,
        );

        Shape::Vec(vec![border, in_connector0, in_connector1, out_connector0])
    }
}

#[derive(Debug)]
pub struct EditorLine {
    pub start: Pos2,
    pub end: Pos2,
}

impl EditorLine {
    pub fn from_single_pos(pos: Pos2) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }
}

impl EditorShape for EditorLine {
    type DraggedInfo = (bool, bool);

    fn get_shape(&self, transform: RectTransform, dragged_info: Self::DraggedInfo) -> Shape {
        let (start_dragged, end_dragged) = dragged_info;
        let real_start = transform.transform_pos(self.start);
        let real_end = transform.transform_pos(self.end);

        let start_shape = Shape::circle_filled(real_start, CONNECTION_RADIUS, Color32::GREEN);
        let end_shape = Shape::circle_filled(real_end, CONNECTION_RADIUS, Color32::GREEN);
        let line_shape = Shape::line(vec![real_start, real_end], Stroke::new(5.0, Color32::GREEN));

        if start_dragged {
            let hover_shape =
                Shape::circle_stroke(real_start, 7.0, Stroke::new(1.0, Color32::GREEN));

            Shape::Vec(vec![start_shape, end_shape, line_shape, hover_shape])
        } else if end_dragged {
            let hover_shape = Shape::circle_stroke(real_end, 7.0, Stroke::new(1.0, Color32::GREEN));

            Shape::Vec(vec![start_shape, end_shape, line_shape, hover_shape])
        } else {
            Shape::Vec(vec![start_shape, end_shape, line_shape])
        }
    }
}
