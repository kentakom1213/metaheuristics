use metaheuristics::{
    opt::{
        algorithms::hill_climbing::{HillClimbingAlgorithm, MultiHillClimbingAlgorithm},
        optimize::Optimizer,
    },
    problem::{benchmark::parabola::Parabola2d, problem::Problem},
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
    let mut opt = HillClimbingAlgorithm::new(p, (2.0, 20.0));

    eprintln!("init: {:?}", opt.get_best());

    // 1ステップ
    opt.step();

    eprintln!("1st: {:?}", opt.get_best());

    // 1000ステップ
    opt.optimize(1000);

    eprintln!("1000th: {:?}", opt.get_best());
}

/// 山登り法での結果
///
/// - Ellipsoid関数: <https://yuyumoyuyu.com/wp-content/uploads/2021/03/benchmarkfunction.pdf>
#[test]
fn test_hill_climbing_ellipsoid() {
    let x_range = (-100.0, 100.0);
    let y_range = (-100.0, 100.0);

    let p = Parabola2d::new(1.0, 0.0, 1000_000.0, 0.0, x_range, y_range);

    // オプティマイザの構成
    let mut opt = HillClimbingAlgorithm::new(p, (1.0, 1000_000.0));

    eprintln!("init: {:?}", opt.get_best());

    // 1ステップ
    opt.step();

    eprintln!("1st: {:?}", opt.get_best());

    // 1000ステップ
    opt.optimize(1000);

    eprintln!("1000th: {:?}", opt.get_best());
}

/// 多点スタート山登り法での結果
#[test]
fn test_multi_hill_climbing() {
    let x_range = (-100.0, 100.0);
    let y_range = (-100.0, 100.0);

    let p = Parabola2d::new(2.0, 1.0, 20.0, 0.0, x_range, y_range);

    // オプティマイザの構成
    let mut opt = MultiHillClimbingAlgorithm::new(p, 100, (2.0, 20.0));

    eprintln!("init: {:?}", opt.get_best());

    // 1ステップ
    opt.step();

    eprintln!("1st: {:?}", opt.get_best());

    // 1000ステップ
    opt.optimize(1000);

    eprintln!("1000th: {:?}", opt.get_best());
}
