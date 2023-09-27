use crate::domain::Card;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct DeckInstance {
    /// デッキの内容を表すVec．ドローの構造上一番上のカードが最後の要素となる．
    deck_names: Vec<String>,
    hands_names: Vec<String>,
}

impl DeckInstance {
    /// O(a * n) (aはカード枚数上限)
    pub fn new(deck: &[Card]) -> Self {
        let mut deck_names = Vec::new();

        for card in deck.iter() {
            for _ in 0..card.number {
                deck_names.push(card.name.to_string());
            }
        }

        DeckInstance {
            deck_names,
            hands_names: Vec::new(),
        }
    }

    /// デッキをシャッフルする．O(n)．
    pub fn shuffle(&mut self) {
        self.deck_names.shuffle(&mut thread_rng());
    }

    /// `draw_n`分ドローする．O(draw_n)．
    pub fn draw(&mut self, draw_n: usize) {
        for _ in 0..draw_n {
            let deck_length = self.deck_names.len();
            let draw_card = self.deck_names.swap_remove(deck_length - 1);
            self.hands_names.push(draw_card);
        }
    }
    pub fn deck(&self) -> &[String] {
        &self.deck_names
    }
    pub fn hands(&self) -> &[String] {
        &self.hands_names
    }
}

#[cfg(test)]
mod test {
    use super::DeckInstance;
    use crate::domain::Card;

    use std::collections::HashSet;

    #[test]
    fn test_deck_instance() {
        let deck = vec![
            Card {
                name: "強欲な壺".to_string(),
                number: 3,
            },
            Card {
                name: "ブラック・マジシャン".to_string(),
                number: 3,
            },
            Card {
                name: "幻獣王ガゼル".to_string(),
                number: 3,
            },
            Card {
                name: "暗黒騎士ガイア".to_string(),
                number: 3,
            },
            Card {
                name: "エルフの剣士".to_string(),
                number: 3,
            },
        ];

        let mut card_name_pool = HashSet::new();
        card_name_pool.insert("強欲な壺".to_string());
        card_name_pool.insert("ブラック・マジシャン".to_string());
        card_name_pool.insert("幻獣王ガゼル".to_string());
        card_name_pool.insert("暗黒騎士ガイア".to_string());
        card_name_pool.insert("エルフの剣士".to_string());

        let mut deck_instance = DeckInstance::new(&deck);
        deck_instance.shuffle();

        assert_eq!(deck_instance.deck().len(), 15);
        assert_eq!(deck_instance.hands().len(), 0);

        deck_instance.draw(5);
        assert_eq!(deck_instance.deck().len(), 10);
        assert_eq!(deck_instance.hands().len(), 5);

        for card_name in deck_instance.deck() {
            assert!(card_name_pool.contains(card_name));
        }
    }
}
