//! 山登り法の実装

use plotters::{element::Circle, style::BLUE};
use rand::{rngs::ThreadRng, thread_rng};

use crate::{
    opt::optimize::Optimizer,
    problem::problem::{Coord1D, Neighbor, Problem},
    visualize::plot::{PlotOptimizer, PlotProblem},
};

/// 山登り法
#[derive(Debug)]
pub struct HillClimbingAlgorithm<P: Problem> {
    pub problem: P,
    pub x: P::X,
    pub score: P::Y,
    pub eps: P::X,
    rng: ThreadRng,
}

impl<P: Problem + Neighbor> HillClimbingAlgorithm<P> {
    pub fn new(problem: P, eps: P::X) -> Self {
        let mut rng = thread_rng();
        // 初期解の生成
        let x_init = problem.sample(&mut rng);
        // 初期解の評価
        let score = problem.eval(&x_init);

        Self {
            problem,
            x: x_init,
            score,
            eps,
            rng,
        }
    }
}

impl<P: Problem + Neighbor> Optimizer<P> for HillClimbingAlgorithm<P> {
    fn step(&mut self) {
        // 近傍の生成
        let x_n = self.problem.get_neighbor(&self.x, &self.eps, &mut self.rng);

        // スコアが改善した場合、更新
        let n_score = self.problem.eval(&x_n);

        // ログ
        eprintln!(
            "[step] f({:.5?})={:.5?} -> f({:.5?})={:.5?}",
            self.x, self.score, x_n, n_score
        );

        if self.score > n_score {
            self.x = x_n;
            self.score = n_score;
        }
    }

    fn get_best(&self) -> (&<P as Problem>::X, &<P as Problem>::Y) {
        (&self.x, &self.score)
    }
}

/// 多点スタートの山登り法
#[derive(Debug)]
pub struct MultiHillClimbingAlgorithm<P: Problem> {
    pub problem: P,
    pub size: usize,
    pub x: Vec<P::X>,
    pub score: Vec<P::Y>,
    pub eps: P::X,
    rng: ThreadRng,
}

impl<P: Problem + Neighbor> MultiHillClimbingAlgorithm<P> {
    pub fn new(problem: P, size: usize, eps: P::X) -> Self {
        let mut rng = thread_rng();

        // 初期解の生成
        let x_init: Vec<P::X> = (0..size).map(|_| problem.sample(&mut rng)).collect();
        // 初期解の評価
        let score = x_init.iter().map(|x| problem.eval(x)).collect();

        Self {
            problem,
            size,
            x: x_init,
            score,
            eps,
            rng,
        }
    }
}

impl<P: Problem + Neighbor> Optimizer<P> for MultiHillClimbingAlgorithm<P> {
    fn step(&mut self) {
        for i in 0..self.size {
            // 近傍の生成
            let x_n = self
                .problem
                .get_neighbor(&self.x[i], &self.eps, &mut self.rng);

            // スコアが改善した場合、更新
            let n_score = self.problem.eval(&x_n);

            if self.score[i] > n_score {
                self.x[i] = x_n;
                self.score[i] = n_score;
            }
        }
    }

    fn get_best(&self) -> (&<P as Problem>::X, &<P as Problem>::Y) {
        let mut i_min = 0;
        let mut y_min = &self.score[0];

        for i in 1..self.size {
            if y_min > &self.score[i] {
                i_min = i;
                y_min = &self.score[i];
            }
        }

        (&self.x[i_min], &self.score[i_min])
    }
}

impl<P: Problem + PlotProblem + Coord1D<P>> PlotOptimizer for HillClimbingAlgorithm<P> {
    fn plot_problem(&self, chart: &mut crate::visualize::plot::Chart) {
        self.problem.plot_problem(chart, (0.0, 0.0), (0.0, 0.0));
    }

    fn plot_current_solution(&self, chart: &mut crate::visualize::plot::Chart) {
        // 現在の点のプロット
        chart
            .draw_series(
                [&self.x]
                    .iter()
                    .map(|&x| Circle::new(P::into_coord(x, &self.problem.eval(x)), 10, BLUE)),
            )
            .unwrap()
            .label("current solution");
    }
}
