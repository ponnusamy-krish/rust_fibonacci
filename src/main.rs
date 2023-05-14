fn main() {
 let n = 10;
 let mut fib1 = 0;
 let mut fib2 = 1;
 let mut current_number: i32 = 0;
 let mut i = 1;

 while i <= n{

     println!("{current_number} {i}");
     fib1 = fib2;
     fib2 = current_number;
     current_number = fib2 + fib1;
     i+=1;
 }


}
