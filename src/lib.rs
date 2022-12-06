use std::{
    fs::{create_dir, read_to_string, write},
    path::Path,
};

const CACHE_DIRECTORY: &str = "./cache";

fn get_content(id: &str) -> String {
    let session_key =
        std::env::var("AOC_COOKIE_SESSION").expect("Unable to load AOC_COOKIE_SESSION var env");
    let session_header = format!("session={}", session_key);

    let client = reqwest::blocking::Client::new();

    let url = format!("https://adventofcode.com/2022/day/{}/input", id);
    let content = client
        .get(url)
        .header("cookie", session_header)
        .send()
        .expect("Unable to load data");

    content.text().expect("Unable to parse data")
}

pub fn load_day(id: &str) -> String {
    let root = Path::new(CACHE_DIRECTORY);
    if !root.exists() {
        create_dir(root).expect("Unable to create cache directory");
    }

    let filename = &format!("day{}.txt", id);
    let cache_file = root.join(filename);

    if cache_file.exists() {
        read_to_string(cache_file).expect("Unable to read cached file")
    } else {
        let content = get_content(id);
        write(cache_file, &content).expect("Unable to write cache file");

        content
    }
}
