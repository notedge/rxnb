pub use error::{KaTeXError, KaTeXResult};

mod error;
mod options;
mod lexer;

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
