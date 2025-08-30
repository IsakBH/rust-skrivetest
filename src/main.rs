use simple_stopwatch::Stopwatch;
use std::time::Duration;
use std::io;

fn main() {
    println!("Er du klar for å skrive?");
    let mut ready = String::new();
    io::stdin()
        .read_line(&mut ready)
        .expect("Failed to read line");
    println!("Brukeren er klar.");

    println!("5..");
    std::thread::sleep(Duration::from_millis(1000));
    println!("4..");
    std::thread::sleep(Duration::from_millis(1000));
    println!("3..");
    std::thread::sleep(Duration::from_millis(1000));
    println!("2..");
    std::thread::sleep(Duration::from_millis(1000));
    println!("1..");
    std::thread::sleep(Duration::from_millis(1000));

    println!("Yo, du skal skrive dette:\njeg heter jonas gahr støre");
    let mut stopwatch = Stopwatch::start_new();
    let mut user_text = String::new();
    io::stdin()
        .read_line(&mut user_text)
        .expect("Failed to read line");
    let userTime = stopwatch.s();
    println!("Du brukte {userTime} sekunder på å skrive det.");
}
