use serde::Deserialize;

#[derive(Deserialize)]
pub struct Engines {
  pub node: Option<String>,
  pub npm: Option<String>
}

#[derive(Deserialize)]
pub struct PackageJson {
  pub engines: Option<Engines>
}
