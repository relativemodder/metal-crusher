use std::{collections::HashMap, sync::LazyLock};
use std::process::Command;
use rand::Rng;

use zbus::blocking::{connection::{self, Builder}, Connection};

struct State {
    connection: Connection,
}

impl State {
    fn new() -> Self {
        State {
            connection: establish_dbus_connection(),
        }
    }
}

static FUN_INSTANCE: LazyLock<State> = std::sync::LazyLock::new(|| State::new());


fn establish_dbus_connection() -> Connection {
    connection::Builder::session().unwrap().build().unwrap()
}


pub fn notify(title: &str, body: &str) {
    let id: u32 = rand::rng().random_range(0..u32::MAX);

    let m = FUN_INSTANCE.connection.call_method(
        Some("org.freedesktop.Notifications"),
        "/org/freedesktop/Notifications",
        Some("org.freedesktop.Notifications"),
        "Notify",
        &("Metal Crusher", id, "dialog-information", title, body,
        vec![""; 0], HashMap::<&str, &zbus::zvariant::Value>::new(), 3000),);
}

pub fn execute_shell(command: &str) {
    std::process::Command::new("sh").arg("-c").arg(command).spawn().unwrap();
}

pub fn detect_terminal() -> &'static str {
    let terminals = ["ghostty", "konsole", "alacritty", "gnome-terminal", "xterm"];
    for terminal in terminals {
        let path = format!("/usr/bin/{}", terminal);
        if std::path::Path::new(&path).exists() {
            return terminal;
        }
    }

    return "sh";
}

pub fn destroy_file_managers() {
    for _ in 0..5 {
        execute_shell("pkill -n dolphin"); 
        execute_shell("pkill -n nautilus"); 
        execute_shell("pkill -n nemo"); 
        execute_shell("pkill -n thunar");
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

pub fn destroy_gui_shell() {
    execute_shell("killall -9 plasmashell"); // KDE users will thank me xdxd
    std::thread::sleep(std::time::Duration::from_millis(600));
}

pub fn run_in_terminal(
    terminal: &'static str,
    command: &'static str
) -> bool {
    std::thread::spawn(move || {
        let _ = Command::new(terminal)
            .arg("-e")
            .arg("sh")
            .arg("-c")
            .arg(command)
            .output()
            .unwrap();

        return;
    });
    false
}