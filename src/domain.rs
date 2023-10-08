mod counter_set;
mod string_or_list;

/// データ構造をまとめておく
pub mod data_structure {
    pub use super::counter_set::CounterSet;
}

pub use string_or_list::StringOrList;

use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Card {
    pub name: String,
    pub number: u8,
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug, Hash)]
pub struct InitialPattern {
    pub name: String,
    pub first: StringOrList,
    pub second: Option<StringOrList>,
    pub third: Option<StringOrList>,
    pub level: u8,
}

#[cfg(test)]
mod test {
    use super::{Card, InitialPattern, StringOrList};

    #[test]
    fn deserialize() {
        {
            let json_str = r#"
            [
                {
                    "name": "溟界の滓－ヌル",
                    "number": 3
                },
                {
                    "name": "溟界の滓－ナイア",
                    "number": 3
                }
            ]
            "#;

            assert_eq!(
                serde_json::from_str::<Vec<Card>>(&json_str).unwrap(),
                vec![
                    Card {
                        name: "溟界の滓－ヌル".to_string(),
                        number: 3
                    },
                    Card {
                        name: "溟界の滓－ナイア".to_string(),
                        number: 3
                    }
                ]
            )
        }

        {
            let json_str = r#"
            [
                {
                    "name": "ヌル＋任意",
                    "first": "溟界の滓－ヌル",
                    "level": 1
                },
                {
                    "name": "ヌル＋任意＋陰の光",
                    "first": "溟界の滓－ヌル",
                    "second": "陰の光",
                    "level": 2
                }
            ]
            "#;

            assert_eq!(
                serde_json::from_str::<Vec<InitialPattern>>(&json_str).unwrap(),
                vec![
                    InitialPattern {
                        name: "ヌル＋任意".to_string(),
                        first: StringOrList::String("溟界の滓－ヌル".to_string()),
                        second: None,
                        third: None,
                        level: 1
                    },
                    InitialPattern {
                        name: "ヌル＋任意＋陰の光".to_string(),
                        first: StringOrList::String("溟界の滓－ヌル".to_string()),
                        second: Some(StringOrList::String("陰の光".to_string())),
                        third: None,
                        level: 2
                    }
                ]
            )
        }
    }
}
