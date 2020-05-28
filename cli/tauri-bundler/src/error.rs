// error_chain! {
//     foreign_links {
//           Glob(::glob::GlobError);
//           GlobPattern(::glob::PatternError);
//           Io(::std::io::Error);
//           Image(::image::ImageError);
//           Target(::target_build_utils::Error);
//           Term(::term::Error);
//           Toml(::toml::de::Error);
//           Walkdir(::walkdir::Error);
//           StripError(std::path::StripPrefixError);
//           ConvertError(std::num::TryFromIntError);
//           RegexError(::regex::Error) #[cfg(windows)];
//           HttpError(::attohttpc::Error) #[cfg(windows)];
//           Json(::serde_json::error::Error);
//       }
//       errors {}
//   }

use thiserror::Error as DeriveError;

#[derive(Debug, DeriveError)]
pub enum Error {
  #[error("{0}")]
  BundlerError(#[from] anyhow::Error),
  #[error("`{0}`")]
  GlobError(#[from] ::glob::GlobError),
  #[error("`{0}`")]
  GlobPatternError(#[from] ::glob::PatternError),
  #[error("`{0}`")]
  IoError(#[from] std::io::Error),
  #[error("`{0}`")]
  ImageError(#[from] ::image::ImageError),
  #[error("`{0}`")]
  TargetError(#[from] ::target_build_utils::Error),
  #[error("`{0}`")]
  TermError(#[from] ::term::Error),
  #[error("`{0}`")]
  TomlError(#[from] ::toml::de::Error),
  #[error("`{0}`")]
  WalkdirError(#[from] ::walkdir::Error),
  #[error("`{0}`")]
  StripError(#[from] std::path::StripPrefixError),
  #[error("`{0}`")]
  ConvertError(#[from] std::num::TryFromIntError),

  #[error("`{0}`")]
  ZipError(#[from] ::zip::result::ZipError),
  #[error("`{0}`")]
  HexError(#[from] ::hex::FromHexError),
  #[error("`{0}`")]
  HandleBarsError(#[from] ::handlebars::RenderError),
  #[error("`{0}`")]
  JsonError(#[from] ::serde_json::error::Error),
  #[cfg(windows)]
  #[error("`{0}`")]
  RegexError(#[from] ::regex::Error),
  #[cfg(windows)]
  #[error("`{0}`")]
  HttpError(#[from] ::attohttpc::Error),
  #[error("hash mismatch of downloaded file")]
  HashError,
  #[error("Architecture Error: `{0}`")]
  ArchError(String),
  #[error("Error running Candle.exe")]
  CandleError,
  #[error("Error running Light.exe")]
  LightError,
  #[error(
    "Couldn't get tauri config; please specify the TAURI_CONFIG or TAURI_DIR environment variables"
  )]
  EnvironmentError,
  #[error("Path Error:`{0}`")]
  PathUtilError(String),
  #[error("Shell Scripting Error:`{0}`")]
  ShellScriptError(String),
}

pub type Result<T> = anyhow::Result<T, Error>;