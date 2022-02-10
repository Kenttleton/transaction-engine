use std::fmt;

pub struct Output {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}, {}, {}", self.client, self.available, self.held, self.total, self.locked)
    }
}