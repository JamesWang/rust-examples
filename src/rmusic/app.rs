use std::{path::PathBuf, rc::Rc};

use gtk::{Application, ApplicationWindow, ContainerExt, DialogExt, FileChooserDialog, FileChooserExt, FileFilter, FileFilterExt, GtkWindowExt, ToolButtonExt, WidgetExt};
use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};

use super::{playlist::Playlist, toolbar::MusicToolbar, PAUSE_STOCK, PLAY_STOCK};
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
    //cover: Image,
    playlist: Rc<Playlist>,
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
        //let cover = Image::new();
        //cover.set_from_file("src/assets/beach.jpg");
        //vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        let playlist = Rc::new(Playlist::new());
        vbox.add(playlist.view());

        window.show_all();

        let app = App {
            adjustment,
            //cover,
            playlist,
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
        let playlist = self.playlist.clone();
        let parent = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move|_|{
            window.destroy();
        });
        self.toolbar.open_button.connect_clicked(move|_|{
            let file = Self::show_open_dialog(&parent);
            if let Some(f) = file {
                playlist.add(&f)
            }
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


fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
    let mut file = None;
    let dialog = FileChooserDialog::new(
        Some("Select an MP3 audio file"), 
        Some(parent), 
        gtk::FileChooserAction::Open
    );

    let filter = FileFilter::new();
    filter.add_mime_type("audio/mp3");
    filter.set_name("MP3 audio file");
    dialog.add_filter(&filter);
    dialog.add_button("Cancel", GTK_RESPONSE_CANCEL);
    dialog.add_button("Accept", GTK_RESPONSE_ACCEPT);
    let result = dialog.run();
    if result == GTK_RESPONSE_ACCEPT{
        file = dialog.get_filename();
    }
    dialog.destroy();
    file
}
}