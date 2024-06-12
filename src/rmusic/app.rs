use gtk::{Application, ApplicationWindow, ContainerExt, GtkWindowExt, ToolButtonExt, WidgetExt};

use super::{toolbar::MusicToolbar, PAUSE_STOCK, PLAY_STOCK};
use gtk:: {
    Adjustment,
    Image,
    ImageExt,
    Scale,
    ScaleExt,
    Orientation::{Horizontal, Vertical}
};

pub struct App {
    adjustment: Adjustment,
    cover: Image,
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    pub fn new(app: Application) -> Self {
        let window = ApplicationWindow::new(&app);
        window.set_title("Rusic");
        let vbox = gtk::Box::new(Vertical, 0);
        window.add(&vbox);

        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());        
        let cover = Image::new();
        cover.set_from_file("src/assets/beach.jpg");
        vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.show_all();

        let app = App {
            adjustment,
            cover,
            toolbar,
            window,
        };
        app.connect_events();
        app.connect_toolbar_events();
        app
    }

    fn connect_events(&self) {

    }
    pub fn connect_toolbar_events(&self){
        let window = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move|_|{
            window.destroy();
        });
        let play_button = self.toolbar.play_button.clone();
        self.toolbar.play_button.connect_clicked(move|_|{
            if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()){
                play_button.set_stock_id(PAUSE_STOCK);
            } else {
                play_button.set_stock_id(PLAY_STOCK);
            }
        });
    }
}