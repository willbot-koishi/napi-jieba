#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{Error, Result};
use jieba_rs::{Jieba, Tag, Token, TokenizeMode};

#[napi(js_name = "Jieba")]
pub struct JsJieba {
    jieba: Jieba
}

#[napi(object, js_name = "Tag")]
pub struct JsTag {
    pub word: String,
    pub tag: String
}

#[napi(object, js_name = "WordDef")]
pub struct JsWordDef {
    pub word: String,
    pub tag: Option<String>,
    pub freq: Option<u32>
}

#[napi(js_name = "TokenizeMode")]
pub enum JsMode {
    Default,
    Search
}

#[napi(object, js_name = "Token")]
pub struct JsToken {
    pub word: String,
    pub start: u32,
    pub end: u32
}

impl Into<TokenizeMode> for JsMode {
    fn into(self) -> TokenizeMode {
        match self {
            JsMode::Default => TokenizeMode::Default,
            JsMode::Search => TokenizeMode::Search
        }
    }
}

#[napi]
impl JsJieba {
    #[napi(constructor)]
    pub fn new() -> Self {
        JsJieba {
            jieba: Jieba::new()
        }
    }

    #[napi(factory)]
    pub fn empty() -> Self {
        JsJieba {
            jieba: Jieba::empty()
        }
    }

    #[napi(factory)]
    pub fn with_word_defs(word_defs: Vec<JsWordDef>) -> Self {
        let mut js_jieba = Self::empty();
        js_jieba.add_words_from_defs(word_defs);
        js_jieba
    }

    #[napi]
    pub fn cut(&self, sentence: String, enable_hmm: Option<bool>) -> Vec<String> {
        self.jieba
            .cut(&sentence, enable_hmm.unwrap_or(true))
            .into_iter()
            .map(|str| str.to_string())
            .collect()
    }

    #[napi]
    pub fn cut_all(&self, sentence: String) -> Vec<String> {
        self.jieba
            .cut_all(&sentence)
            .into_iter()
            .map(|str| str.to_string())
            .collect()
    }

    #[napi]
    pub fn cut_for_search(&self, sentence: String, enable_hmm: Option<bool>) -> Vec<String> {
        self.jieba
            .cut_for_search(&sentence, enable_hmm.unwrap_or(true))
            .into_iter()
            .map(|str| str.to_string())
            .collect()
    }

    #[napi]
    pub fn cut_with_mode(&self, sentence: String, mode: JsMode, enable_hmm: Option<bool>) -> Vec<String> {
        match mode {
            JsMode::Default => self.cut(sentence, enable_hmm),
            JsMode::Search => self.cut_for_search(sentence, enable_hmm)
        }
    }

    #[napi]
    pub fn tag(&self, sentence: String, enable_hmm: Option<bool>) -> Vec<JsTag> {
        self.jieba
            .tag(&sentence, enable_hmm.unwrap_or(true))
            .into_iter()
            .map(|Tag { word, tag }| JsTag { word: word.into(), tag: tag.into() })
            .collect()
    }

    #[napi]
    pub fn add_word(&mut self, word: String, freq: Option<u32>, tag: Option<String>) {
        self.jieba.add_word(&word, freq.map(|size| size as usize), tag.as_deref());
    }

    #[napi]
    pub fn add_word_from_def(&mut self, JsWordDef { word, tag, freq }: JsWordDef) {
        self.add_word(word, freq, tag);
    }

    #[napi]
    pub fn add_words_from_defs(&mut self, word_defs: Vec<JsWordDef>) {
        for word_def in word_defs {
            self.add_word_from_def(word_def);
        }
    }

    #[napi]
    pub fn load_dict(&mut self, mut dict: &[u8]) -> Result<()> {
        self.jieba
            .load_dict(&mut dict)
            .map_err(|err| Error::from_reason(format!("Failed to load dict: {}", err.to_string())))?;
        Ok(())
    }

    #[napi]
    pub fn tokenize_with_mode(&self, sentence: String, mode: JsMode, enable_hmm: Option<bool>) -> Vec<JsToken> {
        self.jieba
            .tokenize(&sentence, mode.into(), enable_hmm.unwrap_or(true))
            .into_iter()
            .map(|Token { word, start, end }| JsToken {
                word: word.to_string(),
                start: start as u32,
                end: end as u32
            })
            .collect()
    }

    #[napi]
    pub fn suggest_freq(&self, segment: String) -> u32 {
        self.jieba.suggest_freq(&segment) as u32
    }
}
