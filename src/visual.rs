use eframe::{epaint::{Rect, Pos2, Stroke, Color32, Vec2}, egui::Painter};
use simulator::{Circuit, element::{Component, Output, Input}};

pub struct VisualCircuit {
    inputs: Vec<VisualInput>,
    outputs: Vec<VisualOutput>,
    components: Vec<VisualComponent>,
}

pub trait EditorPosition {
    fn position(&self, editor_area: Rect) -> Pos2;
}

pub trait ElementDraw<T> {
    fn draw(&self, element: &T, painter: &Painter, editor_area: Rect);
}

#[derive(Default, Clone, Copy)]
pub struct VisualInput {
    position: Pos2,
}

#[derive(Default, Clone, Copy)]
pub struct VisualOutput {
    position: Pos2,
}

#[derive(Default, Clone, Copy)]
pub struct VisualComponent {
    position: Pos2,
}

impl Default for VisualCircuit {
    fn default() -> Self {
        Self {
            inputs: Default::default(),
            outputs: Default::default(),
            components: Default::default()
        }
    }
}

impl VisualCircuit {
    pub fn draw(&mut self, circuit: &Circuit, painter: &Painter, editor_area: Rect) {

        if self.inputs.len() != circuit.all_inputs().len() {
            self.inputs.resize(circuit.all_inputs().len(), VisualInput::default());
        }

        if self.outputs.len() != circuit.all_outputs().len() {
            self.outputs.resize(circuit.all_outputs().len(), VisualOutput::default());
        }

        if self.components.len() != circuit.all_components().len() {
            self.components.resize(circuit.all_components().len(), VisualComponent::default());
        }

        self.inputs.iter()
            .zip(circuit.all_inputs().iter())
            .for_each(|(visual, input)| visual.draw(input, painter, editor_area));

        self.outputs.iter()
            .zip(circuit.all_outputs().iter())
            .for_each(|(visual, output)| visual.draw(output, painter, editor_area));

        self.components.iter()
            .zip(circuit.all_components().iter())
            .for_each(|(visual, components)| visual.draw(components, painter, editor_area));
    }
}

impl ElementDraw<Input> for VisualInput {
    fn draw(&self, element: &Input, painter: &Painter, editor_area: Rect) {
        painter.circle_stroke(self.position(editor_area), 20.0, Stroke::new(5.0, Color32::DARK_GREEN));
    }
}

impl EditorPosition for VisualInput {
    fn position(&self, editor_area: Rect) -> Pos2 {
        self.position + editor_area.left_top().to_vec2()
    }
}

impl ElementDraw<Output> for VisualOutput {
    fn draw(&self, element: &Output, painter: &Painter, editor_area: Rect) {
        painter.circle_stroke(self.position(editor_area), 20.0, Stroke::new(5.0, Color32::DARK_RED));
    }
}

impl EditorPosition for VisualOutput {
    fn position(&self, editor_area: Rect) -> Pos2 {
        self.position + editor_area.left_top().to_vec2()
    }
}

impl ElementDraw<Component> for VisualComponent {
    fn draw(&self, element: &Component, painter: &Painter, editor_area: Rect) {
        painter.rect(Rect::from_center_size(self.position(editor_area), Vec2::splat(20.0)), 2.0, Color32::LIGHT_GRAY, Stroke::new(2.0, Color32::DARK_BLUE));
    }
}

impl EditorPosition for VisualComponent {
    fn position(&self, editor_area: Rect) -> Pos2 {
        self.position + editor_area.left_top().to_vec2()
    }
}