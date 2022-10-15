use scanner::Scanner;

fn main() {
    let src = "
int main() {
    int a = 10;
    int c;
    for (int i = 0; i <= 10; i++) {
        if (i + a <= 15) {
            c = i;
        }
    }
    return 0;
}
";
    println!("Scanning code:\n{}", src);
    let mut scanner = Scanner::new(src);
    println!("TOKENS:");
    while let Some(token) = scanner.next_token() {
        print!("{:?} ", token);
    }

    println!();
}
