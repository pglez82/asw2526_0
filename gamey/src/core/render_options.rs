pub struct RenderOptions {
    pub show_3d_coords: bool,
    pub show_idx: bool,
    pub show_colors: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions {
            show_3d_coords: false,
            show_idx: true,
            show_colors: true,
        }
    }
}
