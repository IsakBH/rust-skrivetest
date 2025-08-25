use std::time::Duration;
use simple_stopwatch::Stopwatch;

fn main() {
    let mut sw = Stopwatch::start_new();

    // vente litt tid
    std::thread::sleep(Duration::from_millis(1000));

    // hent tiden brukt
    let ms = sw.ms();
    println!("Tid brukt: {}", ms);
}
