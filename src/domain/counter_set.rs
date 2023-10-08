use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
};

/// カウンタ―を持ったHashSet
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CounterSet<T: Eq + Hash> {
    /// 内部のSetを意味するマップ
    map: HashMap<T, usize>,
    /// 全ての値のカウンタの合計
    length: usize,
}

impl<T: Eq + Hash> CounterSet<T> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            length: 0,
        }
    }
    /// 値を挿入しその値の個数を返す．O(1)
    pub fn insert(&mut self, value: T) -> usize {
        match self.map.entry(value) {
            Entry::Occupied(mut o) => {
                *o.get_mut() += 1;
                self.length += 1;

                o.get() + 1
            }
            Entry::Vacant(v) => {
                v.insert(1);
                self.length += 1;

                0
            }
        }
    }
    /// 値を削除しその値の個数を返す．O(1)
    pub fn remove(&mut self, value: &T) -> usize {
        if let Some(v) = self.map.get(value) {
            match *v {
                v_a if v_a > 1 => {
                    *self.map.get_mut(value).unwrap() -= 1;
                    self.length -= 1;

                    v_a - 1
                }
                v_a if v_a <= 1 => {
                    self.map.remove(value);
                    self.length -= 1;

                    0
                }
                _ => unreachable!(),
            }
        } else {
            0
        }
    }
    /// 重複を含む長さを取得する．
    pub fn len(&self) -> usize {
        self.length
    }
    /// 含む数を取得する．O(1)
    pub fn contains_n(&self, value: &T) -> usize {
        match self.map.get(value) {
            Some(n) => *n,
            None => 0,
        }
    }
    /// 重複があるかどうかを返す．O(1)
    pub fn is_duplicate(&self) -> bool {
        self.length != self.map.len()
    }

    /// スーパーセットであるかどうか．O(n). otherの全ての値を含んでいて，その個数は大きい．
    pub fn is_superset(&self, other: &Self) -> bool {
        self.length >= other.length
            && other
                .map
                .iter()
                .all(|(value, i)| &self.contains_n(value) >= i)
    }
}

impl<V: Eq + Hash> FromIterator<V> for CounterSet<V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut counter_set = CounterSet::new();
        for value in iter {
            counter_set.insert(value);
        }
        counter_set
    }
}

#[cfg(test)]
mod test {
    use super::CounterSet;

    #[test]
    fn test_counter_set_base() {
        let mut counter_set = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "A".to_string(),
        ]
        .into_iter()
        .collect::<CounterSet<String>>();

        assert_eq!(counter_set.contains_n(&"A".to_string()), 2);
        assert_eq!(counter_set.contains_n(&"B".to_string()), 1);
        assert_eq!(counter_set.len(), 4);
        assert!(counter_set.is_duplicate());

        counter_set.remove(&"A".to_string());
        counter_set.remove(&"B".to_string());

        assert_eq!(counter_set.contains_n(&"A".to_string()), 1);
        assert_eq!(counter_set.contains_n(&"B".to_string()), 0);
        assert_eq!(counter_set.len(), 2);
        assert!(!counter_set.is_duplicate());
    }

    #[test]
    fn test_counter_set_superset() {
        let counter_set = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "A".to_string(),
        ]
        .into_iter()
        .collect::<CounterSet<String>>();

        let subset = vec!["A".to_string(), "B".to_string()]
            .into_iter()
            .collect::<CounterSet<String>>();

        assert!(counter_set.is_superset(&subset));

        let not_subset_1 = vec!["A".to_string(), "D".to_string()]
            .into_iter()
            .collect::<CounterSet<String>>();

        assert!(!counter_set.is_superset(&not_subset_1));

        let not_subset_2 = vec!["A".to_string(), "A".to_string(), "A".to_string()]
            .into_iter()
            .collect::<CounterSet<String>>();

        assert!(!counter_set.is_superset(&not_subset_2));
    }
}
