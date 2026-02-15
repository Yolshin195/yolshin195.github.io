use mycv::models::{Language, TemplateTranslation};
use mycv::templates::{CvTemplate, HtmlTemplate};
use mycv::repositories::ResumeRepository;
use mycv::repositories::TomlResumeRepository;

use axum::{
    Router, extract::{Path, State}, response::IntoResponse, routing::get
};

#[tokio::main]
async fn main() {
    // build our application with some routes
    let app = app();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let _ = axum::serve(listener, app).await;
}

fn app() -> Router {
    let repo = TomlResumeRepository::new("assets".into()).unwrap();

    Router::new()
    .route("/", get(cv_heandler))
    .route("/{lang}", get(cv_heandler))
    .with_state(repo.clone())
}

async fn cv_heandler(
    lang: Option<Path<Language>>,
    State(repo): State<TomlResumeRepository>
) -> impl IntoResponse {
    let lang = match lang {
        Some(Path(lang)) => lang,
        None => Language::Russian,
    };

    let resume = repo.load_resume(&lang).unwrap();

    let template = CvTemplate { 
        resume,
        t: TemplateTranslation::new(&lang),
        lang: lang.to_string(),
    };

    HtmlTemplate(template)
}