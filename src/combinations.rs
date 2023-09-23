/// nからkコ分の部分階乗つまり順列の数え上げ．
/// カードゲームの最初のドロー数はたかだか10枚以下であるため，その10枚程度掛け算を行えばよく、
/// 階乗がオーバーフローするような値だとしても順列を計算できる．
fn permutation_n(n: u64, k: u64) -> Option<u64> {
    if n < k {
        // 掛ける回数が元の数より増えてしまう場合
        None
    } else {
        match k {
            // 掛ける回数が0の場合
            0 => Some(1),
            // 掛ける回数が1の場合，そのまま返す
            1 => Some(n),
            _ => match n {
                // 0の階乗は1
                0 => Some(1),
                // 1の階乗は1
                1 => Some(1),
                // 再帰
                _ => permutation_n(n - 1, k - 1).and_then(|next| next.checked_mul(n)),
            },
        }
    }
}

/// nCkの組み合わせ．部分階乗(順列)を用いて計算する．kか(n-k)の小さい方を用いて計算する．
pub fn combination_n(n: u64, k: u64) -> Option<u64> {
    let k = std::cmp::min(k, n.checked_sub(k)?); // kかn-kの小さい方を新しくkとする．
    match (permutation_n(n, k), permutation_n(k, k)) {
        // どちらもオーバーフローしていない場合
        (Some(den), Some(num)) => den.checked_div(num),
        // どちらかがオーバーフローしている場合
        (_, _) => None,
    }
}

#[cfg(test)]
mod test {
    use super::{combination_n, permutation_n};

    #[test]
    fn test_p() {
        assert_eq!(permutation_n(40, 5), Some(78960960));
    }

    #[test]
    fn test_c() {
        assert_eq!(combination_n(40, 5), Some(658008));
        assert_eq!(combination_n(40, 35), Some(658008));
    }
}
