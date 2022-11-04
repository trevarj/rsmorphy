use std::borrow::Cow;
use std::fmt;

use crate::analyzer::MorphAnalyzer;
use crate::container::abc::*;
use crate::container::paradigm::ParadigmId;
use crate::container::stack::{StackAffix, StackSource};
use crate::container::{Lex, Score};
use crate::opencorpora::OpencorporaTagReg;

#[derive(Debug, Clone, PartialEq)]
pub struct StackHyphenated {
    pub left: StackAffix,
    pub right: Option<StackAffix>,
}

impl StackHyphenated {
    pub fn new<R>(left: StackAffix, right: R) -> Self
    where
        R: Into<Option<StackAffix>>,
    {
        let right = right.into();
        StackHyphenated { left, right }
    }

    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> impl Iterator<Item = Lex> + 'i {
        self.left.iter_lexeme(morph).map(move |lex: Lex| {
            Lex::from_stack(morph, StackHyphenated::new(lex.stack.stack.left, None))
        })
    }
}

impl From<StackAffix> for StackHyphenated {
    fn from(stack: StackAffix) -> Self {
        StackHyphenated {
            left: stack,
            right: None,
        }
    }
}

impl From<StackSource> for StackHyphenated {
    fn from(stack: StackSource) -> Self {
        StackAffix::from(stack).into()
    }
}

impl Source for StackHyphenated {
    fn score(&self) -> Score {
        unimplemented!()
    }

    fn is_lemma(&self) -> bool {
        match self.right {
            None => self.left.is_lemma(),
            Some(ref right) => self.left.is_lemma() && right.is_lemma(),
        }
    }

    fn is_known(&self) -> bool {
        match self.right {
            None => self.left.is_known(),
            Some(ref right) => self.left.is_known() && right.is_known(),
        }
    }

    fn get_word(&self) -> Cow<str> {
        match self.right {
            None => self.left.get_word(),
            Some(ref right) => Cow::from(format!("{}-{}", self.left.get_word(), right.get_word())),
        }
    }

    fn get_normal_form(&self, morph: &MorphAnalyzer) -> Cow<str> {
        match self.right {
            None => self.left.get_normal_form(morph),
            Some(ref right) => format!(
                "{}-{}",
                self.left.get_normal_form(morph),
                right.get_normal_form(morph)
            )
            .into(),
        }
    }

    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg {
        match self.right {
            None => self.left.get_tag(morph),
            Some(_) => unimplemented!(),
        }
    }

    fn try_get_para_id(&self) -> Option<ParadigmId> {
        match self.right {
            None => self.left.try_get_para_id(),
            Some(_) => unimplemented!(),
        }
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        self.left.write_word(f)?;
        if let Some(ref right) = self.right {
            right.write_word(f)?;
        }
        Ok(())
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        self.left.write_normal_form(f, morph)?;
        if let Some(ref right) = self.right {
            right.write_normal_form(f, morph)?;
        }
        Ok(())
    }

    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        self.iter_lexeme(morph).collect()
    }

    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        self.iter_lexeme(morph).next().unwrap()
    }
}

impl MorphySerde for StackHyphenated {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        match self.right {
            Some(ref right) => {
                write!(f, "hw:")?;
                self.left.encode(f)?;
                write!(f, ";-")?;
                right.encode(f)
            }
            None => self.left.encode(f),
        }
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let (s, stack) = StackAffix::decode(s)?;
        let result = (s, StackHyphenated::new(stack, None));
        // TODO
        //        if !s.is_empty() {
        //            match (|s| {
        //                let s = follow_str(s, ";")?;
        //                let s = follow_str(s, "hw").map_err(|e| match e {
        //                    DecodeError::DoesntMatch => DecodeError::UnknownPartType,
        //                    _ => e,
        //                })?;
        //                let (s, word) = take_str_until_char_is(follow_str(s, ":")?,
        // ';')?;                Ok((s, HyphenSeparatedParticle {
        //                    particle: word.to_string(),
        //                }))
        //            })(s) {
        //                Err(DecodeError::UnknownPartType) => (),
        //                Err(e) => Err(e)?,
        //                Ok((s, particle)) => {
        //                    result.0 = s;
        //                    result.1.particle = Some(particle);
        //                },
        //            };
        //        }
        Ok(result)
    }
}
