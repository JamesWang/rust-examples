extern crate gdk_pixbuf;
extern crate id3;

use std::{cell::RefCell, cmp::max, path::Path, sync::{Arc, Mutex}, thread};
use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};
use gtk::{
    CellLayoutExt,
    CellRendererPixbuf,
    CellRendererText,
    ListStore,
    ListStoreExt,
    ListStoreExtManual,
    StaticType,
    ToValue,
    TreeIter,
    TreeModelExt, 
    TreeSelectionExt,
    TreeView,
    TreeViewColumn,
    TreeViewColumnExt,
    TreeViewExt,
    Type,
    WidgetExt,
};

use id3::Tag;
use self::Visibility::*;
use super::{player::{Player, State}, utils::to_millis};

const THUMBNAIL_COLUMN: u32 = 0;
const TITLE_COLUMN: u32 = 1;
const ARTIST_COLUMN: u32 = 2;
const ALBUM_COLUMN: u32 = 3;
const GENRE_COLUMN: u32 = 4;
const YEAR_COLUMN: u32 = 5;
const TRACK_COLUMN: u32 = 6;
const PATH_COLUMN: u32 = 7;
const PIXBUF_COLUMN: u32 = 8;

const IMAGE_SIZE: i32 = 256;
const THUMBNAIL_SIZE: i32 = 64;
const INTERP_HYPER: InterpType = 3;

pub struct Playlist {
    current_song: RefCell<Option<String>>,
    model: ListStore,
    player: Player,
    state: Arc<Mutex<State>>,
    treeview: TreeView,
}


#[derive(PartialEq)]
enum Visibility {
    Invisible,
    Visible,
}

impl Playlist {

    pub fn new(state: Arc<Mutex<State>>) -> Self {
        let model = ListStore::new(&[
            Pixbuf::static_type(),
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Pixbuf::static_type(),
        ]);
        let treeview = TreeView::new_with_model(&model);
        treeview.set_hexpand(true);
        treeview.set_vexpand(true);
        Self::create_columns(&treeview);
        
        Playlist {
            current_song: RefCell::new(None),
            model,
            player: Player::new(state.clone()),
            state,
            treeview
        }
    }

    fn create_columns(treeview: &TreeView) {
        Self::add_pixbuf_column(treeview, THUMBNAIL_COLUMN as i32, Visible);
        Self::add_text_column(treeview, "Title", TITLE_COLUMN as i32);
        Self::add_text_column(treeview, "Artist", TITLE_COLUMN as i32);
        Self::add_text_column(treeview, "Album", TITLE_COLUMN as i32);
        Self::add_text_column(treeview, "Genre", TITLE_COLUMN as i32);
        Self::add_text_column(treeview, "Year", TITLE_COLUMN as i32);
        Self::add_text_column(treeview, "Track", TITLE_COLUMN as i32);
        Self::add_pixbuf_column(treeview, PIXBUF_COLUMN as i32, Invisible);
    }
    
    fn add_text_column(treeview: &TreeView, title: &str, column: i32){
        let view_column = TreeViewColumn::new();
        view_column.set_title(title);
        let cell = CellRendererText::new();
        view_column.set_expand(true);
        view_column.pack_start(&cell, true);
        view_column.add_attribute(&cell, "text", column);
        treeview.append_column(&view_column);
    }

    fn add_pixbuf_column(treeview: &TreeView, column: i32, visibility: Visibility) {
        let view_column = TreeViewColumn::new();
        if visibility == Visible {
            let cell = CellRendererPixbuf::new();
            view_column.pack_start(&cell, true);
            view_column.add_attribute(&cell, "pixbuf", column);
        }
        treeview.append_column(&view_column);
    }

    pub fn view(&self) -> &TreeView {
        &self.treeview
    }

    fn set_pixbuf(&self, row: &TreeIter, tag: &Tag) {
        if let Some(picture) = tag.pictures().next() {
            let pixbuf_loader = PixbufLoader::new();
            pixbuf_loader.set_size(IMAGE_SIZE, IMAGE_SIZE);
            pixbuf_loader.loader_write(&picture.data).unwrap();
            if let Some(pixbuf) = pixbuf_loader.get_pixbuf() {
                let thumbnail = pixbuf.scale_simple(
                    THUMBNAIL_SIZE, 
                    THUMBNAIL_SIZE, 
                    INTERP_HYPER).unwrap();
                self.model.set_value(row, THUMBNAIL_COLUMN, &thumbnail.to_value());
                self.model.set_value(row, PIXBUF_COLUMN, &pixbuf.to_value())
            }
            pixbuf_loader.close().unwrap();
        }
    }

