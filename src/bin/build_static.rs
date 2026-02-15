use askama::Template;
use mycv::models::Language;
use mycv::models::TemplateTranslation;
use mycv::repositories::TomlResumeRepository;
use mycv::repositories::ResumeRepository;
use mycv::templates::CvTemplate;
use std::fs;
use std::path::Path;

fn main() {
    let repo = TomlResumeRepository::new("assets".into()).unwrap();

    let langs = [
        Language::Russian,
        Language::English,
        Language::Thai,
    ];

    for lang in langs {
        let resume = repo.get_resume(&lang).expect("Error loading resume");

        let template = CvTemplate { 
            resume,
            t: TemplateTranslation::new(&lang),
            lang: lang.to_string(),
        };

        let html = template.render().expect("Failed to render template");

        let output_path = if lang == Language::English {
            "dist/index.html".to_string()
        } else {
            format!("dist/{}/index.html", lang)
        };

        // 1️⃣ Создаём родительскую директорию
        if let Some(parent) = Path::new(&output_path).parent() {
            fs::create_dir_all(parent)
                .expect("Failed to create output directory");
        }

        // 2️⃣ Пишем файл
        fs::write(output_path, html)
            .expect("Error writing HTML file");

    }

    println!("cargo:rerun-if-changed=assets/resume.toml");
}
