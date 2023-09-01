#[derive(Default, Clone)]
pub struct Window {
    title: Option<String>,
    width: u32,
    height: u32,
    scale: u32,
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

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }
}
