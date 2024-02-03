use eframe::{egui::{self, CentralPanel, Sense}, epaint::{vec2, Pos2, Rect, Vec2}};
use egui::*;
use simulator::{element::Input, function::Function};

use self::{connection::{Connection, Connections, Element, LinePoint, LinePointIndex}, elements::{EditorComponent, EditorInput, EditorLine, EditorOutput, EditorShape, CONNECTION_RADIUS}};

mod connection;
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
                let output_response = ui.button("Output");
                
                (input_response, component_response, output_response)
            });
            
            let (input_response, component_response, output_response) = button_responses.inner;

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

            if output_response.clicked() {
                self.circuit.outputs.push(EditorOutput::new((response.rect.size() / 2.0).to_pos2()));
            }
            
            let mut moved_input = None;
            let mut released_input = None;

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
                    
                    if point_response.dragged() {
                        input.position += point_response.drag_delta();
                        input.position = to_screen.from().clamp(input.position);
                        
                        moved_input = Some(i);
                        
                    } else if point_response.drag_released() {
                        released_input = Some(i);
                    }
                    
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
            
            if let Some(moved_input) = moved_input {
                self.circuit.apply_input_connections(moved_input);
            }
            
            if let Some(released_input) = released_input {
                let connection_position = self.circuit.inputs[released_input].position + vec2(20.0, 0.0);
                let connection = Connection {
                    element: Element::Input,
                    index: released_input,
                };

                self.circuit
                    .lines
                    .iter()
                    .enumerate()
                    .for_each(|(i, line)| {
                        if connection_position.distance(line.start) < CONNECTION_RADIUS {
                            let line_point_index = LinePointIndex {
                                index: i,
                                point: LinePoint::Start,
                            };

                            self.circuit.connections.insert_connection(line_point_index, connection.clone());

                        } else if connection_position.distance(line.end) < CONNECTION_RADIUS {
                            let line_point_index = LinePointIndex {
                                index: i,
                                point: LinePoint::End,
                            };

                            self.circuit.connections.insert_connection(line_point_index, connection.clone());

                        }
                    });

                self.circuit.apply_input_connections(released_input);
            };

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

            let output_shapes: Vec<Shape> = self.circuit
                .outputs
                .iter_mut()
                .enumerate()
                .map(|(i, output)| {
                    let size = Vec2::splat(60.0);

                    let point_in_screen = to_screen.transform_pos(output.position);
                    let point_in_rect = Rect::from_center_size(point_in_screen, size);
                    let point_id = response.id.with("output".to_owned() + &i.to_string());
                    let point_response = ui.interact(point_in_rect, point_id, Sense::drag());
                    
                    output.position += point_response.drag_delta();
                    output.position = to_screen.from().clamp(output.position);

                    let connector_position = point_in_screen + Vec2::new(-20.0, 0.0);
                    let connector_rect = Rect::from_center_size(connector_position, Vec2::splat(10.0));
                    let connector_id = response.id.with("output connector".to_owned() + &i.to_string());
                    let connector_response = ui.interact(connector_rect, connector_id, Sense::click());
                    
                    if connector_response.clicked() {
                        self.circuit.lines.push(EditorLine::from_single_pos(output.position + vec2(-20.0, 0.0)));
                    }
                    
                    output.get_shape(to_screen, point_response.dragged())
                })
                .collect();
            
            let mut move_started = None;
            let mut moved_line = None;

            let line_shapes: Vec<Shape> = self.circuit
                .lines
                .iter_mut()
                .enumerate()
                .map(|(i, line)| {
                    let size = Vec2::splat(10.0);

                    let start_dragged = {
                        let start_in_screen = to_screen.transform_pos(line.start);
                        let start_rect = Rect::from_center_size(start_in_screen, size);
                        let start_id = response.id.with("line start".to_string() + &i.to_string());
                        let start_response = ui.interact(start_rect, start_id, Sense::drag());
                        
                        if start_response.drag_started() {
                            move_started = Some(LinePointIndex { index: i, point: LinePoint::Start })
                        }
                        
                        if start_response.dragged() {
                            line.start += start_response.drag_delta();
                            line.start = to_screen.from().clamp(line.start);
                            
                            moved_line = Some(i);
                        }
                        
                        start_response.dragged()
                    };
                    
                    let end_dragged = {
                        let end_in_screen = to_screen.transform_pos(line.end);
                        let end_rect = Rect::from_center_size(end_in_screen, size);
                        let end_id = response.id.with("line end".to_string() + &i.to_string());
                        let end_response = ui.interact(end_rect, end_id, Sense::drag());
                        
                        if end_response.drag_started() {
                            move_started = Some(LinePointIndex { index: i, point: LinePoint::End })
                        }

                        if end_response.dragged() {
                            line.end += end_response.drag_delta();
                            line.end = to_screen.from().clamp(line.end);

                            moved_line = Some(i);                        
                        }
                        
                        end_response.dragged()
                    };
                    
                    line.get_shape(to_screen, (start_dragged, end_dragged))
                })
                .collect();
            
            if let Some(move_started) = move_started {
                self.circuit.connections.remove_for(move_started);
            }
            
            input_shapes.iter()
                .chain(component_shapes.iter())
                .chain(output_shapes.iter())
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
    connections: Connections,
}

impl EditorCircuit {
    fn apply_input_connections(&mut self, index: usize) {
        let connection_position = self.inputs[index].position + vec2(20.0, 0.0);

        let connection = Connection {
            element: Element::Input,
            index,
        };
        
        self.connections.connections_for_connected(connection)
            .iter()
            .for_each(|line_point_index| {
                match line_point_index.point {
                    LinePoint::Start => self.lines[line_point_index.index].start = connection_position,
                    LinePoint::End => self.lines[line_point_index.index].end = connection_position,
                }
            });
    }
}