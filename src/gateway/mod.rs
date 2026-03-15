//! IO layer — only this module may perform `std::fs` operations.
//!
//! All file reading and filesystem walking for RustScanners is centralised here.
//! Callers receive `String` content or `Vec<PathBuf>` — never raw IO handles.

mod reader;
mod walker;

pub use reader::{read_config, read_text};
pub use walker::{collect_project_files, find_workspace_root};
