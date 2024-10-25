# Rusty-Tomato-Timer

## Project Description
Hey, yet another Pomodoro-Timer implementation!
Rusty-Tomato-Timer is a simple CLI pomodoro-timer, written in the Rust language (as you may guessed it from the name...).
It creates an ineractive shell, with which you can interact with the timer.

> [!NOTE]
> This work is mainly a learn-project, so excuse me if it is a bit overengineered sometimes.

#### What is the pomodoro technique?
Pomodoro is a technique in which you break work into time intervals, typically 25 minutes.
Before the start of the timer, you decide for a task to complete (Pro Tip: Write it down!).
While the timer runs, you work on the task.
After each completed interval, you make a 5 minute break (and after four completed a 15 minute break).
If you get distracted during the interval, you start from new!
More information can be found on the [Wikipedia-Page](https://en.wikipedia.org/wiki/Pomodoro_Technique).

## Installation & Usage
If (for some reason) you were dreaming using a half-baked pomodoro-timer like this, you can istall it by:
```bash
rustc main_interactive_shell.rs
```
Afterwards you can run it with:
```bash
./main_interactive_shell
```

## Feature Ideas
- [x] Interactive Shell
- [ ] Interactive Shell while the timer runs
- [ ] Jump to Break (When task is competed early)
- [ ] Write down Task
- [ ] Eyecandy: Figlet-Time
- [ ] Statistics
- [ ] Store Settings
- [ ] Help-Page

- [ ] Setting: Auto-continue timer
- [ ] Setting: Custom time-intervals
- [ ] Setting: Write down Task

( - [ ] Add task from WebDAV)
