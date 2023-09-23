/// 20まで計算できる
fn factorial_u64(n: u64) -> Option<u64> {
    match n {
        0 => Some(1),
        1 => Some(1),
        _ => factorial_u64(n - 1).and_then(|x| x.checked_mul(n)),
    }
}

/// 34まで計算できる
fn factorial_u128(n: u128) -> Option<u128> {
    match n {
        0 => Some(1),
        1 => Some(1),
        _ => factorial_u128(n - 1).and_then(|next| next.checked_mul(n)),
    }
}

fn main() {
    let n = 50;
    println!("n: {n}\n");

    for i in 0..n as u64 {
        if let Some(ans) = factorial_u64(i) {
            println!("i: {}, ans: {}", i, ans);
        } else {
            break;
        }
    }

    println!("\n");

    for i in 0..n as u128 {
        if let Some(ans) = factorial_u128(i) {
            println!("i: {}, ans: {}", i, ans);
        } else {
            break;
        }
    }

}
