use eframe::{epaint::{Rect, Stroke, Color32, Vec2}, egui::{Painter, self, CentralPanel, Layout, InputState}};

use self::elements::{EditorInput, EditorOutput, EditorComponent, EditorLine};

mod elements;

pub struct Editor {
    gird_spacing: f32,
    area: Rect,
    circuit: EditorCircuit,
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

            ctx.input(|input| self.circuit.update(self.area, input));

            self.grid(&painter);
            self.circuit.draw(&painter, self.area);

            ui.with_layout(Layout::left_to_right(egui::Align::Max), |ui| {

                if ui.button("Input").clicked() {
                    self.circuit.inputs.push(EditorInput::new(self.area.center()));
                }

                if ui.button("Output").clicked() {
                    self.circuit.outputs.push(EditorOutput::new(self.area.center()));
                }

                if ui.button("Component").clicked() {
                    self.circuit.components.push(EditorComponent::new(self.area.center()));
                }

                if ui.button("Line").clicked() {
                    self.circuit.lines.push(EditorLine::new(self.area.center() - Vec2::new(50.0, 0.0), self.area.center() + Vec2::new(50.0, 0.0)));
                }
            });
        });
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
            gird_spacing: 20.0,
            area: Rect::ZERO,
            circuit: Default::default(),
        }
    }
}

impl EditorCircuit {
    fn update(&mut self, area: Rect, input: &InputState) {
        if let Some(last_position) = input.pointer.latest_pos() {
            if !area.contains(last_position) {
                return;
            }
        }
    }

    fn draw(&self, painter: &Painter, area: Rect) {
        self.inputs.iter()
            .for_each(|input| input.draw(painter, area));
    }
}