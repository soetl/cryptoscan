pub(crate) mod coinmarketcap {
    use crate::driving::tauri::coins::{CoinResponse, CoinsResponse};

    const API_KEY_HEADER: &str = "X-CMC_PRO_API_KEY";
    const API_QUOTES_LATEST: &str =
        "https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest";

    pub async fn fetch_ids(ids: Vec<u32>, token: String) -> Result<CoinsResponse, String> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "{}?id={}",
                API_QUOTES_LATEST,
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ))
            .header(API_KEY_HEADER, token)
            .send()
            .await;

        match res {
            Ok(body) => {
                let body = body.json::<serde_json::Value>().await.unwrap();

                if body["status"]["error_code"] != 0 {
                    return Err(format!("Error: {}", body["status"]["error_message"]));
                }

                let data = body["data"].as_object().unwrap();

                if data.is_empty() {
                    return Err("Error: No data found".to_string());
                }

                let mut results: Vec<CoinResponse> = vec![];

                for (_, value) in data.iter() {
                    results.push(serde_json::from_value(value.clone()).unwrap());
                }

                Ok(CoinsResponse { coins: results })
            }
            Err(e) => Err(format!("Error: {}", e)),
        }
    }

    pub async fn fetch_symbols(
        symbol: Vec<String>,
        token: String,
    ) -> Result<CoinsResponse, String> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}?symbol={}", API_QUOTES_LATEST, symbol.join(",")))
            .header(API_KEY_HEADER, token)
            .send()
            .await;

        match res {
            Ok(body) => {
                let body = body.json::<serde_json::Value>().await.unwrap();

                if body["status"]["error_code"] != 0 {
                    return Err(format!("Error: {}", body["status"]["error_message"]));
                }

                let data = body["data"].as_object().unwrap();

                if data.is_empty() {
                    return Err("Error: No data found".to_string());
                }

                let mut results: Vec<CoinResponse> = vec![];

                for (_, value) in data.iter() {
                    let mut result: Vec<CoinResponse> = serde_json::from_value(value.clone()).unwrap();
                    results.append(&mut result);
                }

                Ok(CoinsResponse { coins: results })
            }
            Err(e) => Err(format!("Error: {}", e)),
        }
    }
}
