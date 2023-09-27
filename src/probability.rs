use crate::combinations::combination_n;
use crate::domain::{Card, InitialPattern, StringOrList};
use crate::AppError;

use std::collections::HashSet;

/// 初動パターンの確率計算
pub fn pattern_prob(deck: &Vec<Card>, pat: &InitialPattern, draw_n: u64) -> Result<f64, AppError> {
    let InitialPattern {
        first,
        second,
        third,
        ..
    } = pat;

    match (second, third) {
        // 一枚パターン
        (None, None) => single_pat_prob(deck, first, draw_n),
        // 二枚パターン
        (Some(second), None) => double_pat_prob(deck, first, second, draw_n),
        // 三枚パターン
        (Some(second), Some(third)) => triple_pat_prob(deck, first, second, third, draw_n),
        _ => Err(AppError::InvalidDataError("invalid pattern".to_string())),
    }
}

/// 1枚パターンの場合の場合の数．
fn single_pat_prob(deck: &Vec<Card>, first: &StringOrList, draw_n: u64) -> Result<f64, AppError> {
    let deck_card_n = deck
        .iter()
        .fold(0_u64, |acc, card| acc + card.number as u64);

    let first_set: HashSet<&String> = first.as_set();

    let first_n = deck
        .iter()
        .filter(|card| first_set.contains(&card.name))
        .fold(0_u64, |acc, card| acc + card.number as u64);

    let all_pattern =
        combination_n(deck_card_n, draw_n).ok_or(AppError::OverflowCombinationError)?; // カードの引き方の全パターン数
    let a_bar_pattern =
        combination_n(deck_card_n - first_n, draw_n).ok_or(AppError::OverflowCombinationError)?; // ￢Aの場合の数

    Ok((all_pattern - a_bar_pattern) as f64 / all_pattern as f64)
}

/// 2枚パターンの場合の場合の数．firstが一枚以上ありsecondが一枚以上ある集合について限定して(1枚も来なかった場合を用いて)計算しているため，
/// firstとsecondに同じカードが含まれている場合はうまく計算できない．
fn double_pat_prob(
    deck: &Vec<Card>,
    first: &StringOrList,
    second: &StringOrList,
    draw_n: u64,
) -> Result<f64, AppError> {
    let deck_card_n = deck
        .iter()
        .fold(0_u64, |acc, card| acc + card.number as u64);

    let first_set: HashSet<&String> = first.as_set();
    let second_set: HashSet<&String> = second.as_set();

    let first_n = deck
        .iter()
        .filter(|card| first_set.contains(&card.name))
        .fold(0_u64, |acc, card| acc + card.number as u64);

    let second_n = deck
        .iter()
        .filter(|card| second_set.contains(&card.name))
        .fold(0_u64, |acc, card| acc + card.number as u64);

    let all_pattern =
        combination_n(deck_card_n, draw_n).ok_or(AppError::OverflowCombinationError)?; // カードの引き方の全パターン数

    || -> Option<f64> {
        Some(
            (all_pattern
                - (combination_n(deck_card_n - first_n, draw_n)?
                    + combination_n(deck_card_n - second_n, draw_n)?
                    - combination_n(deck_card_n - (first_n + second_n), draw_n)?))
                as f64
                / all_pattern as f64,
        )
    }()
    .ok_or(AppError::OverflowCombinationError)
}

/// 3枚パターンの場合の数．firstが一枚以上ありsecondが一枚以上ありthirdが一枚以上ある集合について限定して(1枚も来なかった場合を用いて)計算しているため，
/// first, second, third内に同じカードが存在している場合はうまく計算できない．
fn triple_pat_prob(
    deck: &Vec<Card>,
    first: &StringOrList,
    second: &StringOrList,
    third: &StringOrList,
    draw_n: u64,
) -> Result<f64, AppError> {
    let deck_card_n = deck
        .iter()
        .fold(0_u64, |acc, card| acc + card.number as u64);

    let first_set: HashSet<&String> = first.as_set();
    let second_set: HashSet<&String> = second.as_set();
    let third_set: HashSet<&String> = third.as_set();

    let first_n = deck
        .iter()
        .filter(|card| first_set.contains(&card.name))
        .fold(0_u64, |acc, card| acc + card.number as u64);
    let second_n = deck
        .iter()
        .filter(|card| second_set.contains(&card.name))
        .fold(0_u64, |acc, card| acc + card.number as u64);
    let third_n = deck
        .iter()
        .filter(|card| third_set.contains(&card.name))
        .fold(0_u64, |acc, card| acc + card.number as u64);

    let all_pattern =
        combination_n(deck_card_n, draw_n).ok_or(AppError::OverflowCombinationError)?; // カードの引き方の全パターン数

    || -> Option<f64> {
        Some(
            (all_pattern
                - (combination_n(deck_card_n - first_n, draw_n)?
                    + combination_n(deck_card_n - second_n, draw_n)?
                    + combination_n(deck_card_n - third_n, draw_n)?
                    - combination_n(deck_card_n - (first_n + second_n), draw_n)?
                    - combination_n(deck_card_n - (second_n + third_n), draw_n)?
                    - combination_n(deck_card_n - (third_n + first_n), draw_n)?
                    + combination_n(deck_card_n - (first_n + second_n + third_n), draw_n)?))
                as f64
                / all_pattern as f64,
        )
    }()
    .ok_or(AppError::OverflowCombinationError)
}
