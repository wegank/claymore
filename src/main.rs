mod util;

fn main() {
    let key = String::from("BCDFG-HJKNM-PQRTV-WXY23-46789");
    let key_info = util::KeyInfo::load(&key)
        .unwrap_or_else(|error| panic!("{}", error));
    println!("{:#?}", key_info);
    println!("{}", key_info.save());
}
