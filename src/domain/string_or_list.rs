use std::collections::HashSet;

use serde::Deserialize;

/// 文字列一つかその配列としてデシリアライズする型．
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum StringOrList {
    String(String),
    List(Vec<String>),
}

impl StringOrList {
    pub fn as_set(&self) -> HashSet<&String> {
        let mut hash_set: HashSet<&String> = HashSet::new();
        match self {
            Self::String(s) => {
                hash_set.insert(s);
            }
            Self::List(list) => {
                hash_set.extend(list);
            }
        }
        hash_set
    }
    pub fn as_vec(&self) -> Vec<&String> {
        let mut vec: Vec<&String> = Vec::new();
        match self {
            Self::String(s) => {
                vec.push(s);
            }
            Self::List(list) => {
                vec.extend(list);
            }
        }
        vec
    }
}

impl Default for StringOrList {
    fn default() -> Self {
        StringOrList::List(Vec::new())
    }
}

// impl From<StringOrList> for Vec<String> {
//     fn from(value: StringOrList) -> Self {
//         match value {
//             StringOrList::String(s) => vec![s],
//             StringOrList::List(list) => list,
//         }
//     }
// }

// impl From<StringOrList> for HashSet<String> {
//     fn from(value: StringOrList) -> Self {
//         Into::<Vec<String>>::into(value).into_iter().collect()
//     }
// }

#[cfg(test)]
mod test {
    use super::StringOrList;
    use serde::Deserialize;

    #[derive(PartialEq, Deserialize)]
    struct JsonData {
        string_or_list: StringOrList,
    }

    #[test]
    fn test_deserialize() {
        {
            let json_one_string = r#"
            {
                "string_or_list": "single string"
            }
                    "#;

            let json_data = serde_json::from_str::<JsonData>(&json_one_string).unwrap();
            assert_eq!(
                json_data.string_or_list,
                StringOrList::String("single string".to_string())
            );
        }
        {
            let json_one_string = r#"
            {
                "string_or_list": ["first", "second", "third"]
            }
                    "#;

            let json_data = serde_json::from_str::<JsonData>(&json_one_string).unwrap();
            assert_eq!(
                json_data.string_or_list,
                StringOrList::List(vec![
                    "first".to_string(),
                    "second".to_string(),
                    "third".to_string()
                ])
            );
        }
    }
}
