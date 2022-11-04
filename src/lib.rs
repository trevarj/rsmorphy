#![deny(bare_trait_objects)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::match_bool)]
#![allow(clippy::module_inception)]

#[macro_use]
pub mod macros;

pub mod analyzer;
pub mod container;
pub mod dawg;
pub mod estimator;
pub mod opencorpora;
pub mod release;
pub mod shapes;
pub mod util;

pub mod prelude;

pub use crate::analyzer::units::abc::AnalyzerUnit;
pub use crate::analyzer::MorphAnalyzer;
pub use crate::container::abc::{MorphySerde, Source};
pub use crate::container::{Lex, ParseResult, Parsed, Score};
pub use crate::opencorpora::{Grammeme, GrammemeSet};
