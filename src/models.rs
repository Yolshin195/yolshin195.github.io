use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Language {
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "th")]
    Thai,
}

impl Default for Language {
    fn default() -> Self {
        Language::Russian
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Language::Russian => "ru",
            Language::Thai => "th",
            Language::English => "en",
        };

        write!(f, "{s}")
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ru" => Ok(Language::Russian),
            "th" => Ok(Language::Thai),
            _ => Ok(Language::English),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTemplate {
    #[serde(default)]
    pub lang: Language,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateTranslation {
    pub summary: &'static str,
    pub technical_skills: &'static str,
    pub languages: &'static str,
    pub backend: &'static str,
    pub concurrency: &'static str,
    pub databases: &'static str,
    pub devops: &'static str,
    pub testing: &'static str,
    pub work_experience: &'static str,
    pub projects: &'static str,
    pub education: &'static str,
    pub additional: &'static str,
    pub present: &'static str,
    pub view_project: &'static str,
    pub english_level: &'static str,
    pub relocation: &'static str,
    pub interests: &'static str,
    pub yes: &'static str,
    pub no: &'static str,
}

impl TemplateTranslation {
    pub fn new(lang: &Language) -> Self {
        match lang {
            Language::Russian => Self::russian(),
            Language::Thai => Self::thai(),
            _ => Self::english(),
        }
    }

    fn english() -> Self {
        Self {
            summary: "Professional Summary",
            technical_skills: "Technical Skills",
            languages: "Languages",
            backend: "Backend",
            concurrency: "Concurrency",
            databases: "Databases",
            devops: "DevOps & Tools",
            testing: "Testing",
            work_experience: "Work Experience",
            projects: "Projects",
            education: "Education",
            additional: "Additional Information",
            present: "Present",
            view_project: "View Project",
            english_level: "English Level",
            relocation: "Relocation",
            interests: "Interests",
            yes: "Yes",
            no: "No",
        }
    }

    fn russian() -> Self {
        Self {
            summary: "О себе",
            technical_skills: "Технические навыки",
            languages: "Языки программирования",
            backend: "Backend",
            concurrency: "Многопоточность",
            databases: "Базы данных",
            devops: "DevOps и инструменты",
            testing: "Тестирование",
            work_experience: "Опыт работы",
            projects: "Проекты",
            education: "Образование",
            additional: "Дополнительная информация",
            present: "По настоящее время",
            view_project: "Посмотреть проект",
            english_level: "Уровень английского",
            relocation: "Релокация",
            interests: "Интересы",
            yes: "Да",
            no: "Нет",
        }
    }

    fn thai() -> Self {
        Self {
            summary: "ข้อมูลส่วนตัว",
            technical_skills: "ทักษะทางเทคนิค",
            languages: "ภาษาโปรแกรม",
            backend: "Backend",
            concurrency: "Concurrency",
            databases: "ฐานข้อมูล",
            devops: "DevOps และเครื่องมือ",
            testing: "การทดสอบ",
            work_experience: "ประสบการณ์การทำงาน",
            projects: "โครงการ",
            education: "การศึกษา",
            additional: "ข้อมูลเพิ่มเติม",
            present: "ปัจจุบัน",
            view_project: "ดูโครงการ",
            english_level: "ระดับภาษาอังกฤษ",
            relocation: "การย้ายที่อยู่",
            interests: "ความสนใจ",
            yes: "ใช่",
            no: "ไม่",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub personal_info: PersonalInfo,
    pub summary: String,
    pub technical_skills: TechnicalSkills,
    pub work_experience: Vec<WorkExperience>,
    #[serde(default)]
    pub projects: Vec<Project>,
    pub education: Vec<Education>,
    pub additional: Option<Additional>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub full_name: String,
    pub title: String,
    pub location: String,
    pub email: String,
    pub telegram: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalSkills {
    pub category_skills: Vec<TechnicalSkillsCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalSkillsCategory {
    pub name: String,
    pub skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExperience {
    pub company: String,
    pub position: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub responsibilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub stack: Vec<String>,
    pub highlights: Vec<String>,
    pub github_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub degree: String,
    pub institution: String,
    pub start_year: u16,
    pub end_year: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Additional {
    pub english_level: Option<String>,
    pub relocation_ready: Option<bool>,
    pub interests: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let resume = Resume {
            personal_info: PersonalInfo {
                full_name: "Aleksey Dev".to_string(),
                title: "Rust Backend Developer".to_string(),
                location: "Bangkok, Thailand".to_string(),
                email: "aleksey@example.com".to_string(),
                telegram: Some("@alekseydev".to_string()),
                github: Some("https://github.com/alekseydev".to_string()),
                linkedin: None,
            },
            summary: "Rust backend developer with experience in Axum and gRPC (tonic).".to_string(),
            technical_skills: TechnicalSkills {
                category_skills: vec![
                    TechnicalSkillsCategory {
                        name: "Languages".into(),
                        skills: vec!["Rust".into(), "SQL".into()],
                    },
                    TechnicalSkillsCategory {
                        name: "Backend".into(),
                        skills: vec!["Axum".into(), "Tonic".into(), "REST".into()],
                    },
                    TechnicalSkillsCategory {
                        name: "Concurrency".into(),
                        skills: vec!["Tokio".into(), "Arc".into(), "Mutex".into()],
                    },
                    TechnicalSkillsCategory {
                        name: "Databases".into(),
                        skills: vec!["PostgreSQL".into()],
                    },
                    TechnicalSkillsCategory {
                        name: "DevOps".into(),
                        skills: vec!["Docker".into(), "CI/CD".into()],
                    },
                    TechnicalSkillsCategory {
                        name: "Testing".into(),
                        skills: vec!["cargo test".into(), "integration tests".into()],
                    },
                ],
            },
            work_experience: vec![WorkExperience {
                company: "Tech Company".into(),
                position: "Rust Backend Developer".into(),
                start_date: "2024-01".into(),
                end_date: None,
                responsibilities: vec![
                    "Developed gRPC services using tonic".into(),
                    "Optimized Docker images (2GB -> 150MB)".into(),
                ],
            }],
            projects: vec![Project {
                name: "Accounting Category Service".into(),
                description: "gRPC CRUD service for category management".into(),
                stack: vec![
                    "Rust".into(),
                    "Tonic".into(),
                    "PostgreSQL".into(),
                    "Docker".into(),
                ],
                highlights: vec![
                    "Thread-safe state with Arc<Mutex<_>>".into(),
                    "Multi-stage Docker build".into(),
                ],
                github_url: Some("https://github.com/alekseydev/accounting-category".into()),
            }],
            education: vec![Education {
                degree: "Bachelor of Computer Science".into(),
                institution: "University Name".into(),
                start_year: 2018,
                end_year: Some(2022),
            }],
            additional: Some(Additional {
                english_level: Some("B2".into()),
                relocation_ready: Some(true),
                interests: Some(vec!["High-load systems".into(), "Microservices".into()]),
            }),
        };

        // сериализация в JSON
        let json = serde_json::to_string_pretty(&resume).unwrap();

        println!("{}", json);
    }
}
