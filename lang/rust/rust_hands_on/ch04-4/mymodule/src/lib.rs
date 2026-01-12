// クレートを利用する

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod _tests1 {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// モジュールを定義する

pub mod calc {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }

    // テストを試す
    pub fn is_prime(num: usize) -> bool {
        let mut f = true;
        for n in 2..=(num / 2) {
            if num % n == 0 {
                f = false;
            }
        }
        return f;
    }
}

// テストを試す

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_prime() {
        let data = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29
        ];

        for n in data {
            let res = calc::is_prime(n);
            assert_eq!(res, true);
        }
    }

    #[test]
    fn it_isnot_prime() {
        let data = [
            4, 6, 9, 10, 12, 14, 15, 16, 18, 20
        ];

        for n in data {
            let res = calc::is_prime(n);
            assert_ne!(res, true);
        }
    }
}







