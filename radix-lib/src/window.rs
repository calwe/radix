#[derive(Default, Clone)]
pub struct Window {
    pub(crate) title: Option<String>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) scale: u32,
}

impl Window {
    pub fn new(width: u32, height: u32, scale: u32) -> Self {
        Self {
            width,
            height,
            scale,
            ..Default::default()
        }
    }

    pub fn with_title(width: u32, height: u32, scale: u32, title: &str) -> Self {
        Self {
            width,
            height,
            scale,
            title: Some(title.to_string()),
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}