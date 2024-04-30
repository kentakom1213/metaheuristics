pub mod problem {
    pub mod problem;

    /// 問題例
    pub mod benchmark {
        pub mod ackley;
        pub mod parabola;
    }
}

pub mod opt {
    pub mod optimize;

    /// 各種メタヒューリスティクス手法
    pub mod algorithms {
        pub mod hill_climbing;
    }
}

pub mod visualize {
    pub mod plot;
}
