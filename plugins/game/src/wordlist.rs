use bevy::ecs::system::SystemParam;
use bevy::prelude::{Reflect, Res, Resource};

// Taken from https://github.com/dwyl/english-words/blob/master/words_alpha.txt
// TODO: only compile this for server...?
pub const WORD_LIST: &str = include_str!("wordlist.txt");

#[derive(Debug)]
#[derive(Resource, Reflect)]
pub struct WordList(Vec<String>);

impl<'a> Default for WordList {
    fn default() -> Self {
        Self(
            WORD_LIST
                .lines()
                .map(|word| word.to_lowercase())
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(SystemParam)]
pub struct Dictionary<'w> {
    words: Res<'w, WordList>,
}

impl<'w> Dictionary<'w> {
    pub fn is_word_substring(&self, test_string: &str) -> bool {
        let test_string = test_string.to_lowercase();
        self.words
            .0
            .iter()
            .any(|word| word.starts_with(&test_string))
    }
}
