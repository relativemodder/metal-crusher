use std::io::Cursor;

use zbus::blocking::connection::{self, Builder};

mod fun;
mod music;

fn main() {
    music::play_audio();

    fun::notify("Fun things will happen soon", ":D");

    std::thread::sleep(std::time::Duration::from_millis(1000));

    fun::notify("Fun things will happen soon", ":D");
    std::thread::sleep(std::time::Duration::from_millis(400));


    let user_terminal = fun::detect_terminal();

    std::thread::sleep(std::time::Duration::from_millis(2300));

    let file_managers_wave_1: [&'static str; 5] = [
        "xdg-open /", "xdg-open /bin", "xdg-open /lib", "xdg-open /home", "xdg-open /tmp"
    ];

    let terminals_wave_1: [&'static str; 5] = [
        "ls /; sleep 2000", "ls /tmp; sleep 2000", "ls /bin; sleep 2000", 
        "ls /usr; sleep 2000", "ls /etc; sleep 2000"
    ];

    for file_manager in file_managers_wave_1 {
        fun::execute_shell(file_manager);
        std::thread::sleep(std::time::Duration::from_millis(600));
    }

    for terminal_command in terminals_wave_1 {
        fun::run_in_terminal(user_terminal, terminal_command);
        std::thread::sleep(std::time::Duration::from_millis(600));
    }

    {
        fun::destroy_gui_shell();
        fun::destroy_file_managers();

        fun::execute_shell(format!("killall -9 {}", user_terminal).as_str());
        std::thread::sleep(std::time::Duration::from_millis(1200));
    }


    let file_managers_wave_2: [&'static str; 5] = [
        "xdg-open /dev", "xdg-open /sbin", "xdg-open /run", "xdg-open /sys", "xdg-open /proc"
    ];

    for file_manager in file_managers_wave_2 {
        fun::execute_shell(file_manager);
        std::thread::sleep(std::time::Duration::from_millis(600));
    }

    {
        fun::run_in_terminal(user_terminal, "tree /home; sleep 2000");
        std::thread::sleep(std::time::Duration::from_millis(600));

        for _ in 0..3 {
            fun::run_in_terminal(user_terminal, "ps aux; sleep 2000");
            std::thread::sleep(std::time::Duration::from_millis(600));
        }

    }

    {
        fun::destroy_gui_shell();

        fun::execute_shell(format!("killall -9 {}", user_terminal).as_str());
        std::thread::sleep(std::time::Duration::from_millis(1200));
        
        fun::destroy_file_managers();
    }

    
    std::thread::sleep(std::time::Duration::from_millis(9000));
}