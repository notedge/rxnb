#[derive(Debug, Copy, Clone)]
pub enum KaTeXError {}

pub type KaTeXResult<T> = std::result::Result<T, KaTeXError>;
