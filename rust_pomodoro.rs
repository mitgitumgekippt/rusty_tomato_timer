use std::{thread, time};

fn main() {
    timer();
}

fn timer() {
    print!("Los geht's! ");
    print!("Die derzeitige Pomodoro-Zeit ist auf 25min und die Pausen-Zeit ist auf 5min gesetzt.");

    let five_min = time::Duration::from_secs(300);
    
    let mut n = 0;
    while n < 5 {
        println!("Bereits {} min des Pomodorotimers vergangen!", 5*n);
        thread::sleep(five_min);
        n += 1;
    }

    println!("Super, du hast dir eine Pause verdient!");
    thread::sleep(five_min);
}
