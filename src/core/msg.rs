#[derive(Debug)]
pub struct MsgFileDto {
    content: String,
}

impl MsgFileDto {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string().clone()
        }
    }
}