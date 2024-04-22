//! 山登り法の実装

use rand::{rngs::ThreadRng, thread_rng};

use crate::{
    opt::optimize::Optimizer,
    problem::problem::{Neighbor, Problem},
};

/// 山登り法
#[derive(Debug)]
pub struct HillClimbingAlgorithm<P: Problem> {
    pub problem: P,
    pub x: P::X,
    pub score: P::Y,
    rng: ThreadRng,
}

impl<P: Problem + Neighbor> Optimizer<P> for HillClimbingAlgorithm<P> {
    fn new(problem: P) -> Self {
        let mut rng = thread_rng();
        // 初期解の生成
        let x_init = problem.sample(&mut rng);
        // 初期解の評価
        let score = problem.eval(&x_init);

        Self {
            problem,
            x: x_init,
            score,
            rng,
        }
    }

    fn step(&mut self) {
        // 近傍の生成
        let x_n = self.problem.get_neighbor(&self.x, &mut self.rng);

        // スコアが改善した場合、更新
        let n_score = self.problem.eval(&x_n);

        if self.score > n_score {
            self.x = x_n;
            self.score = n_score;
        }
    }

    fn get_best(&self) -> (&<P as Problem>::X, &<P as Problem>::Y) {
        (&self.x, &self.score)
    }
}
