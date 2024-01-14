use eframe::{epaint::{Rect, Pos2, Stroke, Color32, Vec2, FontId}, egui::{Painter, InputState}, emath::Align2};
use simulator::{Circuit, element::{Component, Output, Input}};

pub struct VisualCircuit {
    inputs: Vec<VisualInput>,
    outputs: Vec<VisualOutput>,
    components: Vec<VisualComponent>,
    dragging: Option<DragSelection>,
}

#[derive(Debug)]
struct DragSelection {
    element: ElementType,
    index: usize,
    start_position: Pos2,
}

#[derive(Debug)]
enum ElementType {
    Input,
    Output,
    Component,
}

pub trait EditorPosition {
    fn get_position(&self, editor_area: Rect) -> Pos2;
    fn set_position(&mut self, editor_area: Rect, position: Pos2);
}

pub trait ElementDraw<T> {
    fn draw(&self, index: usize, element: &T, painter: &Painter, editor_area: Rect);
}

pub trait ElementContainsPosition<T> {
    fn contains(&self, element: &T, editor_area: Rect, position: Pos2) -> bool;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct VisualInput {
    position: Pos2,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct VisualOutput {
    position: Pos2,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct VisualComponent {
    position: Pos2,
}

impl Default for VisualCircuit {
    fn default() -> Self {
        Self {
            inputs: Default::default(),
            outputs: Default::default(),
            components: Default::default(),
            dragging: None,
        }
    }
}

impl VisualCircuit {
    pub fn handle_interaction(&mut self, circuit: &Circuit, editor_area: Rect, input_state: &InputState) {

        if input_state.pointer.primary_clicked() {
            let mouse_down = input_state.pointer.press_origin().unwrap();
            let input = self.inputs.iter_mut()
                .zip(circuit.all_inputs().iter())
                .enumerate()
                .find(|(_, (visual, input))| visual.contains(input, editor_area, mouse_down));

            if let Some((i, (visual, _))) = input {
                self.dragging = Some(DragSelection::new(ElementType::Input, i, mouse_down));
                visual.set_position(editor_area, input_state.pointer.latest_pos().unwrap());
            } else {
                let output = self.outputs.iter_mut()
                    .zip(circuit.all_outputs().iter())
                    .enumerate()
                    .find(|(_, (visual, output))| visual.contains(output, editor_area, mouse_down));

                if let Some((i, (visual, _))) = output {
                    self.dragging = Some(DragSelection::new(ElementType::Output, i, mouse_down));
                    visual.set_position(editor_area, input_state.pointer.latest_pos().unwrap());
                }
            }

        } else if input_state.pointer.primary_released() {
            self.dragging = None;

        } else if let Some(dragging) = &self.dragging {

            let mouse_down = input_state.pointer.press_origin().unwrap();
            if dragging.start_position == mouse_down {
                match dragging.element {
                    ElementType::Input => self.inputs[dragging.index].set_position(editor_area, input_state.pointer.latest_pos().unwrap()),
                    ElementType::Output => self.outputs[dragging.index].set_position(editor_area, input_state.pointer.latest_pos().unwrap()),
                    ElementType::Component => self.components[dragging.index].set_position(editor_area, input_state.pointer.latest_pos().unwrap()),
                }

            } else {
                self.dragging = None;
            }
        }
    }

    pub fn draw(&mut self, circuit: &Circuit, painter: &Painter, editor_area: Rect) {
        self.inputs.iter()
            .zip(circuit.all_inputs().iter())
            .enumerate()
            .for_each(|(i, (visual, input))| visual.draw(i, input, painter, editor_area));

        self.outputs.iter()
            .zip(circuit.all_outputs().iter())
            .enumerate()
            .for_each(|(i, (visual, output))| visual.draw(i, output, painter, editor_area));

        self.components.iter()
            .zip(circuit.all_components().iter())
            .enumerate()
            .for_each(|(i, (visual, components))| visual.draw(i, components, painter, editor_area));
    }

    pub fn adjust_if_neccesary(&mut self, circuit: &Circuit) {
        if self.inputs.len() != circuit.all_inputs().len() {
            self.inputs.resize(circuit.all_inputs().len(), VisualInput::default());
        }

        if self.outputs.len() != circuit.all_outputs().len() {
            self.outputs.resize(circuit.all_outputs().len(), VisualOutput::default());
        }

        if self.components.len() != circuit.all_components().len() {
            self.components.resize(circuit.all_components().len(), VisualComponent::default());
        }
    }
}

impl DragSelection {
    fn new(element: ElementType, index: usize, start_position: Pos2) -> Self {
        Self {
            element,
            index,
            start_position,
        }
    }
}

impl EditorPosition for VisualInput {
    fn get_position(&self, editor_area: Rect) -> Pos2 {
        self.position + editor_area.left_top().to_vec2()
    }

    fn set_position(&mut self, editor_area: Rect, position: Pos2) {
        self.position = position - editor_area.left_top().to_vec2()
    }
}

impl ElementDraw<Input> for VisualInput {
    fn draw(&self, index: usize, _element: &Input, painter: &Painter, editor_area: Rect) {
        painter.circle_stroke(self.get_position(editor_area), 20.0, Stroke::new(5.0, Color32::DARK_GREEN));
        painter.text(self.get_position(editor_area), Align2::CENTER_CENTER, index, FontId::monospace(45.0), Color32::DARK_GREEN);
    }
}

impl ElementContainsPosition<Input> for VisualInput {
    fn contains(&self, _element: &Input, editor_area: Rect, position: Pos2) -> bool {
        let element_position = self.get_position(editor_area);
        let bounding_box = Rect::from_center_size(element_position, Vec2::splat(40.0));

        bounding_box.contains(position)
    }
}

impl EditorPosition for VisualOutput {
    fn get_position(&self, editor_area: Rect) -> Pos2 {
        self.position + editor_area.left_top().to_vec2()
    }

    fn set_position(&mut self, editor_area: Rect, position: Pos2) {
        self.position = position - editor_area.left_top().to_vec2()
    }
}

impl ElementDraw<Output> for VisualOutput {
    fn draw(&self, index: usize, _element: &Output, painter: &Painter, editor_area: Rect) {
        painter.circle_stroke(self.get_position(editor_area), 20.0, Stroke::new(5.0, Color32::DARK_RED));
        painter.text(self.get_position(editor_area), Align2::CENTER_CENTER, index, FontId::monospace(45.0), Color32::DARK_RED);
    }
}

impl ElementContainsPosition<Output> for VisualOutput {
    fn contains(&self, _element: &Output, editor_area: Rect, position: Pos2) -> bool {
        let element_position = self.get_position(editor_area);
        let bounding_box = Rect::from_center_size(element_position, Vec2::splat(40.0));

        bounding_box.contains(position)
    }
}

impl ElementDraw<Component> for VisualComponent {
    fn draw(&self, index: usize, _element: &Component, painter: &Painter, editor_area: Rect) {
        painter.rect(Rect::from_center_size(self.get_position(editor_area), Vec2::splat(20.0)), 2.0, Color32::LIGHT_GRAY, Stroke::new(2.0, Color32::DARK_BLUE));
        painter.text(self.get_position(editor_area), Align2::CENTER_CENTER, index, FontId::monospace(45.0), Color32::DARK_GRAY);
    }
}

impl EditorPosition for VisualComponent {
    fn get_position(&self, editor_area: Rect) -> Pos2 {
        self.position + editor_area.left_top().to_vec2()
    }

    fn set_position(&mut self, editor_area: Rect, position: Pos2) {
        self.position = position - editor_area.left_top().to_vec2()
    }
}