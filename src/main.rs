use eframe::egui::{self, CentralPanel, Visuals};
use egui_plotter::EguiBackend;
use metaheuristics::{
    opt::{algorithms::hill_climbing::HillClimbingAlgorithm, optimize::Optimizer},
    problem::{benchmark::ackley::Ackley1d, problem::Problem},
    visualize::plot::{Chart, PlotOptimizer, PlotProblem},
};
use plotters::{
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    style::{Color, IntoFont, BLACK, WHITE},
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

impl<P: Problem + PlotProblem, Opt: Optimizer<P> + PlotOptimizer> eframe::App
    for Visualizer<P, Opt>
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // 描画エリア
            let root = EguiBackend::new(ui).into_drawing_area();

            // 白で埋める
            root.fill(&WHITE).ok();

            // 枠の設定
            let mut chart: Chart = ChartBuilder::on(&root)
                .caption("Problem", ("sans-serif", 30).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(-30.0..30.0, 0.0..25.0)
                .unwrap();

            // グリッドの描画
            chart.configure_mesh().draw().ok();

            // 目的関数のプロット
            self.optimizer.plot_problem(&mut chart);

            // ラベルの設定と描画
            chart
                .configure_series_labels()
                .background_style(WHITE.mix(0.8))
                .border_style(BLACK)
                .draw()
                .unwrap();

            // 完了
            root.present().ok();
        });
    }
}
