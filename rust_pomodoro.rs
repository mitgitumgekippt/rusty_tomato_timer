use std::{thread, time};

fn main() {
    let mut counter = 0;
    while false {
        timer();
        counter +=1;
        pomo_break(counter);
    }
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
    
}

fn pomo_break(counter: i32) {
        println!("Super, du hast bereits {} Pomodoros abgeschlossen!", counter);
   
    let five_min = time::Duration::from_secs(300);
    let fifteen_min = time::Duration::from_secs(900);


    if (counter % 4) == 0 { 
        println!(" Jetzt ist Zeit für eine lange Pause!");
        thread::sleep(fifteen_min);
    } else {
        thread::sleep(five_min);
    }
}
