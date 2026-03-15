use serde::Deserialize;

const DEFAULT_CHILD_WARN_AT: usize = 100;
const DEFAULT_CHILD_ERROR_AT: usize = 150;

/// Project topology — controls which files the scanner collects.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Topology {
    /// Single crate: scan own `src/` only. Default.
    #[default]
    Flat,
    /// Workspace: scan all `apps/*/src/` and `crates/*/src/` from workspace root.
    Workspace,
}

const TOPOLOGY_WORKSPACE: &str = "workspace";

impl Topology {
    fn from_str(s: &str) -> Self {
        if s.trim().to_lowercase() == TOPOLOGY_WORKSPACE { Self::Workspace } else { Self::Flat }
    }
}

/// Scanner configuration — loaded from `proj/rulestools.toml`.
#[derive(Debug, Clone)]
pub struct Config {
    pub enabled: bool,
    pub deny: bool,
    pub check_magic_numbers: bool,
    pub check_hardcoded_durations: bool,
    pub check_string_states: bool,
    pub check_unwrap_panic: bool,
    pub check_unsafe_no_comment: bool,
    pub check_doc_comments: bool,
    pub check_child_module_size: bool,
    pub child_module_warn_at: usize,
    pub child_module_error_at: usize,
    pub check_shared_guard: bool,
    pub check_sibling_import: bool,
    pub check_duplicate_pub_fn: bool,
    pub exclude: Vec<String>,
    pub topology: Topology,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            deny: false,
            check_magic_numbers: true,
            check_hardcoded_durations: true,
            check_string_states: true,
            check_unwrap_panic: true,
            check_unsafe_no_comment: true,
            check_doc_comments: true,
            check_child_module_size: true,
            child_module_warn_at: DEFAULT_CHILD_WARN_AT,
            child_module_error_at: DEFAULT_CHILD_ERROR_AT,
            check_shared_guard: true,
            check_sibling_import: true,
            check_duplicate_pub_fn: true,
            exclude: Vec::new(),
            topology: Topology::Flat,
        }
    }
}

#[derive(Deserialize, Default)]
struct TomlRoot {
    #[serde(default)]
    rustscanners: Option<TomlScanners>,
    #[serde(default)]
    project: Option<TomlProject>,
}

#[derive(Deserialize, Default)]
struct TomlProject {
    topology: Option<String>,
}

#[derive(Deserialize, Default)]
struct TomlScanners {
    enabled: Option<bool>,
    deny: Option<bool>,
    magic_numbers: Option<bool>,
    hardcoded_durations: Option<bool>,
    string_states: Option<bool>,
    unwrap_panic: Option<bool>,
    unsafe_no_comment: Option<bool>,
    doc_comments: Option<bool>,
    child_module_size: Option<bool>,
    child_module_warn_at: Option<usize>,
    child_module_error_at: Option<usize>,
    shared_guard: Option<bool>,
    sibling_import: Option<bool>,
    duplicate_pub_fn: Option<bool>,
    exclude: Option<Vec<String>>,
}

impl Config {
    /// Parse configuration from the content of `proj/rulestools.toml`.
    /// Pass `None` (or `Some("")`) to use defaults.
    pub fn from_content(content: Option<&str>) -> Self {
        let mut cfg = Self::default();

        let text = match content {
            Some(t) if !t.is_empty() => t,
            _ => return cfg,
        };

        let Ok(parsed) = toml::from_str::<TomlRoot>(text) else {
            return cfg;
        };

        let Some(s) = parsed.rustscanners else {
            return cfg;
        };

        if let Some(v) = s.enabled            { cfg.enabled = v; }
        if let Some(v) = s.deny               { cfg.deny = v; }
        if let Some(v) = s.magic_numbers       { cfg.check_magic_numbers = v; }
        if let Some(v) = s.hardcoded_durations { cfg.check_hardcoded_durations = v; }
        if let Some(v) = s.string_states       { cfg.check_string_states = v; }
        if let Some(v) = s.unwrap_panic        { cfg.check_unwrap_panic = v; }
        if let Some(v) = s.unsafe_no_comment   { cfg.check_unsafe_no_comment = v; }
        if let Some(v) = s.doc_comments        { cfg.check_doc_comments = v; }
        if let Some(v) = s.child_module_size   { cfg.check_child_module_size = v; }
        if let Some(v) = s.child_module_warn_at { cfg.child_module_warn_at = v; }
        if let Some(v) = s.child_module_error_at { cfg.child_module_error_at = v; }
        if let Some(v) = s.shared_guard        { cfg.check_shared_guard = v; }
        if let Some(v) = s.sibling_import      { cfg.check_sibling_import = v; }
        if let Some(v) = s.duplicate_pub_fn    { cfg.check_duplicate_pub_fn = v; }
        if let Some(v) = s.exclude             { cfg.exclude = v; }

        if let Some(p) = parsed.project {
            if let Some(t) = p.topology { cfg.topology = Topology::from_str(&t); }
        }

        cfg
    }
}
