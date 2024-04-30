use eframe::egui::{self, CentralPanel, Visuals};
use egui_plotter::EguiBackend;
use metaheuristics::{
    opt::{algorithms::hill_climbing::HillClimbingAlgorithm, optimize::Optimizer},
    problem::{benchmark::ackley::Ackley1d, problem::Problem},
};
use std::marker::PhantomData;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Metaheuristics Visualizer",
        native_options,
        Box::new(|cc| {
            Box::new(Visualizer::new(
                cc,
                HillClimbingAlgorithm::new(Ackley1d, 1.0),
            ))
        }),
    )
    .unwrap();
}

struct Visualizer<P: Problem, Opt: Optimizer<P>> {
    /// 最適化用のインスタンス
    optimizer: Opt,
    phantom: PhantomData<P>,
}

impl<P: Problem, Opt: Optimizer<P>> Visualizer<P, Opt> {
    fn new(cc: &eframe::CreationContext<'_>, optimizer: Opt) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        Self {
            optimizer,
            phantom: PhantomData::<P>,
        }
    }
}

impl<P: Problem, Opt: Optimizer<P>> eframe::App for Visualizer<P, Opt> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // 描画エリア
            let root = EguiBackend::new(ui);

            let ack = Ackley1d;

            ack.plot_eframe(root, (-30.0, 30.0), (0.0, 25.0), 1000).ok();
        });
    }
}
