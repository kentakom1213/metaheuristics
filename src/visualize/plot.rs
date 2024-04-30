//! プロットを行うためのトレイト

use egui_plotter::EguiBackend;
use plotters::prelude::ChartContext;
use plotters::{coord::types::RangedCoordf64, prelude::Cartesian2d};

/// chartの型
pub type Chart<'a> = ChartContext<'a, EguiBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

/// Problemの描画を行う
pub trait PlotProblem {
    fn plot_problem(&self, chart: &mut Chart, x_range: (f64, f64), y_range: (f64, f64));
}

/// Optimizerの描画を行う
pub trait PlotOptimizer {
    /// 目的関数のプロットを行う
    fn plot_problem(&self, chart: &mut Chart);
    /// 現在の解のプロットを行う
    fn plot_current_solution(&self, chart: &mut Chart);
}
