use std::{collections::HashMap, sync::LazyLock};

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
    let m = FUN_INSTANCE.connection.call_method(
        Some("org.freedesktop.Notifications"),
        "/org/freedesktop/Notifications",
        Some("org.freedesktop.Notifications"),
        "Notify",
        &("Metal Crusher", 0u32, "dialog-information", title, body,
        vec![""; 0], HashMap::<&str, &zbus::zvariant::Value>::new(), 3000),);
}