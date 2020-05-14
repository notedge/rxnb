use std::mem::swap;

#[derive(Clone, Debug)]
pub struct LanguageConfig {
    is_markup: bool,
    name: String,
}

#[derive(Clone, Debug)]
pub struct NotebookLanguage {
    current: LanguageConfig,
    last_markup: LanguageConfig,
    last_code: LanguageConfig,
}

impl Default for NotebookLanguage {
    fn default() -> Self {
        let notedown = LanguageConfig {
            is_markup: true,
            name: String::from("notedown"),
        };

        Self {
            current: notedown.to_owned(),
            last_markup: notedown,
            last_code: LanguageConfig {
                is_markup: false,
                name: String::from("valkyrie"),
            },
        }
    }
}


impl NotebookLanguage {
    pub fn switch(&mut self) {
        match self.current.is_markup {
            true => {
                swap(&mut self.current, &mut self.last_code);
                assert!(!self.current.is_markup)
            }
            false => {
                swap(&mut self.current, &mut self.last_markup);
                assert!(self.current.is_markup)
            }
        }
    }
}
