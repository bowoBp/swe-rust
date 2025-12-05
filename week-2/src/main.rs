mod palindrome;
mod cli;

use cli::Command;
use palindrome::checker::PalindromeChecker;
fn main() {
   match cli::parse_args(){
       Command::Check(n) =>{
           if PalindromeChecker::is_palindrome(n){
               println!("Palindrome is a palindrome");
           }else{
               println!("Palindrome is not a palindrome");
           }
       }
      Command::Invalid =>{
          println!("Palindrome is invalid");
      }
   }
}
