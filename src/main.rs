use std::fs;

use eframe::{egui::{self, TopBottomPanel, SidePanel, Label, Sense, ViewportBuilder}, epaint::Vec2};
use simulator::{Circuit, simulator::Simulator};

const EXAMPLE: &str = r#"{"inputs":[{"value_index":0},{"value_index":1}],"outputs":[{"value_index":2}],"components":[{"input_value_indices":[0,1],"output_value_indices":[2],"owned_value_indices":[],"function":"And"}],"value_list_len":3}"#;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2::new(1024.0, 840.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "circuitbuilder",
        options,
        Box::new(|_cc| Box::<CircuitBuilder>::default())
    )
}

#[derive(Default)]
struct CircuitBuilder {
    circuit: Option<Circuit>,
    selected_element: SelectedElement,
    simulator: Option<Simulator>,
    occupied_sides: OccupiedSides,
}

#[derive(Default)]
struct OccupiedSides {
    top: f32,
    left: f32,
}

#[derive(Default)]
enum SelectedElement {
    #[default]
    None,
    Input(usize),
    Output(usize),
    Component(usize),
}

impl eframe::App for CircuitBuilder {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {

        if let Some(simulator) = &mut self.simulator {
            simulator.simulate();
        }

        self.menu_bar(ctx);
        self.explorer(ctx);
    }
}

impl CircuitBuilder {
    fn menu_bar(&mut self, ctx: &egui::Context) {
        self.occupied_sides.top = TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("New").clicked() {
                    self.circuit = Some(Circuit::new());
                }

                if ui.button("Load").clicked() {
                    if let Some(file) = rfd::FileDialog::new().pick_file() {
                        let contents = fs::read_to_string(file).unwrap();

                        self.circuit = Some(serde_json::from_str(&contents).unwrap());
                    }
                }

                if ui.button("Example").clicked() {
                    self.circuit = Some(serde_json::from_str(EXAMPLE).unwrap());
                }

                if let Some(circuit) = &self.circuit {

                    if self.simulator.is_none() {
                        if ui.button("Start simulation").clicked() {
                            self.simulator = Some(Simulator::new(circuit.clone()));
                        }
                    } else {
                        if ui.button("Stop simulation").clicked() {
                            self.simulator = None;
                        }
                    }
                }
            })
        }).response.rect.height();
    }

    fn explorer(&mut self, ctx: &egui::Context) {
        self.occupied_sides.left = SidePanel::left("explorer").show(ctx, |ui| {
            if let Some(circuit) = &self.circuit {

                ui.label("Circuit");
                ui.separator();

                ui.collapsing("Inputs", |ui| {
                        for i in 0..circuit.all_inputs().len() {
                            let value_index = circuit.all_inputs()[i].value_index();

                            if ui.add(Label::new(format!("{i}: value index: {value_index}")).sense(Sense::click())).clicked() {
                                self.selected_element = SelectedElement::Input(i);
                            }
                        }
                    });

                ui.collapsing("Outputs", |ui| {
                        for i in 0..circuit.all_outputs().len() {
                            let value_index = circuit.all_outputs()[i].value_index();

                            if ui.add(Label::new(format!("{i}: value index: {value_index}")).sense(Sense::click())).clicked() {
                                self.selected_element = SelectedElement::Output(i);
                            }
                        }
                    });

                ui.collapsing("Components", |ui| {
                        for i in 0..circuit.all_components().len() {
                            let component = &circuit.all_components()[i];
                            let function = component.function();

                            ui.collapsing(format!("{i}: {function}"), |ui| {
                                if ui.add(Label::new(format!("Select")).sense(Sense::click())).clicked() {
                                    self.selected_element = SelectedElement::Component(i);
                                }

                                ui.collapsing("Inputs", |ui| {
                                        for i in 0..component.input_value_indices().len() {
                                            let value_index = component.input_value_indices()[i];
                                            ui.label(format!("{i}: value index: {value_index}"));
                                        }
                                    });

                                ui.collapsing("Outputs", |ui| {
                                        for i in 0..component.output_value_indices().len() {
                                            let value_index = component.output_value_indices()[i];
                                            ui.label(format!("{i}: value index: {value_index}"));
                                        }
                                    });
                                });
                        }
                    });
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());

        }).response.rect.height();
    }
}