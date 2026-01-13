fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Light;
    native_options.initial_window_size = Some(egui::Vec2 {x: 400.0, y: 300.0});

    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc)))
    );
}

struct MyEguiApp {
    click_pos: Vec<egui::Pos2>,
}

impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp {
            click_pos: vec![]
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    //fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    //    egui::CentralPanel::default().show(ctx, |ui| {
    //        ui.heading("Hello, World!");
    //        plot(ui);
    //    });
    //}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, World!");
            let resp = ui.allocate_response(egui::vec2(400.0, 300.0), egui::Sense::click());
            if resp.clicked() {
                let p = resp.interact_pointer_pos().unwrap();
                self.click_pos.push(p);
            }
            plot(ui, &self.click_pos);
        });
    }
}

fn _plot1(ui: &mut egui::Ui) {
    ui.painter().rect_filled(
        egui::Rect::from_min_max(
            egui::Pos2::new(50.0, 50.0),
            egui::Pos2::new(150.0, 150.0)
        ),
        egui::Rounding::same(20.0),
        egui::Color32::RED
    );
    ui.painter().rect_stroke(
        egui::Rect::from_min_max(
            egui::Pos2::new(100.0, 100.0),
            egui::Pos2::new(200.0, 200.0)
        ),
        egui::Rounding::none(),
        egui::Stroke::new(10.0, egui::Color32::GREEN)
    );
}

fn _plot2(ui: &mut egui::Ui) {
    let rect_1 = egui::Rect::from_min_max(
        egui::Pos2::new(50.0, 50.0),
        egui::Pos2::new(150.0, 150.0)
    );
    let round_1 = egui::Rounding::same(20.0);
    ui.painter().rect_filled(rect_1, round_1, egui::Color32::RED);

    let rect_2 = egui::Rect::from_min_max(
        egui::Pos2::new(100.0, 100.0),
        egui::Pos2::new(200.0, 200.0)
    );
    let round_2 = egui::Rounding::none();
    let stroke_2 = egui::Stroke::new(10.0, egui::Color32::GREEN);
    ui.painter().rect_stroke(rect_2, round_2, stroke_2);
}

fn _plot3(ui: &mut egui::Ui) {
    let pos_1 = egui::Pos2::new(100.0, 100.0);
    ui.painter().circle_filled(pos_1, 50.0, egui::Color32::RED);
    let pos_2 = egui::Pos2::new(150.0, 150.0);
    let stroke_2 = egui::Stroke::from((10.0, egui::Color32::GREEN));
    ui.painter().circle_stroke(pos_2, 50.0, stroke_2);
}

fn _plot4(ui: &mut egui::Ui) {
    let pos_1 = egui::Pos2::new(50.0, 50.0);
    let pos_2 = egui::Pos2::new(200.0, 200.0);
    let stroke_1 = egui::Stroke::new(5.0, egui::Color32::RED);
    let stroke_2 = egui::Stroke::new(5.0, egui::Color32::GREEN);

    ui.painter().vline(50.0, std::ops::RangeInclusive::new(50.0, 200.0), stroke_1);
    ui.painter().hline(std::ops::RangeInclusive::new(50.0, 200.0), 50.0, stroke_1);
    ui.painter().line_segment([pos_1, pos_2], stroke_2);
}

fn _plot5(ui: &mut egui::Ui) {
    ui.painter().text(
        egui::Pos2 {x: 50.0, y: 50.0},
        egui::Align2::LEFT_CENTER,
        "Hello!",
        egui::FontId::proportional(24.0),
        egui::Color32::RED
    );
    ui.painter().text(
        egui::Pos2 {x: 50.0, y: 100.0},
        egui::Align2::LEFT_CENTER,
        "Sample Message.",
        egui::FontId::proportional(36.0),
        egui::Color32::BLUE
    );
}

fn _plot6(ui: &mut egui::Ui) {
    let data = vec![
        egui::Pos2::new(50.0, 100.0),
        egui::Pos2::new(250.0, 100.0),
        egui::Pos2::new(75.0, 225.0),
        egui::Pos2::new(150.0, 50.0),
        egui::Pos2::new(225.0, 225.0),
    ];
    let stroke_1 = egui::Stroke::new(5.0, egui::Color32::RED);

    let mut shape_1 = eframe::epaint::PathShape::line(data, stroke_1);
    shape_1.closed = true;
    ui.painter().add(shape_1);
}

fn plot(ui: &mut egui::Ui, pos: &Vec<egui::Pos2>) {
    for p in pos {
        ui.painter().circle_filled(
            *p,
            25.0,
            egui::Color32::from_rgba_premultiplied(255, 0, 0, 100)
        );
    }
}

