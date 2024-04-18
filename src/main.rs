//! # プロットのチュートリアル
//! 参考: <https://github.com/emilk/egui/blob/master/examples/save_plot/src/main.rs>

use std::f64::consts::PI;

use eframe::{egui, Theme};
use egui_plot::{Legend, Line, Plot, PlotPoints, Points};

/// ウィンドウのサイズ
const WINDOW_SIZE: [f32; 2] = [1000.0, 800.0];

fn main() -> Result<(), eframe::Error> {
    // ロガーのセットアップ
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        follow_system_theme: false,
        default_theme: Theme::Light,
        ..Default::default()
    };

    eframe::run_native(
        "Metaheuristics Visualizer",
        options,
        Box::new(|_cc| Box::<VisualizerApp>::default()),
    )
}

/// アプリの中身
#[derive(Default)]
struct VisualizerApp;

impl eframe::App for VisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 四角形
        let mut plot_rect = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Save Plot").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
            }

            // プロット
            let my_plot = Plot::new("My Plot").legend(Legend::default());

            // グラフのデータ
            let sin_curve: PlotPoints = (0..1000)
                .map(|i| (2.0 * PI * i as f64 / 1000.0 - PI))
                .map(|x| [x, x.sin()])
                .collect();

            let cos_points: PlotPoints = (0..200)
                .map(|i| (2.0 * PI * i as f64 / 200.0 - PI))
                .map(|x| [x, x.cos()])
                .collect();

            let inner = my_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(sin_curve).name("sin"));
                plot_ui.points(Points::new(cos_points).radius(1.0).name("cos"));
            });

            plot_rect = Some(inner.response.rect);
        });

        // スクリーンショットの確認
        let screenshot = ctx.input(|i| {
            for event in &i.raw.events {
                if let egui::Event::Screenshot { image, .. } = event {
                    return Some(image.clone());
                }
            }
            None
        });

        // 保存ダイアログの表示
        if let (Some(screenshot), Some(plot_location)) = (screenshot, plot_rect) {
            if let Some(mut path) = rfd::FileDialog::new().save_file() {
                path.set_extension("png");

                let pixels_per_point = ctx.pixels_per_point();
                let plot = screenshot.region(&plot_location, Some(pixels_per_point));

                // 画像を保存
                image::save_buffer(
                    &path,
                    plot.as_raw(),
                    plot.width() as u32,
                    plot.height() as u32,
                    image::ColorType::Rgba8,
                )
                .unwrap();

                eprintln!("Image saved to {path:?}.");
            }
        }
    }
}
