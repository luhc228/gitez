use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
pub struct GitUrl {
  pub raw: String,
  pub host: String,
  pub group: String,
  pub project: String,
}

impl GitUrl {
  pub fn new(url: &str) -> Result<Self> {
    let with_git_suffix_re = Regex::new(r"[/@](?P<host>[^/:]+)[/:](?P<group>[^/]+)/(?P<project>[^/]+)\.git$")?;
    let with_git_suffix_caps = with_git_suffix_re.captures(url);

    if let Some(caps) = with_git_suffix_caps {
      Ok(Self {
        raw: url.to_string(),
        group: String::from(&caps["group"]),
        project: String::from(&caps["project"]),
        host: String::from(&caps["host"]),
      })
    } else {
      Err(anyhow::anyhow!("Invalid git URL."))
    }
  }
}
