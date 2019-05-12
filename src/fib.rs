fn fib(n: usize) -> usize {
    let mut result = 0;
    if n == 1 || n == 2 {
        result = 1;
    } else {
        result = fib(n - 1) + fib(n - 2);
    }
    return result;
}
fn fib_memo(n: usize, memo: &mut [usize]) -> usize {
    if memo[n - 1] != 0 {
        println!("{}", n);
        return memo[n - 1];
    }

    let mut result = 0;
    if n == 1 || n == 2 {
        result = 1;
    } else {
        result = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    }
    memo[n - 1] = result;

    return result;
}
