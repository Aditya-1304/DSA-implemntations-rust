///        PSEUDOCODE FOR KARATSUBA ALGO
/// 
///  This algo is generally used for Multiplication of very huge integers 
/// 
///  Input: two n-digit positive integers x and y.
///  Output: the product x * y.
///  Assumption: n is a power of 2.
/// ------------------------------------------------
/// 
///  if n = 1 then             // base case
///  compute x * y in one step and return the result
///  else                     // recursive case
///  a, b := first and second halves of x
///  c, d := first and second halves of y
///  compute p := a + b and q := c + d using normal addition
///  recursively compute ac := a * c, bd := b * d, and
///  pq := p * q
///  compute adbc := pq - ac - bd using normal addition
///  compute 10n * ac + 10n/2 * adbc + bd using normal addition and return the result


use std::cmp::max;

fn main() {
    let x = vec![1,2,3,4];
    let y = vec![5,6,7,8];

    let result = karatsuba(&x, &y);
    println!("1234 * 5678 = {}", vec_to_number(&result));

    let x2 = vec![9,9];
    let y2 = vec![9,9];
    let result2 = karatsuba(&x2, &y2);
    println!("99 * 99 = {}", vec_to_number(&result2));
}

fn karatsuba(x: &Vec<u32>, y: &Vec<u32>) -> Vec<u32> {
    let max_len = max(x.len(), y.len());
    let n = next_power_of_2(max_len);

    let x_padded = pad_to_length(x, n);
    let y_padded = pad_to_length(y, n);

    karatsuba_internal(&x_padded, &y_padded)
}

fn karatsuba_internal(x: &Vec<u32>, y: &Vec<u32>) -> Vec<u32> {
    let n = x.len();

    if n == 1 {
        let product = x[0] * y[0];
        return number_to_vec(product as u64);
    }

    let half = n / 2;
    let a = x[0..half].to_vec();
    let b = x[half..n].to_vec();
    let c = y[0..half].to_vec();
    let d = y[half..n].to_vec();

    let p = add_numbers(&a, &b);
    let q = add_numbers(&c, &d);

    let max_pq_len = max(p.len(), q.len());
    let pq_len = next_power_of_2(max_pq_len);
    let p_padded = pad_to_length(&p, pq_len);
    let q_padded = pad_to_length(&q, pq_len);

    let ac = karatsuba_internal(&a, &c);
    let bd = karatsuba_internal(&b, &d);
    let pq = karatsuba_internal(&p_padded, &q_padded);

    let temp = add_numbers(&ac, &bd);
    let adbc = subtract_numbers(&pq, &temp);

    let term1 = multiply_by_power_of_10(&ac, n);
    let term2 = multiply_by_power_of_10(&adbc, half);
    let result = add_numbers(&add_numbers(&term1, &term2), &bd);

    remove_leading_zeros(result)
}

fn add_numbers(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    let max_len = max(a.len(), b.len());
    let mut result = Vec::new();
    let mut carry = 0;
    
    for i in 0..max_len {
        let digit_a = if i < a.len() { a[a.len() - 1 - i] } else { 0 };
        let digit_b = if i < b.len() { b[b.len() - 1 - i] } else { 0 };
        
        let sum = digit_a + digit_b + carry;
        result.push(sum % 10);
        carry = sum / 10;
    }
    
    if carry > 0 {
        result.push(carry);
    }
    
    result.reverse();
    remove_leading_zeros(result)
}
fn subtract_numbers(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    if is_smaller(a, b) {
        return vec![0];
    }

    let mut result = Vec::new();
    let mut borrow = 0;

    let max_len = max(a.len(), b.len());

    for i in 0..max_len {
        let digit_a = if i < a.len() { a[a.len() - 1 - i] } else { 0 };
        let digit_b = if i < b.len() { b[b.len() - 1 - i] } else { 0 };

        let mut diff = digit_a as i32 - digit_b as i32 - borrow;

        if diff < 0 {
            diff += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        result.push(diff as u32);
    }

    result.reverse();
    remove_leading_zeros(result)
}

fn is_smaller(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    let a_clean = remove_leading_zeros(a.clone());
    let b_clean = remove_leading_zeros(b.clone());

    if a_clean.len() < b_clean.len() {
        return true;
    }
    if a_clean.len() > b_clean.len() {
        return false;
    }

    for i in 0..a_clean.len() {
        if a_clean[i] < b_clean[i] {
            return true;
        }
        if a_clean[i] > b_clean[i] {
            return false;
        }
    }
    false
}

fn multiply_by_power_of_10(num: &Vec<u32>, n: usize) -> Vec<u32> {
    if num.len() == 1 && num[0] == 0 {
        return vec![0];
    }
    
    let mut result = num.clone();
    for _ in 0..n {
        result.push(0);
    }
    result
}

fn pad_to_length(num: &Vec<u32>, length: usize) -> Vec<u32> {
    if num.len() >= length {
        return num.clone();
    }
    
    let mut padded = vec![0; length - num.len()];
    padded.extend_from_slice(num);
    padded
}

fn next_power_of_2(n: usize) -> usize {
    let mut power = 1;
    while power < n {
        power *= 2;
    }
    power
}

fn remove_leading_zeros(mut num: Vec<u32>) -> Vec<u32> {
    while num.len() > 1 && num[0] == 0 {
        num.remove(0);
    }
    if num.is_empty() {
        vec![0]
    } else {
        num
    }
}

fn number_to_vec(mut num: u64) -> Vec<u32> {
    if num == 0 {
        return vec![0];
    }
    
    let mut digits = Vec::new();
    while num > 0 {
        digits.push((num % 10) as u32);
        num /= 10;
    }
    digits.reverse();
    digits
}

fn vec_to_number(num: &Vec<u32>) -> String {
    num.iter().map(|&d| d.to_string()).collect::<String>()
}