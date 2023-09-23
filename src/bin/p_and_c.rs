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
fn combination_n(n: u64, k: u64) -> Option<u64> {
    let k = std::cmp::min(k, n.checked_sub(k)?); // kかn-kの小さい方を新しくkとする．
    match (permutation_n(n, k), permutation_n(k, k)) {
        // どちらもオーバーフローしていない場合
        (Some(den), Some(num)) => den.checked_div(num),
        // どちらかがオーバーフローしている場合
        (_, _) => None,
    }
}

fn main() {
    println!("5 * 4 * 3, 5P3, ans = {:?}", permutation_n(5, 3));
    println!("5 * 4 * 3 * 2 * 1, 5P5, ans = {:?}", permutation_n(5, 5));
    println!("5, ans = {:?}", permutation_n(5, 1));

    0_u64;
    println!("{:?}", 5_u64.checked_sub(5_u64));

    println!(
        "5C3={:?}, 5C2={:?}",
        combination_n(5, 3),
        combination_n(5, 2)
    );

    println!(
        "5C5={:?}, 5C1={:?}",
        combination_n(5, 5),
        combination_n(5, 1)
    );

    println!("40C5={:?}", combination_n(40, 5));
    println!("60C6={:?}", combination_n(60, 6));

    // 40枚のデッキから5枚引いた場合に3枚入れたカードを引く確率
    let p = || -> Option<f64> {
        let n = combination_n(3, 1)? * combination_n(37, 4)?
            + combination_n(3, 2)? * combination_n(37, 3)?
            + combination_n(3, 3)? * combination_n(37, 2)?;

        Some(n as f64 / combination_n(40, 5)? as f64)
    }();

    println!("3枚入れた場合 p = {:?}", p);

    // 40枚のデッキから5枚引いた場合に2枚入れたカードを引く確率
    let p = || -> Option<f64> {
        let n = combination_n(2, 1)? * combination_n(38, 4)?
            + combination_n(2, 2)? * combination_n(38, 3)?;

        Some(n as f64 / combination_n(40, 5)? as f64)
    }();

    println!("2枚入れた場合 p = {:?}", p);

    // 40枚のデッキから5枚引いた場合に1枚入れたカードを引く確率
    let p = || -> Option<f64> {
        let n = combination_n(1, 1)? * combination_n(39, 4)?;

        Some(n as f64 / combination_n(40, 5)? as f64)
    }();

    println!("1枚入れた場合 p = {:?}", p);
}
