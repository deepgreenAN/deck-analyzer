use deck_analyzer::all_search::{all_search_pattern, AllSearchResult};
use deck_analyzer::domain::{Card, InitialPattern};
use deck_analyzer::probability::pattern_prob;
use deck_analyzer::reader::read_json;
use indicatif::ProgressStyle;

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deck = read_json::<Vec<Card>>(&PathBuf::from("sample_deck.json"))?;
    let init_patterns =
        read_json::<Vec<InitialPattern>>(&PathBuf::from("sample_init_pattern.json"))?;

    for pat in init_patterns.iter() {
        let prob = pattern_prob(&deck, pat, 5)?;
        let InitialPattern { name, level, .. } = pat;

        println!("name: {}, level: {}, prob: {:.8}", name, level, prob);
    }

    println!("");

    println!("all search");

    println!("pattern prob");

    let AllSearchResult {
        pattern_result,
        level_result,
    } = all_search_pattern(&deck, &init_patterns, 5, ProgressStyle::default_bar())?;

    for (i, pat) in init_patterns.iter().enumerate() {
        let InitialPattern { name, level, .. } = pat;
        println!(
            "name: {}, level: {}, prob: {:.8}",
            name, level, pattern_result[i]
        );
    }

    println!("level prob");

    for i in 0..level_result.len() {
        println!("level: {}, prob: {:.8}", i, level_result[i]);
    }

    Ok(())
}
