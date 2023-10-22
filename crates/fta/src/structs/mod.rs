use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default)]
pub struct FtaConfigOptional {
    pub extensions: Option<Vec<String>>,
    pub exclude_filenames: Option<Vec<String>>,
    pub exclude_directories: Option<Vec<String>>,
    pub output_limit: Option<usize>,
    pub score_cap: Option<usize>,
    pub include_comments: Option<bool>,
    pub exclude_under: Option<usize>,
}

#[derive(Debug, Deserialize, Default)]
pub struct FtaConfigResolved {
    pub extensions: Vec<String>,
    pub exclude_filenames: Vec<String>,
    pub exclude_directories: Vec<String>,
    pub output_limit: usize,
    pub score_cap: usize,
    pub include_comments: bool,
    pub exclude_under: usize,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct HalsteadMetrics {
    pub uniq_operators: usize,  // number of unique operators
    pub uniq_operands: usize,   // number of unique operands
    pub total_operators: usize, // total number of operators
    pub total_operands: usize,  // total number of operands
    pub program_length: usize,
    pub vocabulary_size: usize,
    pub volume: f64,
    pub difficulty: f64,
    pub effort: f64,
    pub time: f64,
    pub bugs: f64,
}

#[derive(Debug, Serialize)]
pub struct FileData {
    pub file_name: String,
    pub cyclo: usize,
    pub halstead: HalsteadMetrics,
    pub line_count: usize,
    pub fta_score: f64,
    pub assessment: String,
}
