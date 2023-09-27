use deck_analyzer::domain::Card;
use deck_analyzer::sampling::DeckInstance;

fn main() {
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

    let mut deck_instance = DeckInstance::new(&deck);
    deck_instance.shuffle();

    println!("deck : {:?}", deck_instance.deck());
    println!("hands: {:?}", deck_instance.hands());

    deck_instance.draw(5);

    println!("deck : {:?}", deck_instance.deck());
    println!("hands: {:?}", deck_instance.hands());
}
