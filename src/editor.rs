use eframe::{epaint::{Rect, Stroke, Color32, Vec2, Pos2}, egui::{Painter, self, CentralPanel, Layout, InputState}};
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
            let painter = ui.painter_at(self.area);

            ctx.input(|input| self.handle_inputs(input));

            self.grid(&painter);
            self.circuit.draw(&painter, self.gird_spacing, self.area);

            ui.with_layout(Layout::left_to_right(egui::Align::Max), |ui| {

                if ui.button("Input").clicked() {
                    self.circuit.inputs.push(EditorInput::new(Position::new(2, 2)));
                }

                if ui.button("Output").clicked() {
                    self.circuit.outputs.push(EditorOutput::new(Position::new(5, 5)));
                }

                if ui.button("Component").clicked() {
                    self.circuit.components.push(EditorComponent::new(Position::new(10, 10), Function::And));
                }

                if ui.button("Line").clicked() {
                    self.circuit.lines.push(EditorLine::new(Position::new(1, 15), Position::new(15, 15)));
                }
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
        self.lines.iter()
            .for_each(|line| line.draw(painter, scaling, area));

        self.inputs.iter()
            .for_each(|input| input.draw(painter, scaling, area));

        self.outputs.iter()
            .for_each(|output| output.draw(painter, scaling, area));

        self.components.iter()
            .for_each(|component| component.draw(painter, scaling, area));
    }
}