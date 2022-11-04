use crate::analyzer::MorphAnalyzer;
use crate::container::{ParseResult, SeenSet};

pub trait AnalyzerUnit {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        seen_parses: &mut SeenSet,
    );
}
