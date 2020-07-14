#[derive(Debug)]
pub struct HeaderLines {
    pub id: String,
    pub title: String,
}

impl HeaderLines {
    pub fn new(id: String, title: String) -> Self {
        HeaderLines { id, title }
    }
}
