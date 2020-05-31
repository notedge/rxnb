pub use error::{KaTeXError, KaTeXResult};

mod error;
mod lexer;
mod options;

#[derive(Debug, Clone)]
pub struct KaTeXEngine {
    cfg: (),
    ctx: (),
}

impl Default for KaTeXEngine {
    fn default() -> Self {
        unimplemented!()
    }
}
