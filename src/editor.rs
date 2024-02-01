use eframe::{egui::{self, CentralPanel, Sense}, epaint::{vec2, Pos2, Rect, Vec2}};
use egui::*;
use simulator::function::Function;

use self::elements::{EditorComponent, EditorInput, EditorLine, EditorOutput, EditorShape};

mod elements;

pub struct Editor {
    circuit: EditorCircuit,
}

impl Editor {
    pub fn update(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            
            let button_responses = ui.horizontal_top(|ui| {
                let input_response = ui.button("Input");
                let component_response = ui.button("Component");
                
                (input_response, component_response)
            });
            
            let (input_response, component_response) = button_responses.inner;

            let (response, painter) =
                ui.allocate_painter(vec2(ui.available_width(), ui.available_height()), Sense::hover());

            let to_screen = emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect
            );
            
            if input_response.clicked() {
                self.circuit.inputs.push(EditorInput::new((response.rect.size() / 2.0).to_pos2()))
            }

            if component_response.clicked() {
                self.circuit.components.push(EditorComponent::new((response.rect.size() / 2.0).to_pos2(), Function::And));
            }
            
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

                    input.get_shape(to_screen, point_response.dragged())
                })
                .collect();
            
            let component_shapes: Vec<Shape> = self.circuit
                .components
                .iter_mut()
                .enumerate()
                .map(|(i, component)| {
                    let size = Vec2::splat(60.0);

                    let point_in_screen = to_screen.transform_pos(component.position);
                    let point_in_rect = Rect::from_center_size(point_in_screen, size);
                    let point_id = response.id.with("component".to_owned() + &i.to_string());
                    let point_response = ui.interact(point_in_rect, point_id, Sense::drag());
                    
                    component.position += point_response.drag_delta();
                    component.position = to_screen.from().clamp(component.position);
                    
                    component.get_shape(to_screen, point_response.dragged())
                })
                .collect();
            
            let mut dragged_line = None;
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
                    
                    if start_response.dragged() {
                        line.start += start_response.drag_delta();
                        line.start = to_screen.from().clamp(line.start);
                        
                        dragged_line = Some(i);
                    }
                    
                    let end_in_screen = to_screen.transform_pos(line.end);
                    let end_rect = Rect::from_center_size(end_in_screen, size);
                    let end_id = response.id.with("line end".to_string() + &i.to_string());
                    let end_response = ui.interact(end_rect, end_id, Sense::drag());
                    
                    if end_response.dragged() {
                        line.end += end_response.drag_delta();
                        line.end = to_screen.from().clamp(line.end);

                        dragged_line = Some(i);                        
                    }
                    
                    line.get_shape(to_screen, (start_response.dragged(), end_response.dragged()))
                })
                .collect();
                
            self.circuit.input_reconnect_lines_start(connected_lines_start, dragged_line);
            self.circuit.input_reconnect_lines_end(connected_lines_end, dragged_line);

            input_shapes.iter()
                .chain(component_shapes.iter())
                .chain(line_shapes.iter())
                .for_each(|shape| {
                    painter.add(shape.clone());
                });
        });
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            circuit: Default::default(),
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
    
    fn input_reconnect_lines_start(&mut self, connection_list: Vec<Vec<usize>>, dragged_line: Option<usize>) {
        connection_list.iter()
            .enumerate()
            .for_each(|(input_index, line_indices)| {
                let connector_position = self.inputs[input_index].position + Vec2::new(20.0, 0.0);

                line_indices.iter()
                    .filter(|&line_index| {
                        match dragged_line {
                            Some(dragged_index) => *line_index != dragged_index,
                            None => true,
                        }
                    })
                    .for_each(|&line_index| {
                        self.lines[line_index].start = connector_position;
                    })

            })
    }

    fn input_reconnect_lines_end(&mut self, connection_list: Vec<Vec<usize>>, dragged_line: Option<usize>) {
        connection_list.iter()
            .enumerate()
            .for_each(|(input_index, line_indices)| {
                let connector_position = self.inputs[input_index].position + Vec2::new(20.0, 0.0);

                line_indices.iter()
                    .filter(|&line_index| {
                        match dragged_line {
                            Some(dragged_index) => *line_index != dragged_index,
                            None => true,
                        }
                    })
                    .for_each(|&line_index| {
                        self.lines[line_index].end = connector_position;
                    })

            })
    }
}