#[derive(Debug)]
pub struct FileDto {
    content: String,
}

impl FileDto {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string().clone(),
        }
    }
}