    pub fn add(&self, path: &Path) {
    
        let filename = path.file_stem().unwrap_or_default().to_str().unwrap_or_default();
        let row = self.model.append();
        if let Ok(tag) = Tag::read_from_path(path) {
            let title = tag.title().unwrap_or(filename);
            let artist = tag.artist().unwrap_or("(no artist)");
            let album = tag.album().unwrap_or("(no album)");
            let genre = tag.genre().unwrap_or("(no genre)");
            let year = tag.year().map(|year|year.to_string()).unwrap_or("(no year)".to_string());
            let track = tag.track().map(|track|track.to_string()).unwrap_or("??".to_string());

            let total_tracks = tag.total_tracks().map(|tt| tt.to_string()).unwrap_or("??".to_string());
            let track_value = format!("{} / {}", track, total_tracks);

            self.set_pixbuf(&row, &tag);

            self.model.set_value(&row, TITLE_COLUMN, &title.to_value());
            self.model.set_value(&row, ARTIST_COLUMN, &artist.to_value());
            self.model.set_value(&row, ALBUM_COLUMN, &album.to_value());
            self.model.set_value(&row, GENRE_COLUMN, &genre.to_value());
            self.model.set_value(&row, YEAR_COLUMN, &year.to_value());
            self.model.set_value(&row, TRACK_COLUMN, &track_value.to_value());
        } else {
            self.model.set_value(&row, TITLE_COLUMN, &filename.to_value());
        }

        let path = path.to_str().unwrap_or_default();
        self.model.set_value(&row, PATH_COLUMN, &path.to_value());
    }

    pub fn remove_selection(&self) {
        let selection = self.treeview.get_selection();
        println!("remove_selection {:?}", selection.get_selected_rows().1);
        if let Some((_, iter)) = selection.get_selected() {
            self.model.remove(&iter);
        }
    }
    pub fn pixbuf(&self) -> Option<Pixbuf> {
        let selection = self.treeview.get_selection();
        if let Some((_, iter)) = selection.get_selected(){
            let value = self.model.get_value(&iter, PIXBUF_COLUMN as i32);
            return value.get::<Pixbuf>();
        }
        None
    }

    fn selected_path(&self) -> Option<String> {
        let selection = self.treeview.get_selection();
        if let Some((_, iter)) = selection.get_selected() {
            let value = self.model.get_value(&iter,PATH_COLUMN as i32);
            return value.get::<String>();
        }
        None
    }
    pub fn pause(&self) {
        self.player.pause()
    }
    
    pub fn play(&self) -> bool {
        if let Some(path) = self.selected_path() {
            if self.player.is_paused() && Some(&path) == self.path().as_ref() {
                self.player.resume();
            } else {
                self.player.load(&path);
                *self.current_song.borrow_mut() = Some(path.into());
            }
            true
        } else {
            false
        }
    }
    pub fn path(&self)-> Option<String> {
        self.current_song.borrow().clone()
    }
    pub fn stop(&self) {
        *self.current_song.borrow_mut() = None;
        self.player.stop();
    }

    pub fn next(&self) -> bool {
        let selection = self.treeview.get_selection();
        let next_iter = 
            if let Some((_, iter)) = selection.get_selected() {
                if !self.model.iter_next(&iter){
                    return false;
                }
                Some(iter)
            } else {
                self.model.get_iter_first()
            };
        if let Some(ref iter) = next_iter {
            selection.select_iter(iter);
            self.play();
        }
        next_iter.is_some()
    }

    pub fn previous(&self)  -> bool {
        let selection = self.treeview.get_selection();
        let previous_iter = 
            if let Some((_, iter)) = selection.get_selected() {
                if !self.model.iter_previous(&iter) {
                    return false;
                }
                Some(iter)
            } else {
                self.model.iter_nth_child(None,max(0,self.model.iter_n_children(None)-1))
            };
        if let Some(ref iter) = previous_iter {
            selection.select_iter(iter);
            self.play();
        }
        previous_iter.is_some()
    }


    pub fn load(&self, path: &Path) {
        let mut reader = m3u::Reader::open(path).unwrap();
        for entry in reader.entries() {
            if let Ok(m3u::Entry::Path(path)) = entry {
                self.add(&path);
            }
        }
    }
}


