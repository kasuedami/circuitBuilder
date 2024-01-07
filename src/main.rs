use std::fs;

use eframe::egui::{self, TopBottomPanel, SidePanel};
use simulator::{Circuit, simulator::Simulator};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
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
    simulator: Option<Simulator>,
    occupied_sides: OccupiedSides,
}

#[derive(Default)]
struct OccupiedSides {
    top: f32,
    left: f32,
}

impl eframe::App for CircuitBuilder {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.menu_bar(ctx);
        self.explorer(ctx);
    }
}

impl CircuitBuilder {
    fn menu_bar(&mut self, ctx: &egui::Context) {
        self.occupied_sides.top = TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("New").clicked() {

                }

                if ui.button("Load").clicked() {
                    if let Some(file) = rfd::FileDialog::new().pick_file() {
                        let contents = fs::read_to_string(file).unwrap();

                        self.circuit = Some(serde_json::from_str(&contents).unwrap());
                    }
                }

                if ui.button("Example").clicked() {
                    self.circuit = Some(Circuit::new());
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
                            ui.label(format!("{i}: value index: {value_index}"));
                        }
                    });

                ui.collapsing("Outputs", |ui| {
                        for i in 0..circuit.all_outputs().len() {
                            let value_index = circuit.all_outputs()[i].value_index();
                            ui.label(format!("{i}: value index: {value_index}"));
                        }
                    });

                ui.collapsing("Components", |ui| {
                        for i in 0..circuit.all_components().len() {
                            let component = &circuit.all_components()[i];
                            let function = component.function();

                            ui.collapsing(format!("Component: {function}"), |ui| {
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