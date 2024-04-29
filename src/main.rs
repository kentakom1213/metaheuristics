use metaheuristics::problem::benchmark::ackley::Ackley1d;

fn main() {
    let ack = Ackley1d;

    ack.plot((-30.0, 30.0), (0.0, 25.0), 1000).ok();
}
