#[derive(Debug)]
pub struct DtpFileDto {
    content: String,
}

impl DtpFileDto {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string().clone()
        }
    }
}