//! # プロットのチュートリアル
//! 参考: <https://github.com/emilk/egui/blob/master/examples/save_plot/src/main.rs>

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

fn main() -> Result<(), eframe::Error> {
    // ロガーのセットアップ
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App with a plot.",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

/// アプリの中身
#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
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
            let graph: Vec<[f64; 2]> = vec![[0.0, 1.0], [2.0, 3.0]];

            let inner = my_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(graph)).name("curve"));
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
