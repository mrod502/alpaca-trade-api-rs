pub mod alpaca;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use alpaca::{
        requests::GetAccountActivities,
        rest::{ClientOptions, ClientV2},
    };
    use chrono::NaiveDateTime;

    use super::*;

    #[tokio::test]
    async fn account() {
        let opts = ClientOptions {
            key: "".into(),
            secret: "".into(),
            broker_key: "".into(),
            broker_secret: "".into(),
            oauth: "".into(),
            base_url: "https://api.alpaca.markets/v2/".into(),
            max_retry: 3,
        };
        let client = &ClientV2::new(opts);
        let acct = client.get_account().await;
        match acct {
            Ok(a) => {
                println!("{:?}", a);
            }
            Err(e) => {
                println!("failed:{}", e);
            }
        }
    }
    #[tokio::test]
    async fn positions() {
        let opts = ClientOptions {
            key: "".into(),
            secret: "".into(),
            broker_key: "".into(),
            broker_secret: "".into(),
            oauth: "".into(),
            base_url: "https://api.alpaca.markets/v2/".into(),
            max_retry: 3,
        };
        let client = &ClientV2::new(opts);
        let pos = client.get_account().await;
        match pos {
            Ok(a) => {
                println!("{:?}", a);
            }
            Err(e) => {
                println!("failed:{}", e);
            }
        }
    }

    #[tokio::test]
    async fn account_activities() {
        println!("client options");
        let opts = ClientOptions {
            key: "".into(),
            secret: "".into(),
            broker_key: "".into(),
            broker_secret: "".into(),
            oauth: "".into(),
            base_url: "https://api.alpaca.markets/v2/".into(),
            max_retry: 3,
        };
        let client = &ClientV2::new(opts);

        let res = client
            .get_account_activities(GetAccountActivities {
                activity_types: vec![],
                date: NaiveDateTime::parse_from_str("2022-01-01 23:56:04", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                until: NaiveDateTime::parse_from_str("2024-09-01 23:56:04", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                after: NaiveDateTime::parse_from_str("2022-01-01 23:56:04", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                direction: None,
                page_size: None,
                page_token: None,
                category: None,
            })
            .await;
        println!("activities");
        match res {
            Ok(v) => {
                println!("OK:{:?}", v);
            }
            Err(e) => {
                println!("FAIL:{:?}", e)
            }
        }
    }
}
