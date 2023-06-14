mod config;
mod cyclo;
mod halstead;
pub mod output;
pub mod parse; // Also used by fta-wasm
mod structs;

use config::read_config;
use ignore::WalkBuilder;
use log::debug;
use log::warn;
use std::env;
use std::fs;

use globset::{Glob, GlobSetBuilder};
use ignore::DirEntry;
use structs::{FileData, FtaConfig, HalsteadMetrics};
use swc_ecma_ast::Module;
use swc_ecma_parser::error::Error;

fn is_excluded_filename(file_name: &str, patterns: &[String]) -> bool {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let glob = Glob::new(pattern).unwrap();
        builder.add(glob);
    }

    let glob_set = builder.build().unwrap();

    glob_set.is_match(file_name)
}

fn is_valid_file(repo_path: &String, entry: &DirEntry, config: &FtaConfig) -> bool {
    let file_name = entry.path().file_name().unwrap().to_str().unwrap();
    let relative_path = entry
        .path()
        .strip_prefix(repo_path)
        .unwrap()
        .to_str()
        .unwrap();

    let valid_extension = config
        .extensions
        .as_ref()
        .map_or(true, |exts| exts.iter().any(|ext| file_name.ends_with(ext)));

    let is_excluded_filename = config
        .exclude_filenames
        .as_ref()
        .map_or(false, |patterns| is_excluded_filename(file_name, patterns));

    let is_excluded_directory = config.exclude_directories.as_ref().map_or(false, |dirs| {
        dirs.iter().any(|dir| relative_path.starts_with(dir))
    });

    valid_extension && !is_excluded_filename && !is_excluded_directory
}

pub fn analyze_file(module: &Module, line_count: usize) -> (usize, HalsteadMetrics, f64) {
    let cyclo = cyclo::cyclomatic_complexity(module.clone());
    let halstead_metrics = halstead::analyze_module(module);

    let line_count_float = line_count as f64;
    let cyclo_float = cyclo as f64;
    let vocab_float = halstead_metrics.vocabulary_size as f64;

    let factor = if cyclo_float.ln() < 1.0 {
        1.0
    } else {
        line_count_float / cyclo_float.ln()
    };

    // Normalization formula based on original research
    // Originates from codehawk-cli
    let absolute_fta_score =
        171.0 - 5.2 * vocab_float.ln() - 0.23 * cyclo_float - 16.2 * factor.ln();
    let mut fta_score = 100.0 - ((absolute_fta_score * 100.0) / 171.0);

    if fta_score < 0.0 {
        fta_score = 0.0;
    }

    (cyclo, halstead_metrics, fta_score)
}

fn get_assessment(score: f64) -> String {
    if score > 60.0 {
        "(Needs improvement)".to_string()
    } else if score > 50.0 {
        "(Could be better)".to_string()
    } else {
        "OK".to_string()
    }
}

fn analyze_parsed_code(file_name: String, module: Module, line_count: usize) -> FileData {
    let (cyclo, halstead, fta_score) = analyze_file(&module, line_count);
    debug!("{} cyclo: {}, halstead: {:?}", file_name, cyclo, halstead);

    FileData {
        file_name,
        cyclo,
        halstead,
        fta_score,
        line_count,
        assessment: get_assessment(fta_score),
    }
}

fn check_score_cap_breach(
    file_name: String,
    fta_score: f64,
    score_cap: std::option::Option<usize>,
) {
    // Exit 1 if score_cap breached
    if let Some(score_cap) = score_cap {
        if fta_score > score_cap as f64 {
            eprintln!(
                "File {} has a score of {}, which is beyond the score cap of {}, exiting.",
                file_name, fta_score, score_cap
            );
            std::process::exit(1);
        }
    }
}

fn collect_results(
    entry: &DirEntry,
    repo_path: &str,
    module: Module,
    line_count: usize,
    score_cap: std::option::Option<usize>,
) -> FileData {
    // Parse the source code and run the analysis
    let file_name = entry
        .path()
        .strip_prefix(repo_path)
        .unwrap()
        .display()
        .to_string();
    let file_name_cloned = file_name.clone();
    let file_data = analyze_parsed_code(file_name, module, line_count);

    // Keep a record of the fta_score before moving the FileData
    let fta_score = file_data.fta_score;

    // Check if the score cap is breached
    check_score_cap_breach(file_name_cloned.clone(), fta_score, score_cap);

    file_data
}

fn do_analysis(
    entry: &DirEntry,
    repo_path: &str,
    config: &FtaConfig,
    source_code: &str,
    use_tsx: bool,
) -> Result<FileData, Error> {
    let (result, line_count) = parse::parse_module(source_code, use_tsx);

    match result {
        Ok(module) => Ok(collect_results(
            entry,
            repo_path,
            module,
            line_count,
            config.score_cap,
        )),
        Err(err) => Err(err),
    }
}

fn warn_about_language(file_name: &str, use_tsx: bool) {
    let tsx_name = if use_tsx { "j/tsx" } else { "non-j/tsx" };
    let opposite_tsx_name = if use_tsx { "non-j/tsx" } else { "j/tsx" };

    warn!(
        "File {} was interpreted as {} but seems to actually be {}. The file extension may be incorrect.",
        file_name,
        tsx_name,
        opposite_tsx_name
    );
}

pub fn analyze(repo_path: &String) -> Vec<FileData> {
    // Initialize the logger
    let mut builder = env_logger::Builder::new();

    // Check if debug mode is enabled using an environment variable
    if env::var("DEBUG").is_ok() {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    // Parse user config
    let config_path = format!("{}/fta.json", repo_path);
    let config = read_config(&config_path);

    let walk = WalkBuilder::new(repo_path)
        .git_ignore(true)
        .git_exclude(true)
        .standard_filters(true)
        .build();

    let mut file_data_list: Vec<FileData> = Vec::new();

    walk.filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().unwrap().is_file())
        .filter(|entry| is_valid_file(repo_path, &entry, &config))
        .for_each(|entry| {
            if file_data_list.len() >= config.output_limit.unwrap_or_default() {
                return;
            }
            let file_name = entry.path().display();
            let source_code = match fs::read_to_string(file_name.to_string()) {
                Ok(code) => code,
                Err(_) => return,
            };

            let file_extension = entry
                .path()
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default()
                .to_string();
            let use_tsx = file_extension == "tsx" || file_extension == "jsx";

            let mut file_data_result =
                do_analysis(&entry, repo_path, &config, &source_code, use_tsx);

            if file_data_result.is_err() {
                warn_about_language(&file_name.to_string(), use_tsx);
                file_data_result = do_analysis(&entry, repo_path, &config, &source_code, !use_tsx);
            }

            if file_data_result.is_err() {
                warn!(
                    "Failed to analyze {}: {:?}",
                    file_name,
                    file_data_result.unwrap_err()
                );
                return;
            }

            if let Ok(data) = file_data_result {
                file_data_list.push(data);
            }
        });

    return file_data_list;
}
