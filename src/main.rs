use chrono::{Datelike, Utc};

use tokio::process::Command;
use tokio::time::{sleep, Duration};

fn get_scheme(is_day: bool) -> String {
    dotenv::dotenv().ok();
    let light_scheme = dotenv::var("LIGHT_SCHEME").unwrap_or("BreezeClassic".to_string());
    let dark_scheme = dotenv::var("DARK_SCHEME").unwrap_or("BreezeDark".to_string());
    match is_day {
        true => light_scheme.to_string(),
        _ => dark_scheme.to_string(),
    }
}

async fn change_theme(theme: &str) {
    let mut child = Command::new("plasma-apply-colorscheme")
        .arg(theme)
        .spawn()
        .expect("failed to spawn");
    let _status = child.wait().await;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let location = dotenv::var("LOCATION").unwrap_or("51.509865,-0.118092".to_string());
    let location = location.split(",");
    let location = location.collect::<Vec<&str>>();
    let long: f64 = location[0].parse().unwrap();
    let lat: f64 = location[1].parse().unwrap();
    println!("location: {},{}", long, lat);
    loop {
        let now = Utc::now();
        let next_day = now + chrono::Duration::days(1);
        let (sunrise, sunset) =
            sunrise::sunrise_sunset(long, lat, now.year(), now.month(), now.day());
        let (sunrise_next, _sunset_next) =
            sunrise::sunrise_sunset(long, lat, next_day.year(), next_day.month(), next_day.day());

        let is_day = now.timestamp() < sunset;
        let scheme = get_scheme(is_day);
        change_theme(&scheme).await;

        println!("now: {} sunrise: {} sunset: {}", is_day, sunrise, sunset);

        let dur = match now.timestamp() < sunset {
            true => Duration::from_secs((sunset - now.timestamp()).try_into().unwrap()),
            _ => Duration::from_secs((sunrise_next - now.timestamp()).try_into().unwrap()),
        };

        println!("sleeping until: {:?}", dur);
        sleep(dur).await;
    }
}
