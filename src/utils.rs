use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use std::{
    borrow::Cow,
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub fn unix_now() -> Result<Duration> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .with_context(|| "Invalid system time")
}

pub fn encode_uri(v: &str) -> String {
    let parts: Vec<_> = v.split('/').map(urlencoding::encode).collect();
    parts.join("/")
}

pub fn get_extension_from_filename(filename: String) -> String {
    //Change it to a canonical file path.
    let path = Path::new(&filename)
        .canonicalize()
        .expect("Expecting an existing filename");

    let filepath = path.to_str();
    let name = filepath.unwrap().split('/');
    let names: Vec<&str> = name.collect();
    let extension = names.last().expect("File extension can not be read.");
    let extens: Vec<&str> = extension.split(".").collect();

    extens[1..(extens.len())].join(".").to_string()
}

pub fn decode_uri(v: &str) -> Option<Cow<str>> {
    percent_encoding::percent_decode(v.as_bytes())
        .decode_utf8()
        .ok()
}

pub fn get_file_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|v| v.to_str())
        .unwrap_or_default()
}

#[cfg(unix)]
pub async fn get_file_mtime_and_mode(path: &Path) -> Result<(DateTime<Utc>, u16)> {
    use std::os::unix::prelude::MetadataExt;
    let meta = tokio::fs::metadata(path).await?;
    let datetime: DateTime<Utc> = meta.modified()?.into();
    Ok((datetime, meta.mode() as u16))
}

#[cfg(not(unix))]
pub async fn get_file_mtime_and_mode(path: &Path) -> Result<(DateTime<Utc>, u16)> {
    let meta = tokio::fs::metadata(&path).await?;
    let datetime: DateTime<Utc> = meta.modified()?.into();
    Ok((datetime, 0o644))
}

pub fn try_get_file_name(path: &Path) -> Result<&str> {
    path.file_name()
        .and_then(|v| v.to_str())
        .ok_or_else(|| anyhow!("Failed to get file name of `{}`", path.display()))
}

pub fn glob(pattern: &str, target: &str) -> bool {
    let pat = match ::glob::Pattern::new(pattern) {
        Ok(pat) => pat,
        Err(_) => return false,
    };
    pat.matches(target)
}

#[test]
fn test_glob_key() {
    assert!(glob("", ""));
    assert!(glob(".*", ".git"));
    assert!(glob("abc", "abc"));
    assert!(glob("a*c", "abc"));
    assert!(glob("a?c", "abc"));
    assert!(glob("a*c", "abbc"));
    assert!(glob("*c", "abc"));
    assert!(glob("a*", "abc"));
    assert!(glob("?c", "bc"));
    assert!(glob("a?", "ab"));
    assert!(!glob("abc", "adc"));
    assert!(!glob("abc", "abcd"));
    assert!(!glob("a?c", "abbc"));
    assert!(!glob("*.log", "log"));
    assert!(glob("*.abc-cba", "xyz.abc-cba"));
    assert!(glob("*.abc-cba", "123.xyz.abc-cba"));
    assert!(glob("*.log", ".log"));
    assert!(glob("*.log", "a.log"));
    assert!(glob("*/", "abc/"));
    assert!(!glob("*/", "abc"));
}
