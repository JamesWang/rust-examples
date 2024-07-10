extern crate gio;
extern crate gtk;
extern crate gdk_pixbuf;
extern crate id3;
extern crate gtk_sys;
extern crate crossbeam;
extern crate pulse_simple;

use std::env;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk:: {
    Application, ApplicationWindow, GtkWindowExt, WidgetExt
};

use gtk:: {
    ContainerExt, 
    SeparatorToolItem,
    Toolbar, 
    ToolButton
};
use log::info;
use simple_logger::SimpleLogger;

const PLAY_STOCK: &str = "gtk-media-play";

#[path = "../rmusic/mod.rs"]
mod app;


fn main() {
    SimpleLogger::new().with_utc_timestamps().init().unwrap();
    info!("initialize logger");
    let application = Application::new("music.example", ApplicationFlags::empty())
    .expect("Application initialization failed");
    //application.connect_startup(startup_handler);
    application.connect_startup(|app|{app::app::App::new(app.clone());});
    application.connect_activate(|_|{});
    application.run(&env::args().collect::<Vec<_>>());
}

fn startup_handler(app: &Application) {
    let window = ApplicationWindow::new(&app);
    window.set_title("Rusic");
    let tool_bar = Toolbar::new();
    let open_button: ToolButton = ToolButton::new_from_stock("gtk-open");
    tool_bar.add(&open_button);
    tool_bar.add(&SeparatorToolItem::new());
    tool_bar.add(&ToolButton::new_from_stock("gtk-media-previous"));
    tool_bar.add(&ToolButton::new_from_stock(PLAY_STOCK));
    tool_bar.add(&ToolButton::new_from_stock("gtk-media-stop"));
    tool_bar.add(&ToolButton::new_from_stock("gtk-media-next"));
    tool_bar.add(&SeparatorToolItem::new());
    tool_bar.add(&ToolButton::new_from_stock("gtk-remove"));
    tool_bar.add(&SeparatorToolItem::new());
    tool_bar.add(&ToolButton::new_from_stock("gtk-quit"));

    window.add(&tool_bar);
    window.connect_delete_event(|_, _|{gtk::Inhibit(false)}); //put true, window will not be close    
    //window.show();
    window.show_all();
}