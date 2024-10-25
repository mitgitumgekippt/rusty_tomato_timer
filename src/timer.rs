use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time};

pub fn productive_time() {
    print!("Los geht's! ");
    println!(
        "Die derzeitige Pomodoro-Zeit ist auf 25min und die Pausen-Zeit ist auf 5min gesetzt."
    );
    timer(25);
}

pub fn break_time(long: bool) {
    if long {
        println!("Du hast dir ne Pause verdient!");
        timer(5);
    } else {
        println!(" Jetzt ist Zeit für eine lange Pause!");
        timer(15);
    }
}

pub fn timer(lenght: i32) {
    let one_min = time::Duration::from_secs(60);

    for n in (1..(lenght + 1)).rev() {
        println!("Noch {} min bis der Pomodorotimer klingelt!", n);
        thread::sleep(one_min);
    }
}
