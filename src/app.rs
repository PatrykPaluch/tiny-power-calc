use egui::{CentralPanel, Key, TextEdit, Widget};

const TARGET_WIDTH: f32 = 300.0;
const OUTPUT_PRECISION: usize = 7;

#[derive(Default)]
pub struct MyApp {
    autofocus: bool,
    history: Vec<String>,
    current_history_selection: Option<usize>,
    input_equation: String,
    non_history_element: String,
    response: String,
    previous_input: String,
    is_input_invalid: bool,
}

impl MyApp {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self {
            autofocus: true,
            ..Self::default()
        }
    }

    fn draw_ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.draw_input(ctx, ui);
        self.draw_output(ui);
    }

    fn draw_output(&mut self, ui: &mut egui::Ui) {
        let text_color = self.get_output_color();

        TextEdit::singleline(&mut self.response)
            .frame(false)
            .desired_width(TARGET_WIDTH)
            .interactive(false)
            .font(egui::TextStyle::Monospace)
            .horizontal_align(egui::Align::Center)
            .text_color(text_color)
            .ui(ui);
    }

    fn draw_input(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let text_response = TextEdit::singleline(&mut self.input_equation)
            .frame(false)
            .desired_width(TARGET_WIDTH)
            .hint_text("Enter an equation")
            .font(egui::TextStyle::Monospace)
            .horizontal_align(egui::Align::Center)
            .ui(ui);

        self.default_focus(&text_response);

        ctx.input(|i| {
            if i.key_pressed(Key::ArrowUp) {
                if let Some(index) = self.current_history_selection {
                    if index > 0 {
                        self.current_history_selection = Some(index - 1);
                        self.input_equation = self.history[index - 1].clone();
                    }
                } else if !self.history.is_empty() {
                    self.non_history_element = self.input_equation.clone();
                    self.current_history_selection = Some(self.history.len() - 1);
                    self.input_equation = self.history[self.history.len() - 1].clone();
                }
            } else if i.key_pressed(Key::ArrowDown) {
                if let Some(index) = self.current_history_selection {
                    if index < self.history.len() - 1 {
                        self.current_history_selection = Some(index + 1);
                        self.input_equation = self.history[index + 1].clone();
                    } else {
                        self.current_history_selection = None;
                        self.input_equation = self.non_history_element.clone();
                    }
                }
            } else if i.key_pressed(Key::Escape) {
                // ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                std::process::exit(0);
            }
        });

        if self.previous_input != self.input_equation {
            self.update_response(false);
        }

        if text_response.lost_focus() {
            text_response.request_focus();

            if self.input_equation.trim().is_empty() {
                return;
            }

            println!("Input equation: {}", self.input_equation);
            if self.input_equation.trim().to_lowercase() == "exit" {
                std::process::exit(0);
            }

            self.current_history_selection = None;
            self.history.push(self.input_equation.clone());
            self.update_response(true);
            self.input_equation.clear();
        }

        self.previous_input = self.input_equation.clone();
    }

    fn update_response(&mut self, show_invalid: bool) {
        let response = Self::calculate_response(&self.input_equation);
        self.is_input_invalid = response.is_none();
        if show_invalid {
            self.response = response.unwrap_or("Invalid input".to_string());
        } else if response.is_some() {
            self.response = response.unwrap();
        }
    }

    fn get_output_color(&self) -> egui::Color32 {
        if self.is_input_invalid {
            egui::Color32::DARK_GRAY
        } else {
            egui::Color32::WHITE
        }
    }

    fn default_focus(&mut self, response: &egui::Response) {
        if self.autofocus {
            response.request_focus();
            self.autofocus = false;
        }
    }

    fn calculate_response(input: &str) -> Option<String> {
        meval::eval_str(input)
            .map(|v| {
                format!("{:.1$}", v, OUTPUT_PRECISION)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            })
            .ok()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.0);
        CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::BLACK))
            .show(ctx, |ui| {
                self.draw_ui(ctx, ui);
            });
    }
}
