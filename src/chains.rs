use std::{
    fs::{self, File},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSignature {
    pub msg_id: Option<String>,
    pub author_id: Option<String>,
    pub channel_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub from_word: String,
    pub to_word: String,

    pub from_word_signature: Option<ChainSignature>,
    pub to_word_signature: Option<ChainSignature>,
}

impl Chain {
    pub fn tokenize(text: String) -> Vec<Self> {
        let mut chains: Vec<Self> = Vec::new();

        let s = text.split(" ").collect::<Vec<&str>>();

        let mut prev_word = "\\x02";

        for w in s {
            chains.push(Self {
                from_word: String::from(prev_word),
                to_word: String::from(w),
                from_word_signature: None,
                to_word_signature: None,
            });

            prev_word = w;
        }

        chains.push(Self {
            from_word: prev_word.to_string(),
            to_word: "\\x03".to_string(),
            from_word_signature: None,
            to_word_signature: None,
        });

        chains
    }
}

pub struct ChainManager {
    pub chains: Vec<Chain>,
}

impl ChainManager {
    pub fn new() -> Self {
        Self { chains: Vec::new() }
    }

    pub fn scan_text(&mut self, text: &String, text_signature: Option<ChainSignature>) {
        let tokens = Chain::tokenize(text.to_owned());

        for mut token in tokens {
            token.to_word_signature = text_signature.clone();
            token.from_word_signature = text_signature.clone();

            let mut chain: &mut Option<&mut Chain> = &mut self
                .chains
                .iter_mut()
                .find(|p| p.from_word.eq(&token.from_word));

            if chain.is_none() {
                self.chains.push(token);
            } else {
                chain.as_mut().unwrap().to_word = token.to_word;
                chain.as_mut().unwrap().to_word_signature = text_signature.clone();
            }
        }
    }

    pub fn generate_text(&self, text: &String) -> String {
        let s = text.split(" ").collect::<Vec<&str>>();
        let mut message = String::new();

        for w in s {
            let first_chain = &self.chains.iter().find(|p| p.from_word.eq(w));

            if first_chain.is_none() || first_chain.unwrap().to_word.eq("\\x03") {
                continue;
            }

            let mut next_chain: Option<&Chain> = None;

            loop {
                let mut chain: Option<&Chain> = None;

                if next_chain.is_none() {
                    message.push_str(&first_chain.unwrap().from_word.to_owned());
                    message.push_str(" ");

                    chain = self
                        .chains
                        .iter()
                        .find(|p| p.from_word.eq(&first_chain.unwrap().from_word));

                    if chain.is_none() {
                        break;
                    }

                    next_chain = chain;
                } else {
                    message.push_str(&next_chain.unwrap().from_word);
                    message.push_str(" ");

                    chain = self
                        .chains
                        .iter()
                        .find(|p| p.from_word.eq(&next_chain.unwrap().to_word));

                    if chain.is_none() {
                        break;
                    }

                    next_chain = chain;
                }
            }
        }

        message
    }

    pub fn load(&mut self, file_path: &str) -> bool {
        if !Path::new(file_path).exists() {
            println!("Chain file ({}) not exists! Nothing to load.", file_path);
            return false;
        }

        let file = File::open(file_path).unwrap();
        let mut loaded: Vec<Chain> =
            serde_json::from_reader(file).expect("JSON file with chains is not well formatted!");

        self.chains.append(&mut loaded);

        println!("LOADED {} CHAINS!", loaded.len());
        true
    }

    pub fn save(&self, file_path: &str) {
        fs::write(
            file_path,
            serde_json::to_string_pretty(&self.chains).unwrap(),
        )
        .unwrap();

        println!("SAVED!");
    }
}
