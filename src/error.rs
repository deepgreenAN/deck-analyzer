/// アプリケーション全体のエラー
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// jsonのデシリアライズに関するエラー
    #[error(transparent)]
    DeserializeError(#[from] serde_json::Error),
    /// jsonのデータが有効でない場合のエラー
    #[error("AppError::InvalidDataError: {0}")]
    InvalidDataError(String),
    /// 組み合わせ計算がオーバーフローした場合のエラー
    #[error("AppError::OverflowCombinationError: overflowed combination.")]
    OverflowCombinationError,
    /// ファイルの読み込みエラー
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
