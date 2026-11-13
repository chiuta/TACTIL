use tactil_core::brf_to_unicode;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["--version"] => println!("tactil {}", env!("CARGO_PKG_VERSION")),
        ["brf", text] => println!("{}", brf_to_unicode(text)),
        _ => {
            eprintln!("usage: tactil brf TEXT | --version");
            std::process::exit(2);
        }
    }
}
