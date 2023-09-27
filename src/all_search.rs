use crate::combinations::combination_n;
use crate::domain::{Card, InitialPattern};
use crate::error::AppError;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;

// 全探索の結果
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

    let mut patterns_vec: Vec<(Vec<&String>, Option<Vec<&String>>, Option<Vec<&String>>)> =
        Vec::new(); // パターンのfirst, second, thirdをそれぞれHashSetにしたもの
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
                patterns_vec.push((first.as_vec(), None, None));
            }
            // first, secondのみの場合
            (Some(second), None) => {
                patterns_vec.push((first.as_vec(), Some(second.as_vec()), None));
            }
            // first, second, thirdの場合
            (Some(second), Some(third)) => {
                patterns_vec.push((first.as_vec(), Some(second.as_vec()), Some(third.as_vec())))
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
        let mut hand_names = hands
            .into_iter()
            .map(|j| card_names[j])
            .collect::<Vec<&String>>();

        // 手札のカード名をソート
        hand_names.sort();

        let mut flags_per_level: Vec<bool> = vec![false; max_level as usize + 1];

        for (pat_i, (first_vec, second_vec, third_vec)) in patterns_vec.iter().enumerate() {
            let level = patterns[pat_i].level;

            match (second_vec, third_vec) {
                // firstのみの場合
                (None, None) => {
                    let mut pattern_flag = false;

                    for first_card_name in first_vec.into_iter() {
                        let pattern_names = vec![*first_card_name];
                        if lexicographical_superset(&hand_names, &pattern_names) {
                            pattern_flag = true;
                        }
                    }

                    if pattern_flag {
                        numbers_per_pat[pat_i] += 1;
                        flags_per_level[level as usize] = true;
                    }
                }
                // first, secondのみの場合
                (Some(second_vec), None) => {
                    let mut pattern_flag = false;

                    for pattern_names in vec![first_vec, second_vec]
                        .into_iter()
                        .multi_cartesian_product()
                    {
                        let mut pattern_names = pattern_names
                            .into_iter()
                            .map(|name| *name)
                            .collect::<Vec<_>>();
                        pattern_names.sort();
                        if lexicographical_superset(&hand_names, &pattern_names) {
                            pattern_flag = true;
                        }
                    }

                    if pattern_flag {
                        numbers_per_pat[pat_i] += 1;
                        flags_per_level[level as usize] = true;
                    }
                }
                // first, second, thirdの場合
                (Some(second_vec), Some(third_vec)) => {
                    let mut pattern_flag = false;

                    for pattern_names in vec![first_vec, second_vec, third_vec]
                        .into_iter()
                        .multi_cartesian_product()
                    {
                        let mut pattern_names = pattern_names
                            .into_iter()
                            .map(|name| *name)
                            .collect::<Vec<_>>();
                        pattern_names.sort();
                        if lexicographical_superset(&hand_names, &pattern_names) {
                            pattern_flag = true;
                        }
                    }

                    if pattern_flag {
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

/// ソートされた手札とその部分パターンについて，辞書式に比較して手札がスーパーセットであるかどうかを取得する
fn lexicographical_superset<T: PartialEq>(hands: &[T], pattern: &[T]) -> bool {
    let hands_iter = hands.iter();
    let mut pattern_iter = pattern.iter();

    if let Some(mut pattern_card_name) = pattern_iter.next() {
        for hand_card_name in hands_iter {
            // 手札の一枚とパターンの一枚が一致する場合
            if hand_card_name == pattern_card_name {
                match pattern_iter.next() {
                    Some(next_pattern_card_name) => {
                        pattern_card_name = next_pattern_card_name;
                    }
                    None => {
                        // pattern内のカードが全て一致したため
                        return true;
                    }
                }
            }
        }
        // trueが返る前にhands_iterのイテレーションが終了してしまった場合
        false
    } else {
        // パターンにカードが一つも入っていない場合
        true
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_lexicographical_superset() {
        let hands = vec!["A", "B", "C", "D", "E"];
        let pattern_1 = vec!["C", "D"];
        assert!(super::lexicographical_superset(&hands, &pattern_1));

        let pattern_2 = vec!["C", "F"];
        assert!(!super::lexicographical_superset(&hands, &pattern_2));

        let pattern_3 = vec!["C", "B"];
        assert!(!super::lexicographical_superset(&hands, &pattern_3));
    }
}
