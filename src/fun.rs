use std::ops::Deref;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, sync::LazyLock};
use std::process::Command;
use rand::Rng;
use xdg_user;
use zbus::blocking::{connection::{self, Builder}, Connection};
use x11::xlib;

struct State {
    connection: Connection,
    gnome_overview_state: bool
}

impl State {
    fn new() -> Self {
        State {
            connection: establish_dbus_connection(),
            gnome_overview_state: false
        }
    }
}

static FUN_INSTANCE: LazyLock<State> = std::sync::LazyLock::new(|| State::new());
static GNOME_OVERVIEW_STATE: Mutex<bool> = Mutex::new(false);

fn establish_dbus_connection() -> Connection {
    connection::Builder::session().unwrap().build().unwrap()
}


pub fn notify(title: &str, body: &str) {
    let id: u32 = rand::rng().random_range(0..u32::MAX);

    let _m = FUN_INSTANCE.connection.call_method(
        Some("org.freedesktop.Notifications"),
        "/org/freedesktop/Notifications",
        Some("org.freedesktop.Notifications"),
        "Notify",
        &("Metal Crusher", id, "dialog-information", title, body,
        vec![""; 0], HashMap::<&str, &zbus::zvariant::Value>::new(), 3000),);
}


pub fn toggle_desktop_overview() {
    static mut GNOME_STATE: Option<bool> = Some(false);

    if is_kde_plasma() {
        let _m = FUN_INSTANCE.connection.call_method(
            Some("org.kde.plasmashell"),
            "/PlasmaShell",
            Some("org.kde.PlasmaShell"),
            "toggleDashboard",
            &(),); // KDE Plasma
    }

    if is_gnome() {

        unsafe { GNOME_STATE = Some(!GNOME_STATE.unwrap()) };


        if unsafe { GNOME_STATE.unwrap() } {
            let gnome_command = "dbus-send --session --dest=org.gnome.Shell --type=method_call /org/gnome/Shell org.freedesktop.DBus.Properties.Set string:org.gnome.Shell string:OverviewActive variant:boolean:true";
            execute_shell(gnome_command);
        } else {
            let gnome_command = "dbus-send --session --dest=org.gnome.Shell --type=method_call /org/gnome/Shell org.freedesktop.DBus.Properties.Set string:org.gnome.Shell string:OverviewActive variant:boolean:false";
            execute_shell(gnome_command);
        }
    }
}

pub fn draw_creepy_shit() {
    let display = unsafe { xlib::XOpenDisplay(std::ptr::null()) };
    let screen = unsafe { xlib::XDefaultScreen(display) };

    let x: i32 = rand::rng().random_range(10..800);
    let y: i32 = rand::rng().random_range(10..600);

    let x1: u32 = rand::rng().random_range(50..800);
    let y1: u32 = rand::rng().random_range(50..600);

    let window = unsafe { xlib::XCreateSimpleWindow(
        display,
        xlib::XRootWindow(display, screen),
        x,
        y,
        x1,
        y1,
        1,
        xlib::XBlackPixel(display, screen),
        xlib::XWhitePixel(display, screen),
    ) };
    unsafe { xlib::XMapWindow(display, window) };

    let gc = unsafe { xlib::XCreateGC(display, window, 0, core::ptr::null_mut()) };
    unsafe { xlib::XSetForeground(display, gc, xlib::XBlackPixel(display, screen)) };

    // Draw a creepy face
    unsafe { xlib::XDrawArc(display, window, gc, 100, 100, 200, 200, 0, 360 * 64) };
    unsafe { xlib::XDrawArc(display, window, gc, 150, 150, 50, 50, 0, 360 * 64) };
    unsafe { xlib::XDrawArc(display, window, gc, 250, 150, 50, 50, 0, 360 * 64) };
    unsafe { xlib::XDrawLine(display, window, gc, 200, 200, 200, 300) };

    // Update the display
    unsafe { xlib::XFlush(display) };

    // Wait for 5 seconds
    thread::sleep(Duration::from_secs(5));

    // Close the display
    unsafe { xlib::XCloseDisplay(display) };
}

pub fn is_kde_plasma() -> bool {
    let xdg_current_desktop = std::env::var("XDG_CURRENT_DESKTOP");

    xdg_current_desktop.is_ok() && xdg_current_desktop.unwrap().contains("KDE")
}

pub fn is_gnome() -> bool {
    let xdg_current_desktop = std::env::var("XDG_CURRENT_DESKTOP");

    xdg_current_desktop.is_ok() && xdg_current_desktop.unwrap().contains("GNOME")
}

pub fn reanimate_gui_shell() {
    if is_kde_plasma() {
        execute_shell("plasmashell");
    }
}

pub fn put_random_shit_on_desktop() {
    let desktop_dir_res = xdg_user::desktop();

    if !desktop_dir_res.is_ok() {
        return;
    }

    let desktop_dir_option = desktop_dir_res.unwrap();

    if !desktop_dir_option.is_some() {
        return;
    }

    let desktop_dir = desktop_dir_option.unwrap();

    for _ in 0..12 {
        let file_name = format!("CRUSHER_{}", rand::rng().random::<u32>());
        let file_path = desktop_dir.join(file_name);
        let mut file = std::fs::File::create(file_path).unwrap();
        let random_text = "METAL CRUSHER";
        std::io::Write::write_all(&mut file, random_text.as_bytes()).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

pub fn execute_shell(command: &str) {
    std::process::Command::new("sh").arg("-c").arg(command).spawn().unwrap();
}

pub fn set_kde_brightness(brightness: u32) {
    let _m = FUN_INSTANCE.connection.call_method(
        Some("org.freedesktop.PowerManagement"),
        "/org/kde/Solid/PowerManagement/Actions/BrightnessControl",
        Some("org.kde.Solid.PowerManagement.Actions.BrightnessControl"),
        "setBrightness",
        &(brightness),);
}

pub fn detect_terminal() -> &'static str {
    let terminals = ["kgx", "ghostty", "konsole", "alacritty", "gnome-terminal", "xterm"];
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
