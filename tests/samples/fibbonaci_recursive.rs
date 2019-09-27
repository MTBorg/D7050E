fn fib_rec(n: i32) -> i32{
    if n == 0{
        return 0;
    }
    if n == 1{
        return 1;
    }
    return fib_rec(n - 1) + fib_rec(n - 2);
}

fn main(){
    return fib_rec(9);
}
