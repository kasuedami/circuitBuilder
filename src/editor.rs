use eframe::{egui::{self, CentralPanel, Sense}, emath::RectTransform, epaint::{vec2, Pos2, Rect, Vec2}};
use egui::*;

use self::elements::{EditorInput, EditorOutput, EditorComponent, EditorLine};

mod elements;

pub struct Editor {
    offset: Pos2,
    gird_spacing: f32,
    area: Rect,
    circuit: EditorCircuit,
    pressed: bool, // TODO: turn into option to handle element selection
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
            
            let connected_lines_start = self.circuit.input_connected_lines_start();
            let connected_lines_end = self.circuit.input_connected_lines_end();
                    
            let input_shapes: Vec<Shape> = self.circuit
                .inputs
                .iter_mut()
                .enumerate()
                .map(|(i, input)| {
                    let size = Vec2::splat(40.0);

                    let point_in_screen = to_screen.transform_pos(input.position);
                    let point_in_rect = Rect::from_center_size(point_in_screen, size);
                    let point_id = response.id.with("input".to_owned() + &i.to_string());
                    let point_response = ui.interact(point_in_rect, point_id, Sense::drag());
                    
                    input.position += point_response.drag_delta();
                    input.position = to_screen.from().clamp(input.position);
                    
                    let connector_position = point_in_screen + Vec2::new(20.0, 0.0);
                    let connector_rect = Rect::from_center_size(connector_position, Vec2::splat(10.0));
                    let connector_id = response.id.with("input connector".to_owned() + &i.to_string());
                    let connector_response = ui.interact(connector_rect, connector_id, Sense::click());
                    
                    if connector_response.clicked() {
                        self.circuit.lines.push(EditorLine::from_single_pos(input.position + vec2(20.0, 0.0)));
                    }

                    input.shape(to_screen, point_response.dragged())
                })
                .collect();

            self.circuit.input_reconnect_lines_start(connected_lines_start);
            self.circuit.input_reconnect_lines_end(connected_lines_end);
            
            let line_shapes: Vec<Shape> = self.circuit
                .lines
                .iter_mut()
                .enumerate()
                .map(|(i, line)| {
                    let size = Vec2::splat(10.0);

                    let start_in_screen = to_screen.transform_pos(line.start);
                    let start_rect = Rect::from_center_size(start_in_screen, size);
                    let start_id = response.id.with("line start".to_string() + &i.to_string());
                    let start_response = ui.interact(start_rect, start_id, Sense::drag());
                    
                    line.start += start_response.drag_delta();
                    line.start = to_screen.from().clamp(line.start);                    

                    let end_in_screen = to_screen.transform_pos(line.end);
                    let end_rect = Rect::from_center_size(end_in_screen, size);
                    let end_id = response.id.with("line end".to_string() + &i.to_string());
                    let end_response = ui.interact(end_rect, end_id, Sense::drag());
                    
                    line.end += end_response.drag_delta();
                    line.end = to_screen.from().clamp(line.end);                    
                    
                    line.shape(to_screen, start_response.dragged(), end_response.dragged())
                })
                .collect();

            input_shapes.iter().chain(line_shapes.iter())
                .for_each(|shape| {
                    painter.add(shape.clone());
                });
        });
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

#[derive(Default)]
struct EditorCircuit {
    inputs: Vec<EditorInput>,
    outputs: Vec<EditorOutput>,
    components: Vec<EditorComponent>,
    lines: Vec<EditorLine>,
}

impl EditorCircuit {
    fn input_connected_lines_start(&self) -> Vec<Vec<usize>> {
        self.inputs.iter().enumerate().map(|(input_index, _)| {

            let connector_position = self.inputs[input_index].position + Vec2::new(20.0, 0.0);
            let connector_area = Rect::from_center_size(connector_position, Vec2::splat(10.0));

            self.lines
                .iter()
                .enumerate()
                .filter_map(|(i, line)| {
                    if connector_area.contains(line.start) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
    }

    fn input_connected_lines_end(&self) -> Vec<Vec<usize>> {
        self.inputs.iter().enumerate().map(|(input_index, _)| {

            let connector_position = self.inputs[input_index].position + Vec2::new(20.0, 0.0);
            let connector_area = Rect::from_center_size(connector_position, Vec2::splat(10.0));

            self.lines
                .iter()
                .enumerate()
                .filter_map(|(i, line)| {
                    if connector_area.contains(line.end) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
    }
    
    fn input_reconnect_lines_start(&mut self, connection_list: Vec<Vec<usize>>) {
        connection_list.iter()
            .enumerate()
            .for_each(|(input_index, line_indices)| {
                let connector_position = self.inputs[input_index].position + Vec2::new(20.0, 0.0);

                line_indices.iter()
                    .for_each(|&line_index| {
                        self.lines[line_index].start = connector_position;
                    })

            })
    }

    fn input_reconnect_lines_end(&mut self, connection_list: Vec<Vec<usize>>) {
        connection_list.iter()
            .enumerate()
            .for_each(|(input_index, line_indices)| {
                let connector_position = self.inputs[input_index].position + Vec2::new(20.0, 0.0);

                line_indices.iter()
                    .for_each(|&line_index| {
                        self.lines[line_index].end = connector_position;
                    })

            })
    }
}