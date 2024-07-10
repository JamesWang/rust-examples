use std::{collections::HashMap, path::PathBuf, rc::Rc, sync::{Arc, Mutex}};

use gtk::{AdjustmentExt, Application, ApplicationWindow, ContainerExt, Continue, DialogExt, FileChooserDialog, FileChooserExt, FileFilter, FileFilterExt, GtkWindowExt, ToolButtonExt, WidgetExt};
use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};

use super::{player::State, playlist::Playlist, toolbar::MusicToolbar, PAUSE_STOCK, PLAY_STOCK};
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
    playlist: Rc<Playlist>,
    state: Arc<Mutex<State>>,
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
        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        let current_time = 0;
        let durations = HashMap::new();

        let state = Arc::new(Mutex::new(State{
            current_time,
            durations,
            stopped: true,
        }));

        let playlist = Rc::new(Playlist::new(state.clone()));
        vbox.add(playlist.view());

        let cover = Image::new();
        cover.set_from_file("src/assets/b2.png");
        
        vbox.add(&cover);


        window.show_all();

        let app = App {
            adjustment,
            cover,
            playlist,
            state,
            toolbar,
            window,
        };
        app.connect_events();
        app.connect_toolbar_events();
        app
    }


    fn set_cover(cover: &Image, playlist: &Playlist) {
        cover.set_from_pixbuf(playlist.pixbuf().as_ref());
        cover.show();
    }

    fn connect_events(&self) {
        let playlist = self.playlist.clone();
        let adjustment = self.adjustment.clone();
        let state = self.state.clone();
        //let play_image = self.toolbar.play_image.clone();
        gtk::timeout_add(100, move||{
            let state = state.lock().unwrap();
            if let Some(path)  = playlist.path() {
                if let Some(&durations) = state.durations.get(&path){
                    adjustment.set_upper(durations as f64);
                }
            }
            /*if state.stopped {
                set_image_icon(&play_image, PLAY_ICON);
            } else {
                set_image_icon(&play_image, PAUSE_ICON);
            }
            */
            adjustment.set_value(state.current_time as f64);
            Continue(true)
        });
    }
    pub fn connect_toolbar_events(&self){
        let window = self.window.clone();
        let playlist = self.playlist.clone();
        let state  = self.state.clone();
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
        let playlist = self.playlist.clone();
        let play_button = self.toolbar.play_button.clone();
        //let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();

        self.toolbar.play_button.connect_clicked(move|_|{
            if state.lock().unwrap().stopped {
                if playlist.play() {
                    //set_image_icon(&play_image, PAUSE_ICON);
                    //set_cover(&cover, &playlist);
                } else {
                    playlist.pause();
                    //set_image_icon(&play_image, PLAY_ICON);
                }
            }
            if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()){
                play_button.set_stock_id(PAUSE_STOCK);
                Self::set_cover(&cover, &playlist);
            } else {
                playlist.pause();
                play_button.set_stock_id(PLAY_STOCK);
            }
        });
        let playlist = self.playlist.clone();
        self.toolbar.remove_button.connect_clicked(move|_|{playlist.remove_selection();});

        let playlist = self.playlist.clone();
        //let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        let play_button = self.toolbar.play_button.clone();
        self.toolbar.stop_button.connect_clicked(move|_|{
            playlist.stop();
            cover.hide();
            //set_image_icon(&play_image, PLAY_ICON);
            //set_cover(&cover, &playlist);
            play_button.set_stock_id(PLAY_STOCK);
        });

        let playlist = self.playlist.clone();
        //let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        self.toolbar.next_button.connect_clicked(move|_|{
            if playlist.next() {
                //set_image_icon(&play_image, PAUSE_ICON);
                //set_cover(&cover, &playlist);
            }
        });

        let playlist = self.playlist.clone();
        //let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        self.toolbar.prev_button.connect_clicked(move|_|{
            playlist.previous();
            //set_image_icon(&play_image, PLAY_ICON);
            //set_cover(&cover, &playlist);
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