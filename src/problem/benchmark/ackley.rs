//! Ackley関数
//!
//! ### 参考
//! - [Ackley function - wikipedia](https://en.wikipedia.org/wiki/Ackley_function)
//! - [最適化手法評価のためのベンチマーク関数](https://yuyumoyuyu.com/wp-content/uploads/2021/03/benchmarkfunction.pdf)

use std::{
    f64::consts::{E, PI},
    fs,
};

use egui_plotter::EguiBackend;
use plotters::{
    backend::SVGBackend,
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    element::PathElement,
    series::LineSeries,
    style::{Color, IntoFont, BLACK, RED, WHITE},
};
use rand::Rng;

use crate::problem::problem::{Neighbor, Problem};

/// 1次元のAckley関数
#[derive(Debug)]
pub struct Ackley1d;

impl Problem for Ackley1d {
    type X = f64;
    type Y = f64;

    fn sample(&self, rng: &mut rand::prelude::ThreadRng) -> Self::X {
        rng.gen_range(-30.0..=30.0)
    }

    fn eval(&self, &x: &Self::X) -> Self::Y {
        // 範囲外の場合はINF
        if x < -30.0 || 30.0 < x {
            return f64::INFINITY;
        }

        20.0 - 20.0 * (-0.2 * x.abs()).exp() + E - (2.0 * PI * x).cos().exp()
    }
}

impl Neighbor for Ackley1d {
    fn get_neighbor(
        &self,
        x: &Self::X,
        eps: &Self::X,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Self::X {
        rng.gen_range(x - eps..=x + eps)
    }
}

impl Ackley1d {
    /// 関数をプロットする
    pub fn plot_svg(
        &self,
        _x_range @ (x_min, x_max): (f64, f64),
        _y_range @ (y_min, y_max): (f64, f64),
        num: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert!(x_min < x_max);
        // プロット幅
        let x_width = x_max - x_min;

        // ディレクトリの作成
        fs::create_dir_all("output")?;

        // 描画エリアの設定
        let root = SVGBackend::new("output/sample.svg", (640, 480)).into_drawing_area();

        root.fill(&WHITE)?;

        // 枠の設定
        let mut chart = ChartBuilder::on(&root)
            .caption("Ackley function", ("Ubuntu Mono", 30).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        // 枠の描画
        chart.configure_mesh().draw()?;

        // プロット
        chart
            .draw_series(LineSeries::new(
                (0..num)
                    .map(|x| x_min + x_width * (x as f64 / num as f64))
                    .map(|x| (x, self.eval(&x))),
                &RED,
            ))?
            .label("y = f(x)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        // ラベルの設定と描画
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        // 結果の保存
        root.present()?;

        Ok(())
    }

    /// GUI画面に関数をプロットする
    pub fn plot_eframe(
        &self,
        backend: EguiBackend,
        _x_range @ (x_min, x_max): (f64, f64),
        _y_range @ (y_min, y_max): (f64, f64),
        num: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert!(x_min < x_max);
        // プロット幅
        let x_width = x_max - x_min;

        let root = backend.into_drawing_area();

        root.fill(&WHITE)?;

        // 枠の設定
        let mut chart = ChartBuilder::on(&root)
            .caption("Ackley function", ("sans-serif", 30).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        // 枠の描画
        chart.configure_mesh().draw()?;

        // プロット
        chart
            .draw_series(LineSeries::new(
                (0..num)
                    .map(|x| x_min + x_width * (x as f64 / num as f64))
                    .map(|x| (x, self.eval(&x))),
                &RED,
            ))?
            .label("y = f(x)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        // ラベルの設定と描画
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        // 結果の保存
        root.present()?;

        Ok(())
    }
}
