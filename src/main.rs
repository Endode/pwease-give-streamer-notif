mod global;

enum Platform {
    YouTube,
    Twitch,
}

struct Streamer {
    username: String,
    platform: Platform,
}

fn main() {
    println!("{} v{}", global::PROGRAM_NAME, global::VERSION.get_string());
    let mut args = std::env::args();
    args.next();
    for arg in args {
        println!("Arg: {}", arg);
    }
}
