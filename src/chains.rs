#[derive(Debug)]
pub struct ChainSignature {
    pub msg_id: &'static str,
    pub author_id: &'static str,
    pub channel_id: &'static str,
    pub timestamp: u128,
}

impl Copy for ChainSignature {}

impl Clone for ChainSignature {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug)]
pub struct Chain {
    pub id: usize,
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
                id: chains.len() + 1,
                from_word: String::from(prev_word),
                to_word: String::from(w),
                from_word_signature: None,
                to_word_signature: None,
            });

            prev_word = w;
        }

        chains.push(Self {
            id: chains.len() + 1,
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
}
