use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use core::panic;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls{
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist{
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Song {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T>{
    items: Vec<T>
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse{
    songs: Items<Song>,
}

fn print_songs(songs: Vec<&Song>){
    for song in songs {
        println!("{}", song.name);
        println!("{}", song.album.name);
        println!(
            "{}",
            song
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("{}", song.external_urls.spotify);
        println!("-----------------")
    }
}

#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let auth_token = &args[2];
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query
    );

    let client = reqwest::Client::new();
    let response = client
    .get(url)
    .header(AUTHORIZATION, format!("Bearer {}", auth_token))
    .header(CONTENT_TYPE, "application/json")
    .header(ACCEPT, "application/json")
    .send()
    .await
    .unwrap();
    match response.status(){
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_songs(parsed.songs.items.iter().collect()),
                Err(_) => println!("Response didn't match the struct")
            };
        }
        
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to get a new token");
        }
        other => {
            panic!("Oops! Something unexpected happened: {:?}", other);
        }
    };
}