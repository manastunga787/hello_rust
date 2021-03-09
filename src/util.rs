pub fn find_multiples(num: i32) -> Vec<i32> {
    let mut start = num;
    let mut multiples: Vec<i32> = Vec::new();
    while start != 0 {
        if num % start == 0 {
            multiples.push(start);
        }
        start -= 1
    }
    multiples
}

pub fn get_hcf(mut a: i32, mut b: i32) -> i32 {
    if a == 0 || b == 0 {
        0
    } else {
        if a > b {
            // swap numbers
            b = a + b;
            a = b - a;
            b = b - a;
        }
        //println!("{} {}", a, b);
        loop {
            if b % a == 0 {
                break;
            } else {
                let r = b % a;
                b = a;
                a = r;
            }
        }
        a
    }
}

pub fn get_hcf_of(nums: &Vec<i32>) -> i32 {
    let mut my_nums = nums.clone();
    my_nums.sort();
    let hcf = get_hcf(my_nums[0], my_nums[1]);
    let multiples = find_multiples(hcf);

    for m in multiples {
        let mut matched = false;
        // skipping first two
        for i in 2..my_nums.len() {
            if my_nums[i] % m == 0 {
                matched = true;
            } else {
                matched = false;
                break;
            }
        }
        if matched {
            return m;
        }
    }
    1
}

#[cfg(test)]
mod testing_find_multiples {
    #[test]
    fn get_multiple_of_20() {
        let result = super::find_multiples(20);
        //println!("{:?}", result);
        assert_eq!(result.len(), 6);
        assert_eq!(result, [20, 10, 5, 4, 2, 1]);
    }
    #[test]
    fn get_multiple_of_35() {
        let result = super::find_multiples(35);
        assert_eq!(result.len(), 4);
        assert_eq!(result, [35, 7, 5, 1]);
    }
    #[test]
    fn get_multiples_of_8() {
        let result = super::find_multiples(8);
        assert_eq!(result.len(), 4);
        assert_eq!(result, [8, 4, 2, 1]);
    }
}

#[cfg(test)]

mod testing_get_hcf {
    #[test]
    fn get_hcf_8_12() {
        let result = super::get_hcf(8, 12);
        assert_eq!(result, 4);
    }

    #[test]
    fn get_hcf_12_8() {
        let result = super::get_hcf(12, 8);
        assert_eq!(result, 4);
    }

    #[test]
    fn get_hcf_5_9() {
        let result = super::get_hcf(5, 9);
        assert_eq!(result, 1);
    }
}

#[cfg(test)]
mod testing_get_hcf_of {
    #[test]
    fn get_hcf_of_30_60_45_90() {
        let data = vec![30, 60, 45, 90];
        let result = super::get_hcf_of(&data);
        assert_eq!(result, 15);
    }

    #[test]
    fn get_hcf_of_30_60_90() {
        let data = vec![30, 60, 90];
        let result = super::get_hcf_of(&data);
        assert_eq!(result, 30);
    }

    #[test]
    fn get_hcf_of_12_6_9() {
        let data = vec![12, 6, 9];
        let result = super::get_hcf_of(&data);
        assert_eq!(result, 3);
    }

    #[test]
    fn get_hcf_of_22_12_44_60() {
        let data = vec![22, 12, 44, 60];
        let result = super::get_hcf_of(&data);
        assert_eq!(result, 2);
    }
}
