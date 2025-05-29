use wasm_bindgen::prelude::*;
use js_sys::Math;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// console.log を Rust から呼び出すためのバインディング
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// パニック時のスタックトレースをより分かりやすくする
fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// 指定された範囲の乱数を生成します
#[wasm_bindgen]
pub fn random_between(min: i32, max: i32) -> i32 {
    set_panic_hook();
    
    if min >= max {
        return min;
    }
    
    let range = max - min;
    let random_value = Math::random() * range as f64;
    min + random_value as i32
}

/// 偶数かどうかを判定します
#[wasm_bindgen]
pub fn is_even(n: i32) -> bool {
    set_panic_hook();
    n % 2 == 0
}

/// 奇数かどうかを判定します
#[wasm_bindgen]
pub fn is_odd(n: i32) -> bool {
    set_panic_hook();
    n % 2 != 0
}

/// 素数かどうかを判定します
#[wasm_bindgen]
pub fn is_prime(n: i32) -> bool {
    set_panic_hook();
    
    if n < 2 {
        return false;
    }
    
    if n == 2 {
        return true;
    }
    
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as i32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

/// フィボナッチ数列のn番目の値を計算します
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
    set_panic_hook();
    
    if n <= 1 {
        return n as u64;
    }
    
    let mut a = 0u64;
    let mut b = 1u64;
    
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
}

/// 最大公約数を計算します
#[wasm_bindgen]
pub fn gcd(a: i32, b: i32) -> i32 {
    set_panic_hook();
    
    let mut a = a.abs();
    let mut b = b.abs();
    
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        assert!(!is_even(1));
        assert!(!is_even(-3));
    }

    #[test]
    fn test_is_odd() {
        assert!(is_odd(1));
        assert!(is_odd(3));
        assert!(is_odd(-5));
        assert!(!is_odd(2));
        assert!(!is_odd(0));
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(-12, 8), 4);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_random_between() {
        // 範囲のテスト（確率的なので完璧ではない）
        for _ in 0..100 {
            let result = random_between(1, 10);
            assert!(result >= 1);
            assert!(result < 10);
        }
        
        // エッジケースのテスト
        assert_eq!(random_between(5, 5), 5);
        assert_eq!(random_between(10, 5), 10);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        assert!(!is_even(1));
        assert!(!is_even(-3));
    }

    #[test]
    fn test_is_odd() {
        assert!(is_odd(1));
        assert!(is_odd(3));
        assert!(is_odd(-5));
        assert!(!is_odd(2));
        assert!(!is_odd(0));
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(-12, 8), 4);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_random_between() {
        // 範囲のテスト（確率的なので完璧ではない）
        for _ in 0..100 {
            let result = random_between(1, 10);
            assert!(result >= 1);
            assert!(result < 10);
        }
        
        // エッジケースのテスト
        assert_eq!(random_between(5, 5), 5);
        assert_eq!(random_between(10, 5), 10);
    }
}