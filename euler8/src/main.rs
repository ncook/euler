fn main() {
    let thousand_digits = "\
        73167176531330624919225119674426574742355349194934\
        96983520312774506326239578318016984801869478851843\
        85861560789112949495459501737958331952853208805511\
        12540698747158523863050715693290963295227443043557\
        66896648950445244523161731856403098711121722383113\
        62229893423380308135336276614282806444486645238749\
        30358907296290491560440772390713810515859307960866\
        70172427121883998797908792274921901699720888093776\
        65727333001053367881220235421809751254540594752243\
        52584907711670556013604839586446706324415722155397\
        53697817977846174064955149290862569321978468622482\
        83972241375657056057490261407972968652414535100474\
        82166370484403199890008895243450658541227588666881\
        16427171479924442928230863465674813919123162824586\
        17866458359124566529476545682848912883142607690042\
        24219022671055626321111109370544217506941658960408\
        07198403850962455444362981230987879927244284909188\
        84580156166097919133875499200524063689912560717606\
        05886116467109405077541002256983155200055935729725\
        71636269561882670428252483600823257530420752963450\
    ";
    let product = max_product_of(thousand_digits, 13);
    println!("Product of 13 adjacent digits is {}", product);
}

fn max_product_of(digits: &str, ndigits: usize) -> u64 {
    let mut iter = digits.chars().into_iter();
    let mut digit_count = 0;
    let mut combo_ptr: usize = 0;
    let mut combo = vec![0; ndigits];
    let mut max_product = 0;
    while let Some(digit) = iter.next() {
        add_digit(&mut combo, &mut combo_ptr, digit, ndigits);
        digit_count += 1;
        if digit_count >= ndigits {
           let product = combo.iter().product();
           println!("product: {}", product);
           max_product = u64::max(product, max_product);
       }
   }
   max_product
}

fn add_digit (arr: &mut [u64], ptr: &mut usize, digit: char, ndigits: usize) {
    if *ptr >= ndigits {
        *ptr = 0;
    }
    let r = digit.to_digit(10).expect("Digits only!") as u64;
    arr[*ptr] = r;
    *ptr += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_of() {
        let x = "2343665";
        assert_eq!(max_product_of(x, 1), 6);
        assert_eq!(max_product_of(x, 2), 36);
    }
}
