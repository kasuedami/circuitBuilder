use eframe::{epaint::{vec2, Color32, Pos2, Rect, Stroke, Vec2}, egui::{self, CentralPanel, InputState, Layout, Painter, Response, Sense}};
use egui::*;
use simulator::function::Function;

use self::elements::{EditorInput, EditorOutput, EditorComponent, EditorLine, Draw, Position};

mod elements;

pub struct Editor {
    offset: Pos2,
    gird_spacing: f32,
    area: Rect,
    circuit: EditorCircuit,
    pressed: bool, // TODO: turn into option to handle element selection
}

#[derive(Default)]
struct EditorCircuit {
    inputs: Vec<EditorInput>,
    outputs: Vec<EditorOutput>,
    components: Vec<EditorComponent>,
    lines: Vec<EditorLine>,
}

impl Editor {
    pub fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {

            if ui.button("Input").clicked() {
                self.circuit.inputs.push(EditorInput::new(pos2(20.0, 20.0)));
            }

            let (response, painter) =
                ui.allocate_painter(vec2(ui.available_width(), ui.available_height()), Sense::hover());

            let to_screen = emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect
            );

            let input_shapes: Vec<Shape> = self.circuit
                .inputs
                .iter_mut()
                .enumerate()
                .map(|(i, input)| {
                    let size = Vec2::splat(20.0);

                    let point_in_screen = to_screen.transform_pos(input.position);
                    let point_in_rect = Rect::from_center_size(point_in_screen, size);
                    let point_id = response.id.with(i);
                    let point_response = ui.interact(point_in_rect, point_id, Sense::drag());

                    input.position += point_response.drag_delta();
                    input.position = to_screen.from().clamp(input.position);

                    let point_in_screen = to_screen.transform_pos(input.position);
                    let stroke = ui.style().interact(&point_response).fg_stroke;

                    Shape::circle_stroke(point_in_screen, 20.0, stroke)
                })
                .collect();

            let positions_in_screen: Vec<Pos2> = self
                .circuit
                .inputs
                .iter()
                .map(|input| to_screen * input.position)
                .collect();

            input_shapes.iter()
                .for_each(|(shape)| {
                    painter.add(shape.clone());
                });
        });
    }

    fn handle_inputs(&mut self, input: &InputState) {
        if let Some(last_position) = input.pointer.latest_pos() {
            if !self.area.contains(last_position) {
                return;
            }

            if input.pointer.primary_pressed() {
                println!("pressed");

                // TODO: check not on element
                self.pressed = true;
            }

            if input.pointer.primary_released() {
                println!("released");
                self.pressed = false;
            }

            if self.pressed {
                self.offset += input.pointer.delta();
                dbg!("{}", self.offset);
            }
        }
    }

    fn grid(&self, painter: &Painter) {
        let editor_v_start = self.area.top();
        let editor_v_end = self.area.bottom();
        let editor_h_start = self.area.left();
        let editor_h_end = self.area.right();

        let editor_height_range = editor_v_start..=editor_v_end;
        let editor_width_range = editor_h_start..=editor_h_end;

        let editor_grid_stroke = Stroke::new(2.0, Color32::DARK_GRAY);

        let mut h_line_height = editor_v_start;
        while h_line_height < editor_v_end {
            painter.hline(editor_width_range.clone(), h_line_height, editor_grid_stroke);
            h_line_height += self.gird_spacing;
        }

        let mut v_line_height = editor_h_start;
        while v_line_height < editor_h_end {
            painter.vline(v_line_height, editor_height_range.clone(), editor_grid_stroke);
            v_line_height += self.gird_spacing;
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            offset: Pos2::ZERO,
            gird_spacing: 20.0,
            area: Rect::ZERO,
            circuit: Default::default(),
            pressed: false,
        }
    }
}

impl EditorCircuit {
    fn draw(&self, painter: &Painter, scaling: f32, area: Rect) {
        self.outputs.iter()
            .for_each(|output| output.draw(painter, scaling, area));

        self.components.iter()
            .for_each(|component| component.draw(painter, scaling, area));
    }
}