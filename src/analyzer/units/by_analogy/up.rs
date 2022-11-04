use crate::analyzer::units::abc::AnalyzerUnit;
use crate::analyzer::MorphAnalyzer;
use crate::container::abc::*;
use crate::container::stack::StackAffix;
use crate::container::{Affix, Lex, ParseResult, Parsed, SeenSet};
use crate::util::{add_parsed_if_not_seen, word_splits};

// FIXME unused const MIN_REMINDER_LENGTH: isize = 3;

/// Parses the word by parsing only the word suffix
/// (with restrictions on prefix & suffix lengths).
///
/// Example: байткод -> (байт) + код
#[derive(Debug, Clone, Copy)]
pub struct UnknownPrefixAnalyzer {
    estimate_decay: f64,
}

impl Default for UnknownPrefixAnalyzer {
    fn default() -> Self {
        UnknownPrefixAnalyzer {
            estimate_decay: 0.5,
        }
    }
}

impl AnalyzerUnit for UnknownPrefixAnalyzer {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        seen_parses: &mut SeenSet,
    ) {
        log::trace!("UnknownPrefixAnalyzer::parse()");
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        for (prefix, unprefixed_word) in word_splits(word_lower, None, None) {
            let mut subresult = ParseResult::new();
            morph.units.dictionary.parse(
                morph,
                &mut subresult,
                unprefixed_word,
                unprefixed_word,
                seen_parses,
            );

            'iter_parses: for parsed in subresult {
                let tag = parsed.lex.get_tag(morph);
                if !tag.is_productive() {
                    continue 'iter_parses;
                }
                let stack = parsed.lex.stack.stack.left.stack;
                let affix = Affix::unknown_prefix(prefix);
                let lex = Lex::from_stack(morph, StackAffix::new(stack, affix));
                let score = parsed.score * self.estimate_decay;
                add_parsed_if_not_seen(morph, result, seen_parses, Parsed::new(lex, score));
            }
        }
    }
}
