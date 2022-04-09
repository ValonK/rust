// 1, 1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610....

pub fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    
    return fib(n - 1) + fib(n - 2);
}