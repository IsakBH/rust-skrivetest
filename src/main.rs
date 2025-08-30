use simple_stopwatch::Stopwatch;
use std::io;
use std::time::Duration;

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

    let text = "hei. jeg heter jonas gahr støre.";
    println!("Yo, du skal skrive dette:\n{text}");
    let mut stopwatch = Stopwatch::start_new();
    let mut user_text = String::new();
    io::stdin()
        .read_line(&mut user_text)
        .expect("Failed to read line");
    let user_time = stopwatch.s();
    if(user_text == text) {
        println!("Du brukte {user_time} sekunder på å skrive det.");
    }
    else {
        println!("Det du skrev matchet ikke det du skulle skrive. Bad boy.");
    }
}
