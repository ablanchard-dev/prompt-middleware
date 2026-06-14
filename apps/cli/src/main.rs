//! `prompt-optimize` — run the local prompt engine from the command line.
//!
//! Reads a raw prompt from an argument or standard input, optimizes it with the
//! pure `prompt-engine`, and prints the result as text or JSON. No network and
//! no external service: this is the same engine the browser extension uses,
//! exposed as a standalone tool.

use std::io::{self, Read, Write};
use std::process::ExitCode;

use prompt_engine::config::EngineConfig;
use shared_types::api::{OptimizeRequest, OptimizeResponse, UserPreferences};
use shared_types::domain::{DetailLevel, OptimizeMode, RequestedLanguage, TargetPlatform};

const USAGE: &str = "\
prompt-optimize - optimize a prompt locally with the Rust engine

USAGE:
    prompt-optimize [OPTIONS] [PROMPT]

If PROMPT is omitted, it is read from standard input.

OPTIONS:
    -l, --lang <fr|en|auto>                          requested language (default: auto)
    -p, --platform <chatgpt|claude|gemini|deepseek>  target platform (default: chatgpt)
    -d, --detail <short|normal|detailed|expert>      detail level (default: normal)
        --tone <TEXT>                                preferred tone
        --json                                       print the full response as JSON
    -h, --help                                       print this help

EXAMPLES:
    prompt-optimize \"corrige mon code python qui plante\"
    echo \"write unit tests\" | prompt-optimize --lang en --json
";

struct Options {
    prompt: Option<String>,
    language: RequestedLanguage,
    platform: TargetPlatform,
    detail: DetailLevel,
    tone: Option<String>,
    json: bool,
}

