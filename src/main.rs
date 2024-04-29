use eframe::egui::{self, CentralPanel, Visuals};
use egui_plotter::EguiBackend;
use metaheuristics::problem::benchmark::ackley::Ackley1d;
use plotters::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Metaheuristics Visualizer",
        native_options,
        Box::new(|cc| Box::new(Simple::new(cc))),
    )
    .unwrap();
}

struct Simple;

impl Simple {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        Self
    }
}

impl eframe::App for Simple {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // 描画エリア
            let root = EguiBackend::new(ui);

            let ack = Ackley1d;

            ack.plot_eframe(root, (-30.0, 30.0), (0.0, 25.0), 1000).ok();
        });
    }
}
