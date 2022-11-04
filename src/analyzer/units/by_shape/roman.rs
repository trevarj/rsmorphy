use crate::analyzer::units::abc::AnalyzerUnit;
use crate::analyzer::MorphAnalyzer;
use crate::container::abc::*;
use crate::container::stack::StackSource;
use crate::container::{Lex, ParseResult, Parsed, SeenSet, Shaped};
use crate::opencorpora::OpencorporaTagReg;
use crate::shapes::is_roman_number;

#[derive(Debug, Clone)]
pub struct RomanAnalyzer {
    pub tag: OpencorporaTagReg,
}

impl Default for RomanAnalyzer {
    fn default() -> Self {
        RomanAnalyzer {
            tag: OpencorporaTagReg::new("ROMN"),
        }
    }
}

impl AnalyzerUnit for RomanAnalyzer {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        _seen_parses: &mut SeenSet,
    ) {
        log::trace!("RomanAnalyzer::parse()");
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);
        if !is_roman_number(word_lower) {
            return;
        }
        let shaped = Shaped::roman_number(word_lower);
        let score = shaped.score();
        let lex = Lex::from_stack(morph, StackSource::from(shaped));
        result.push(Parsed::new(lex, score));
    }
}
