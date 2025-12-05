mod cli;

fn main() {
    println!("Hello, world!");
    let number = 121;
    let result = is_palindrome(number);
    println!("result is {}", result);

    // let result = calculator(5, 2, "$");
    // let result2=  match result {
    //     Ok(v) => v,
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         return;
    //     }
    // };
    //
    // println!("Result2: {}", result2);



}

fn is_palindrome(num: i32) -> bool {
    let chars: Vec<char> = num.to_string().chars().collect();
    let mut right = chars.len() -1;
    let mut left = 0 ;

    while left < right {
        if chars[left] != chars[right] {
            false;
        }
        right -= 1;
        left += 1;
    }
     true
}

fn calculator(
    num1: i32,
    num2: i32,
    operator: &str,
) -> Result<i32, String> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" =>{
            if num2 == 0 {
                Err("Divide by zero".to_string())
            }else {
                Ok(num1 / num2)
            }

        },
        _ => Err(format!("Unknown operator: {}", operator))

    }

}
