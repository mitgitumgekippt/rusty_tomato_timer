use std::{thread, time};

/*
fn main() {
    let mut counter = 0;
    while true {
        productive_time();
        break_time();
        /*
        timer();
        counter += 1;
        pomo_break(counter);
         */
    }
}
 */

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

/*
fn pomo_break(counter: i32) {
    println!(
        "Super, du hast bereits {} Pomodoros abgeschlossen!",
        counter
    );

    let five_min = time::Duration::from_secs(300);
    let fifteen_min = time::Duration::from_secs(900);

    if (counter % 4) == 0 {
        println!(" Jetzt ist Zeit für eine lange Pause!");
        thread::sleep(fifteen_min);
    } else {
        thread::sleep(five_min);
    }
}
 */
