use serde::Serialize;

#[derive(Serialize, Clone)]
#[derive(Debug)]
pub struct Word {
    pub word: String,
    pub length: usize,
}