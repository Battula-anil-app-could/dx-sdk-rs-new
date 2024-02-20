use super::{repo_temp_download::RepoSource, template_source::template_sources};

pub async fn print_template_names() {
    let repo_temp_download = RepoSource::download_from_github(std::env::temp_dir()).await;
    let template_names = template_names_from_repo(&repo_temp_download);
    for template_name in template_names {
        println!("{template_name}");
    }
}

pub fn template_names_from_repo(repo_temp_download: &RepoSource) -> Vec<String> {
    let sources = template_sources(repo_temp_download);
    sources
        .iter()
        .map(|source| source.metadata.name.clone())
        .collect()
}