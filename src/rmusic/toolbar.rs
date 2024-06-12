use gtk:: {
    ContainerExt,
    SeparatorToolItem,
    Toolbar,
    ToolButton,
    ToolButtonExt,
    WidgetExt,
};

use super::PLAY_STOCK;


pub struct MusicToolbar {
    pub open_button: ToolButton,
    pub next_button: ToolButton,
    pub play_button: ToolButton,
    pub prev_button: ToolButton,
    pub quit_button: ToolButton,
    pub remove_button: ToolButton,
    pub stop_button: ToolButton,
    pub toolbar: Toolbar,
}

impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();

        let open_button = ToolButton::new_from_stock("gtk-open");
        toolbar.add(&open_button);

        toolbar.add(&SeparatorToolItem::new());
        let prev_button = ToolButton::new_from_stock("gtk-media-previous");
        toolbar.add(&prev_button);

        let play_button = ToolButton::new_from_stock(PLAY_STOCK);
        toolbar.add(&play_button);

        let stop_button = ToolButton::new_from_stock("gtk-media-stop");
        toolbar.add(&stop_button);

        let next_button = ToolButton::new_from_stock("gtk-media-next");
        toolbar.add(&next_button);

        toolbar.add(&SeparatorToolItem::new());

        let remove_button = ToolButton::new_from_stock("gtk-remove");
        toolbar.add(&remove_button);
        toolbar.add(&SeparatorToolItem::new());

        let quit_button = ToolButton::new_from_stock("gtk-quit");
        toolbar.add(&quit_button);
        MusicToolbar {
            open_button,
            next_button,
            play_button,
            prev_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,
        }
    }
    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}