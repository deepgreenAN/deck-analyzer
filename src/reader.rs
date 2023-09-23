use crate::error::AppError;

use serde::de::DeserializeOwned;
use std::fs::File;
use std::path::Path;

/// jsonのジェネリックなリーダー
pub fn read_json<T: DeserializeOwned>(file_name: &Path) -> Result<T, AppError> {
    let file = File::open(file_name)?;

    Ok(serde_json::from_reader(file)?)
}
