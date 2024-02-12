use std::f32::consts::PI;

use eframe::{
    egui::Rect,
    emath::RectTransform,
    epaint::{
        pos2, vec2, Color32, FontFamily, FontId, Fonts, Pos2, Shape, Stroke, TextShape, Vec2,
    },
};
use simulator::function::Function;

pub const CONNECTION_RADIUS: f32 = 5.0;

pub trait EditorShape {
    type AdditionalInfo;

    fn get_shape(&self, transform: &RectTransform, additional_info: Self::AdditionalInfo) -> Shape;
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
    type AdditionalInfo = bool;

    fn get_shape(&self, transform: &RectTransform, additional_info: Self::AdditionalInfo) -> Shape {
        let transformed_position = transform.transform_pos(self.position);

        let border_stroke = if additional_info {
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
    type AdditionalInfo = bool;

    fn get_shape(&self, transform: &RectTransform, additional_info: Self::AdditionalInfo) -> Shape {
        let transformed_position = transform.transform_pos(self.position);

        let border_stroke = if additional_info {
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

impl EditorComponent {
    pub fn size(&self) -> Vec2 {
        vec2(
            40.0,
            match self.function {
                Function::And => 50.0,
                Function::Or => 50.0,
                Function::Xor => 50.0,
                Function::Not => 40.0,
                Function::Nand => 50.0,
                Function::Nor => 50.0,
                Function::Xnor => 50.0,
                Function::Circuit(_) => todo!(),
                Function::FlipFlopRS => 50.0,
                Function::FlipFlopJK => 70.0,
                Function::FlipFlopD => 50.0,
                Function::FlipFlopT => 50.0,
            },
        )
    }

    pub fn input_connector_positions(&self) -> Vec<Pos2> {
        let size = self.size();
        let input_count = self.function.input_value_count();
        let input_spacing = size.y / input_count as f32;

        (0..input_count)
            .map(|i| {
                pos2(
                    self.position.x - (size.x / 2.0),
                    self.position.y - (size.y / 2.0) + (input_spacing * (i as f32 + 0.5)),
                )
            })
            .collect()
    }

    pub fn output_connector_positions(&self) -> Vec<Pos2> {
        let size = self.size();
        let output_count = self.function.output_value_count();
        let output_spacing = size.y / output_count as f32;

        (0..output_count)
            .map(|i| {
                pos2(
                    self.position.x + (size.x / 2.0),
                    self.position.y - (size.y / 2.0) + (output_spacing * (i as f32 + 0.5)),
                )
            })
            .collect()
    }
}

impl EditorShape for EditorComponent {
    type AdditionalInfo = (bool, Fonts);

    fn get_shape(&self, transform: &RectTransform, additional_info: Self::AdditionalInfo) -> Shape {
        let mut shapes = vec![];
        let transformed_position = transform.transform_pos(self.position);

        let border_stroke = if additional_info.0 {
            Stroke::new(4.0, Color32::WHITE)
        } else {
            Stroke::new(3.0, Color32::WHITE)
        };

        // Border
        shapes.push(Shape::rect_stroke(
            Rect::from_center_size(transformed_position, self.size()),
            0.0,
            border_stroke,
        ));

        // Text
        let text = self.function.to_string().replace("FlipFlop", "FF-");
        let galley = additional_info.1.layout_no_wrap(
            text.to_string(),
            FontId::new(12.0, FontFamily::Monospace),
            Color32::WHITE,
        );
        let rotated_text_poistion =
            transformed_position + (vec2(-galley.size().y, galley.size().x) / 2.0);

        shapes.push(
            TextShape::new(rotated_text_poistion, galley, Color32::WHITE)
                .with_angle(-PI / 2.0)
                .into(),
        );

        // Inputs
        self.input_connector_positions()
            .iter()
            .for_each(|&input_connector_position| {
                shapes.push(Shape::circle_filled(
                    transform.transform_pos(input_connector_position),
                    CONNECTION_RADIUS,
                    Color32::RED,
                ));
            });

        // Outputs
        self.output_connector_positions()
            .iter()
            .for_each(|&output_connector_position| {
                shapes.push(Shape::circle_filled(
                    transform.transform_pos(output_connector_position),
                    CONNECTION_RADIUS,
                    Color32::RED,
                ));
            });

        Shape::Vec(shapes)
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
    type AdditionalInfo = (bool, bool);

    fn get_shape(&self, transform: &RectTransform, additional_info: Self::AdditionalInfo) -> Shape {
        let (start_dragged, end_dragged) = additional_info;
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
