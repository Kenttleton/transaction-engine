use std::fmt;

#[derive(Copy, Clone)]
pub struct Client {
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {:.4}, {:.4}, {:.4}, {}", self.client, self.available, self.held, self.total, self.locked)
    }
}