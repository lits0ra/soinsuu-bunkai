//素因数分解
fn prime_factorization(number: u64) -> Vec<u64> {
    let mut result = number;
    for i in 2..((number as f64).sqrt() as u64)+1 {
        if number % i == 0 {
            result = i;
            break;
        }
    }

    if result == number {
        return vec![number];
    } else {
        let mut v1 = vec![result];
        let mut v2 = prime_factorization(number / result);

        v1.append(&mut v2);

        return v1;
    }
}

fn main() {
    let number = 114514;
    println!("{:?}", prime_factorization(number));
}
