use metaheuristics::{
    opt::{algorithms::hill_climbing::HillClimbingAlgorithm, optimize::Optimizer},
    problem::{examples::parabola2d::Parabola2d, problem::Problem},
};
use rand::thread_rng;

#[test]
fn test_build() {
    let x_range = (-100.0, 100.0);
    let y_range = (-100.0, 100.0);

    let p = Parabola2d::new(2.0, 1.0, 20.0, 0.0, x_range, y_range);

    eprintln!("{:?}", p);

    let mut rng = thread_rng();

    for _ in 0..20 {
        eprintln!("{:?}", p.sample(&mut rng));
    }
}

/// 山登り法での結果
#[test]
fn test_hill_climbing() {
    let x_range = (-100.0, 100.0);
    let y_range = (-100.0, 100.0);

    let p = Parabola2d::new(2.0, 1.0, 20.0, 0.0, x_range, y_range);

    // オプティマイザの構成
    let mut opt = HillClimbingAlgorithm::new(p);

    eprintln!("init: {:?}", opt.get_best());

    // 1ステップ
    opt.step();

    eprintln!("1st: {:?}", opt.get_best());

    // 1000ステップ
    opt.optimize(1000);

    eprintln!("1000th: {:?}", opt.get_best());
}
