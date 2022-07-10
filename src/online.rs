use serde::Deserialize;

const CSV_URL: &str = "https://www.national-lottery.co.uk/results/euromillions/draw-history/csv";

#[derive(Deserialize)]
pub struct Record {
    // #[serde(rename = "DrawDate")]
    // draw_date: String,
    #[serde(rename = "Ball 1")]
    ball_1: u8,
    #[serde(rename = "Ball 2")]
    ball_2: u8,
    #[serde(rename = "Ball 3")]
    ball_3: u8,
    #[serde(rename = "Ball 4")]
    ball_4: u8,
    #[serde(rename = "Ball 5")]
    ball_5: u8,
    #[serde(rename = "Lucky Star 1")]
    lucky_star_1: u8,
    #[serde(rename = "Lucky Star 2")]
    lucky_star_2: u8,
    // #[serde(rename = "UK Millionaire Maker")]
    // uk_millionaire_maker: String,
    // #[serde(rename = "European Millionaire Maker")]
    // european_millionaire_maker: String,
    // #[serde(rename = "DrawNumber")]
    // draw_number: usize,
}

pub async fn get_recent_winning_numbers() -> Result<Vec<Record>, csv::Error> {
    let response = reqwest::get(CSV_URL).await.expect("Could not download file")
        .text().await.expect("Could not read downloaded file");
    let mut reader = csv::Reader::from_reader(response.as_bytes());
    reader.deserialize().collect()
}

pub fn get_numbers_from_records(records: &[Record]) -> Vec<u8> {
    records.iter()
        .flat_map(|r| [r.ball_1, r.ball_2, r.ball_3, r.ball_4, r.ball_5])
        .collect()
}

pub fn get_lucky_stars_from_records(records: &[Record]) -> Vec<u8> {
    records.iter()
        .flat_map(|r| [r.lucky_star_1, r.lucky_star_2])
        .collect()
}
