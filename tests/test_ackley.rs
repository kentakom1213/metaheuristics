use metaheuristics::{
    opt::{
        algorithms::hill_climbing::{HillClimbingAlgorithm, MultiHillClimbingAlgorithm},
        optimize::Optimizer,
    },
    problem::{benchmark::ackley::Ackley1d, problem::Problem},
};
use rand::thread_rng;

#[test]
fn test_build() {
    let mut rng = thread_rng();

    let ack = Ackley1d;

    for _ in 0..20 {
        let x = ack.sample(&mut rng);
        eprintln!("{}", x);
    }

    eprintln!("{}", ack.eval(&0.0));
}

/// 山登り法での結果
///
/// - Ellipsoid関数: <https://yuyumoyuyu.com/wp-content/uploads/2021/03/benchmarkfunction.pdf>
#[test]
fn test_hill_climbing_ellipsoid() {
    let p = Ackley1d;

    // オプティマイザの構成
    let mut opt = HillClimbingAlgorithm::new(p, 0.5);

    eprintln!("init: {:?}", opt);

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
    let p = Ackley1d;

    // オプティマイザの構成
    let mut opt = MultiHillClimbingAlgorithm::new(p, 100, 0.5);

    eprintln!("init: {:?}", opt.get_best());

    // 1ステップ
    opt.step();

    eprintln!("1st: {:?}", opt.get_best());

    // 1000ステップ
    opt.optimize(1000);

    eprintln!("1000th: {:?}", opt.get_best());
}
