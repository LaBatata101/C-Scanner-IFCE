use scanner::{Scanner, Token};

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
    loop {
        let token = scanner.next_token().unwrap();
        if token == &Token::Eof {
            break;
        }

        print!("{:?} ", token);
    }
    println!();
}
