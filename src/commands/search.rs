use crate::config::Config;
use clap::Args;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Args)]
pub struct SearchArgs {
    #[arg(help = "Arama terimi (altyazı başlığı veya anime adı)")]
    query: String,
}

#[derive(Deserialize, Debug)]
struct SubtitleData {
    subtitle_id: u64,
    subtitle_title: String,
    subtitle_release_name: String,
    subtitle_language: String,
    approval_status: String,
}

#[derive(Deserialize, Debug)]
struct SubtitleResponse {
    success: bool,
    data: Option<Vec<SubtitleData>>,
    message: Option<String>,
}

pub fn execute(config: &Config, args: &SearchArgs) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = "https://anisub.co/api/subtitles";
    
    let mut request = client.get(url)
        .query(&[("q", &args.query)]);

    if let Some(token) = &config.token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let response = request.send()?;
    
    if !response.status().is_success() {
        println!("İstek başarısız oldu: HTTP {}", response.status());
        return Ok(());
    }

    let result: SubtitleResponse = response.json()?;

    if !result.success {
        println!("Hata: {}", result.message.unwrap_or_else(|| "Bilinmeyen bir hata oluştu.".to_string()));
        return Ok(());
    }

    if let Some(subtitles) = result.data {
        if subtitles.is_empty() {
            println!("Arama sonucunda hiçbir altyazı bulunamadı.");
        } else {
            println!("Bulunan Altyazılar:\n");
            for sub in subtitles {
                println!("- Başlık: {}", sub.subtitle_title);
                println!("  ID: {}", sub.subtitle_id);
                println!("  Yayımlanma adı: {}", sub.subtitle_release_name);
                println!("  Dil: {}", sub.subtitle_language);
                println!("  Durum: {}\n", sub.approval_status);
            }
        }
    } else {
        println!("Altyazı verisi alınamadı.");
    }

    Ok(())
}