fn main() -> ExitCode {
    let options = match parse_args(std::env::args().skip(1)) {
        Ok(Some(options)) => options,
        Ok(None) => {
            print!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Err(message) => {
            eprintln!("error: {message}\n\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    let raw_user_input = match resolve_prompt(options.prompt) {
        Ok(input) => input,
        Err(message) => {
            eprintln!("error: {message}");
            return ExitCode::FAILURE;
        }
    };

    let request = OptimizeRequest {
        raw_user_input,
        target_platform: options.platform,
        language: options.language,
        mode: OptimizeMode::Preview,
        user_preferences: UserPreferences {
            tone: options.tone,
            detail_level: options.detail,
        },
    };

    match prompt_engine::optimize_prompt(request, &EngineConfig::default()) {
        Ok(response) if options.json => match serde_json::to_string_pretty(&response) {
            Ok(json) => {
                println!("{json}");
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("error: failed to serialize response: {error}");
                ExitCode::FAILURE
            }
        },
        Ok(response) => {
            print_human(&response);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn parse_args<I: Iterator<Item = String>>(mut args: I) -> Result<Option<Options>, String> {
    let mut prompt = None;
    let mut language = RequestedLanguage::Auto;
    let mut platform = TargetPlatform::Chatgpt;
    let mut detail = DetailLevel::Normal;
    let mut tone = None;
    let mut json = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(None),
            "--json" => json = true,
            "-l" | "--lang" => language = parse_language(&take_value(&arg, &mut args)?)?,
            "-p" | "--platform" => platform = parse_platform(&take_value(&arg, &mut args)?)?,
            "-d" | "--detail" => detail = parse_detail(&take_value(&arg, &mut args)?)?,
            "--tone" => tone = Some(take_value(&arg, &mut args)?),
            other if other.starts_with('-') && other != "-" => {
                return Err(format!("unknown option: {other}"));
            }
            _ => {
                if prompt.is_some() {
                    return Err("multiple prompts given; pass a single PROMPT".to_owned());
                }
                prompt = Some(arg);
            }
        }
    }

    Ok(Some(Options {
        prompt,
        language,
        platform,
        detail,
        tone,
        json,
    }))
}

fn take_value<I: Iterator<Item = String>>(flag: &str, args: &mut I) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("missing value for {flag}"))
}

fn resolve_prompt(prompt: Option<String>) -> Result<String, String> {
    if let Some(prompt) = prompt {
        return Ok(prompt);
    }

    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|error| format!("failed to read stdin: {error}"))?;

    let trimmed = buffer.trim();
    if trimmed.is_empty() {
        Err("no prompt given (pass it as an argument or pipe it via stdin)".to_owned())
    } else {
        Ok(trimmed.to_owned())
    }
}

fn print_human(response: &OptimizeResponse) {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let _ = writeln!(out, "{}", response.optimized_prompt);
    let _ = writeln!(
        out,
        "--- detected: {} / {} / {} (quality {:.2}) ---",
        response.detected_language,
        response.detected_domain,
        response.detected_intent,
        response.quality_score.overall
    );
    for warning in &response.warnings {
        let _ = writeln!(out, "! {warning}");
    }
    for question in &response.clarification_questions {
        let _ = writeln!(out, "? {question}");
    }
}

fn parse_language(value: &str) -> Result<RequestedLanguage, String> {
    match value.to_lowercase().as_str() {
        "fr" => Ok(RequestedLanguage::Fr),
        "en" => Ok(RequestedLanguage::En),
        "auto" => Ok(RequestedLanguage::Auto),
        other => Err(format!(
            "invalid language '{other}' (expected fr, en, or auto)"
        )),
    }
}

fn parse_platform(value: &str) -> Result<TargetPlatform, String> {
    match value.to_lowercase().as_str() {
        "chatgpt" => Ok(TargetPlatform::Chatgpt),
        "claude" => Ok(TargetPlatform::Claude),
        "gemini" => Ok(TargetPlatform::Gemini),
        "deepseek" => Ok(TargetPlatform::Deepseek),
        other => Err(format!(
            "invalid platform '{other}' (expected chatgpt, claude, gemini, or deepseek)"
        )),
    }
}

fn parse_detail(value: &str) -> Result<DetailLevel, String> {
    match value.to_lowercase().as_str() {
        "short" => Ok(DetailLevel::Short),
        "normal" => Ok(DetailLevel::Normal),
        "detailed" => Ok(DetailLevel::Detailed),
        "expert" => Ok(DetailLevel::Expert),
        other => Err(format!(
            "invalid detail level '{other}' (expected short, normal, detailed, or expert)"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Option<Options>, String> {
        parse_args(args.iter().map(|arg| (*arg).to_owned()))
    }

    #[test]
    fn parses_a_prompt_with_defaults() {
        let options = parse(&["hello world"]).unwrap().unwrap();
        assert_eq!(options.prompt.as_deref(), Some("hello world"));
        assert_eq!(options.language, RequestedLanguage::Auto);
        assert_eq!(options.platform, TargetPlatform::Chatgpt);
        assert_eq!(options.detail, DetailLevel::Normal);
        assert!(!options.json);
    }

    #[test]
    fn parses_flags_in_any_order() {
        let options = parse(&["--lang", "fr", "--json", "-d", "expert", "corrige"])
            .unwrap()
            .unwrap();
        assert_eq!(options.language, RequestedLanguage::Fr);
        assert_eq!(options.detail, DetailLevel::Expert);
        assert!(options.json);
        assert_eq!(options.prompt.as_deref(), Some("corrige"));
    }

    #[test]
    fn help_flag_requests_usage() {
        assert!(parse(&["--help"]).unwrap().is_none());
    }

    #[test]
    fn rejects_unknown_option() {
        assert!(parse(&["--nope", "x"]).is_err());
    }

    #[test]
    fn rejects_invalid_language() {
        assert!(parse(&["--lang", "es", "x"]).is_err());
    }

    #[test]
    fn rejects_missing_flag_value() {
        assert!(parse(&["--lang"]).is_err());
    }

    #[test]
    fn rejects_multiple_prompts() {
        assert!(parse(&["one", "two"]).is_err());
    }
}
