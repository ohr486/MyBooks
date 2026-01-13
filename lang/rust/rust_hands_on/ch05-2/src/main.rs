use eframe;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Light;
    native_options.initial_window_size = Some(egui::Vec2 {x: 400.0, y: 600.0});

    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc|
            Box::new(MyEguiApp::new(cc))
        )
    );
}

#[derive(PartialEq, Debug)]
enum RadioValue { First, Second, Third }

struct MyEguiApp {
    pub value1: usize,
    pub value2: bool,
    pub value3: RadioValue,
}

impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp {
            value1: 0,
            value2: true,
            value3: RadioValue::First,
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, World!");

            ui.separator();

            let msg = format!("click: {} times.", self.value1);
            let label_txt = egui::RichText::new(msg).size(32.0);
            let label = egui::Label::new(label_txt);
            ui.add(label);

            ui.spacing();

            let btn_txt = egui::RichText::new("Click!").font(egui::FontId::proportional(24.0));
            let btn = egui::Button::new(btn_txt);
            let resp = ui.add_sized(egui::Vec2 {x: 150.0, y: 40.0}, btn);
            if resp.clicked() {
                self.value1 += 1;
            }

            ui.separator();

            let msg = format!("checked = {}.", self.value2);
            let label_txt = egui::RichText::new(msg).size(32.0);
            let label = egui::Label::new(label_txt);
            ui.add(label);

            ui.spacing();

            let check_txt = egui::RichText::new("Checkbox").size(24.0);
            let check = egui::Checkbox::new(&mut self.value2, check_txt);
            let _resp = ui.add(check);

            ui.separator();

            let msg = format!("checked = {:?}.", self.value3);
            let label_txt = egui::RichText::new(msg).size(32.0);
            let label = egui::Label::new(label_txt);
            ui.add(label);

            ui.spacing();

            let label_1 = egui::RichText::new("First").size(24.0);
            let label_2 = egui::RichText::new("Second").size(24.0);
            let label_3 = egui::RichText::new("Third").size(24.0);
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.value3, RadioValue::First, label_1);
                ui.radio_value(&mut self.value3, RadioValue::Second, label_2);
                ui.radio_value(&mut self.value3, RadioValue::Third, label_3);
            });

            ui.separator();






        });
    }
}

