use crate::combinations::combination_n;
use crate::domain::{Card, InitialPattern};
use crate::error::AppError;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct AllSearchResult {
    pub pattern_result: Vec<f64>,
    pub level_result: Vec<f64>,
}

/// ドローパターンを全探索
pub fn all_search_pattern(
    deck: &Vec<Card>,
    patterns: &Vec<InitialPattern>,
    draw_n: u64,
    pb_style: ProgressStyle,
) -> Result<AllSearchResult, AppError> {
    let mut numbers_per_pat: Vec<u64> = vec![0; patterns.len()]; // パターンの場合の数
    let max_level =
        patterns
            .iter()
            .map(|pat| pat.level)
            .max()
            .ok_or(AppError::InvalidDataError(
                "Invalid init_patterns.".to_string(),
            ))?;
    let mut numbers_per_level: Vec<u64> = vec![0; max_level as usize + 1];

    let mut pattern_sets: Vec<(
        HashSet<&String>,
        Option<HashSet<&String>>,
        Option<HashSet<&String>>,
    )> = Vec::new(); // パターンのfirst, second, thirdをそれぞれHashSetにしたもの
    for pat in patterns.iter() {
        let InitialPattern {
            first,
            second,
            third,
            ..
        } = pat;
        match (second, third) {
            // firstのみの場合
            (None, None) => {
                pattern_sets.push((first.as_set(), None, None));
            }
            // first, secondのみの場合
            (Some(second), None) => {
                pattern_sets.push((first.as_set(), Some(second.as_set()), None));
            }
            // first, second, thirdの場合
            (Some(second), Some(third)) => {
                pattern_sets.push((first.as_set(), Some(second.as_set()), Some(third.as_set())))
            }
            (_, _) => {
                return Err(AppError::InvalidDataError(
                    "Invalid init_patters".to_string(),
                ));
            }
        }
    }

    let mut card_names: Vec<&String> = Vec::new(); // 枚数を考慮したデッキのカードの羅列
    for card in deck.iter() {
        for _ in 0..card.number {
            card_names.push(&card.name);
        }
    }
    let deck_number = card_names.len(); // デッキのカード枚数

    let all_search_combination = (0..deck_number).combinations(draw_n as usize);
    let all_pattern_number =
        combination_n(deck_number as u64, draw_n).ok_or(AppError::OverflowCombinationError)?;

    // 全探索のイテレーション
    for hands in all_search_combination
        .progress_with(ProgressBar::new(all_pattern_number).with_style(pb_style))
    {
        let hands_set = hands
            .into_iter()
            .map(|j| card_names[j])
            .collect::<HashSet<&String>>();

        let mut flags_per_level: Vec<bool> = vec![false; max_level as usize + 1];

        for (pat_i, (first_set, second_set, third_set)) in pattern_sets.iter().enumerate() {
            let level = patterns[pat_i].level;

            match (second_set, third_set) {
                // firstのみの場合
                (None, None) => {
                    if !hands_set.is_disjoint(first_set) {
                        numbers_per_pat[pat_i] += 1;
                        flags_per_level[level as usize] = true;
                    }
                }
                // first, secondのみの場合
                (Some(second_set), None) => {
                    if !hands_set.is_disjoint(first_set) && !hands_set.is_disjoint(second_set) {
                        numbers_per_pat[pat_i] += 1;
                        flags_per_level[level as usize] = true;
                    }
                }
                // first, second, thirdの場合
                (Some(second_set), Some(third_set)) => {
                    if !hands_set.is_disjoint(first_set)
                        && !hands_set.is_disjoint(second_set)
                        && !hands_set.is_disjoint(third_set)
                    {
                        numbers_per_pat[pat_i] += 1;
                        flags_per_level[level as usize] = true;
                    }
                }
                (_, _) => {}
            }
        }

        for level in flags_per_level
            .iter()
            .enumerate()
            .filter_map(|(level, flag)| flag.then_some(level))
        {
            numbers_per_level[level] += 1;
        }
    }

    Ok(AllSearchResult {
        pattern_result: numbers_per_pat
            .into_iter()
            .map(|pat_n| pat_n as f64 / all_pattern_number as f64)
            .collect(),
        level_result: numbers_per_level
            .into_iter()
            .map(|level_n| level_n as f64 / all_pattern_number as f64)
            .collect(),
    })
}
