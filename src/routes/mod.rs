//! src/routes/mod.rs
mod ping;
mod hospitais;
mod serializers;
mod leitos;
mod pacientes;

pub use ping::*;
pub use hospitais::*;
pub use serializers::*;
pub use leitos::*;
pub use pacientes::*;