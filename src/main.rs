mod global;
use std::vec;
use std::time;
use std::thread;

// 5 minutes
const DEFAULT_FREQUENCY: time::Duration = time::Duration::from_secs(60*5);

#[derive(Debug)]
enum Platform {
    None,
    YouTube,
    Twitch,
}

fn string_to_platform(platform: &str) -> Platform {
    match platform {
        "none" => Platform::None,
        "youtube" => Platform::YouTube,
        "twitch" => Platform::Twitch,
        _ => Platform::None,
    }
}

fn get_platform_url(username: &str, platform: &Platform) -> String {
    match platform {
        Platform::YouTube => format!("https://www.youtube.com/{}/live", username),
        // Platform::Twitch => format!("https://www.twitch.tv/{}", username),
        Platform::Twitch => format!("https://static-cdn.jtvnw.net/previews-ttv/live_user_{}-440x248.jpg", username),
        _ => String::new(),
    }
}

struct Streamer {
    username: String,
    platform: Platform,
    streaming: bool,
}

impl Streamer {
    fn new(username: String, platform: Platform) -> Streamer {
        Streamer { username, platform, streaming: false }
    }

    fn set_platform(&mut self, platform: Platform) {
        self.platform = platform;
    }

    async fn check_streaming(&mut self) {
        #[cfg(debug_assertions)]
        println!("Checking if [{}] is live on [{:?}]", self.username, self.platform);
        let client = reqwest::Client::new();
        let response = client.get(get_platform_url(self.username.as_str(), &self.platform)).send().await.unwrap();
        let now_streaming: bool;
        match self.platform {
            Platform::YouTube => {
                now_streaming = response.text().await.unwrap().contains("isLiveBroadcast")
            }
            Platform::Twitch => {
                now_streaming = response.url().path().contains("previews-ttv")
            }
            _ => {
                now_streaming = false
            }
        }

        if now_streaming {
            if !self.streaming {
                self.streaming = true;
                #[cfg(debug_assertions)]
                println!("[{}] is live on [{:?}]", self.username, self.platform);
                let _ = notifica::notify(self.username.as_str(), format!("{} is live on {:?}!", self.username, self.platform).as_str());
            }
        } else {
            self.streaming = false;
        }
    }
}
#[tokio::main]
async fn main() {
    println!("{} v{}", global::PROGRAM_NAME, global::VERSION.get_string());
    let mut args = std::env::args();
    args.next();
    let mut streamers: Vec<Streamer> = vec![];
    if args.len() < 2 {
        // println!("Usage: pwease-give-streamer-notif <username> <platform> [ <frequency> ]");
        println!("Usage: pwease-give-streamer-notif <username> <platform> <username2> <platform2>");
        println!("Example: pwease-give-streamer-notif bingus youtube");
        // println!("Example: pwease-give-streamer-notif tsoding twitch 10m");
        std::process::exit(1);
    }
    let mut streamer: Option<Streamer> = None;
    for arg in args {
        match streamer {
            Some(mut s) => {
                s.set_platform(string_to_platform(&arg));
                streamers.push(s);
                streamer = None;
            }
            None => {
                streamer = Some(Streamer::new(arg, Platform::None));
            }
        }
    }
    loop {
        for s in &mut streamers {
            s.check_streaming().await;
        }
        thread::sleep(DEFAULT_FREQUENCY);
    }
}
