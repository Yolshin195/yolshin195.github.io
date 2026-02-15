use std::collections::HashMap;

use crate::models::{Language, Resume};

pub trait ResumeRepository {
    fn get_resume(&self, lang: &Language) -> Option<Resume>;
    fn load_resume(&self, lang: &Language) -> Result<Resume, String>;
}

#[derive(Debug, Clone)]
pub struct TomlResumeRepository {
    pub assets_path: String,
    pub resume_map: HashMap<Language, Resume>,
}

impl TomlResumeRepository {
    pub fn new(path: String) -> Result<Self, String> {
        let mut repo = TomlResumeRepository {
            assets_path: path,
            resume_map: HashMap::new(),
        };

        repo.laad_all_resumes()?; // загружаем один раз

        Ok(repo)
    }

    fn load_resume(&self, lang: &Language) -> Result<Resume, String> {
        let path = format!("{}/resume_{}.toml", self.assets_path, lang);
        let cv_toml = std::fs::read_to_string(&path)
            .map_err(|err| format!("Failed to read cv.toml: {err}"))?;
        let resume: Resume = toml::from_str(&cv_toml)
            .map_err(|err| format!("Failed to parse cv.toml: {err}"))?;

        Ok(resume)
    }

    fn laad_all_resumes(&mut self) -> Result<(), String> {
        let langs = [
            Language::Russian,
            Language::English,
            Language::Thai,
        ];

        for lang in langs {
            match self.load_resume(&lang) {
                Ok(resume) => {
                    self.resume_map.insert(lang, resume);
                },
                Err(err) => {
                    return Err(format!("Failed to load resume for language {lang}: {err}"));
                }
                
            }
        }

        Ok(())
    }
}


impl ResumeRepository for TomlResumeRepository {
    fn get_resume(&self, lang: &Language) -> Option<Resume> {
        self.resume_map.get(lang).cloned()
    }

    fn load_resume(&self, lang: &Language) -> Result<Resume, String> {
        self.load_resume(lang)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cv_toml_reader() {
        let repo = TomlResumeRepository::new("assets".into()).unwrap();
        let resume = repo.get_resume(&Language::Russian);
        
        assert!(resume.is_some());
    }
}