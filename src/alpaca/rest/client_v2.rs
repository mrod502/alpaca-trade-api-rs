use std::{collections::HashMap, fmt::Display, str::FromStr};

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, Duration, Offset, TimeZone,
};
use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Bytes, Incoming},
    Method, Request, Response, Uri,
};
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    add,
    alpaca::{
        entities::{
            Account, AccountActivity, AccountConfigurations, Announcement, Asset, CalendarDay,
            Clock, Order, PortfolioHistory, Position, Watchlist,
        },
        requests::{
            AddSymbolToWatchlist, CloseAllPositions, ClosePosition, CreateWatchlist,
            GetAccountActivities, GetAnnouncements, GetAssets, GetCalendar, GetOrders,
            GetPortfolioHistory, PlaceOrder, RemoveSymbolFromWatchlist, ReplaceOrder,
            UpdateAccountConfigurations, UpdateWatchlist,
        },
        responses::{APIError, CloseAllPositionsResponse},
        util::query::Query,
    },
    query,
};

type APIResult<T> = Result<T, APIError>;

fn format_rfc_3339_nano<'a, T>(v: DateTime<T>) -> DelayedFormat<StrftimeItems<'a>>
where
    T: TimeZone,
    T::Offset: Display,
{
    v.format("%Y-%m-%dT%H:%M:%S%.f%:z")
}

pub struct ClientOptions {
    pub key: String,
    pub secret: String,
    pub broker_key: String,
    pub broker_secret: String,
    pub oauth: String,
    pub base_url: String,
    pub max_retry: isize,
    pub retry_delay: Duration,
}

impl ClientOptions {}

pub struct ClientV2 {
    opts: ClientOptions,
}

#[derive(Clone)]
struct PerformOptions<T>
where
    T: Serialize + Clone,
{
    path: String,
    params: Option<HashMap<String, String>>,
    body: Option<T>,
}

impl<T> Default for PerformOptions<T>
where
    T: Serialize + Clone,
{
    fn default() -> Self {
        Self {
            path: "".into(),
            params: None,
            body: None,
        }
    }
}

impl<T> PerformOptions<T>
where
    T: Serialize + Clone,
{
    fn with_path(self, path: String) -> Self {
        Self {
            path,
            params: self.params,
            body: self.body,
        }
    }
}

const HDR_API_KEY_ID: &str = "APCA-API-KEY-ID";
const HDR_API_SECRET: &str = "APCA-API-SECRET-KEY";

impl ClientV2 {
    pub fn new(opts: ClientOptions) -> Self {
        Self { opts }
    }

    pub async fn get_account(&self) -> APIResult<Account> {
        self.perform_json::<(), Account>(
            Method::GET,
            PerformOptions::default().with_path("account".into()),
        )
        .await
    }
    pub async fn get_account_configurations(&self) -> APIResult<AccountConfigurations> {
        self.perform_json::<(), AccountConfigurations>(
            Method::GET,
            PerformOptions::default().with_path("account/configurations".into()),
        )
        .await
    }
    pub async fn update_account_configurations(
        &self,
        params: UpdateAccountConfigurations,
    ) -> APIResult<AccountConfigurations> {
        self.perform_json(
            Method::PATCH,
            PerformOptions {
                path: "account/configurations".into(),
                params: None,
                body: Some(params),
            },
        )
        .await
    }
    pub async fn get_account_activities(
        &self,
        params: GetAccountActivities,
    ) -> APIResult<Vec<AccountActivity>> {
        self.perform_json(
            Method::GET,
            PerformOptions {
                path: "account/activities".into(),
                params: None,
                body: Some(params),
            },
        )
        .await
    }
    pub async fn get_portfolio_history(
        &self,
        GetPortfolioHistory {
            period,
            time_frame,
            date_end,
            extended_hours,
        }: GetPortfolioHistory,
    ) -> APIResult<PortfolioHistory> {
        let mut q = Query::new();
        if let Some(p) = period {
            q.add("period", p);
        }

        if let Some(tf) = time_frame {
            q.add("timeframe", tf);
        }
        if let Some(de) = date_end {
            q.add("date_end", de.date_naive().format("%Y-%m-%d"));
        }
        q.add("extended_hours", extended_hours);

        self.get_json(format!("account/portfolio/history{}", q.to_string()))
            .await
    }

    pub async fn get_positions(&self) -> APIResult<Vec<Position>> {
        self.get_json("positions").await
    }

    pub async fn get_position(&self, symbol: &str) -> APIResult<Position> {
        self.get_json(format!("positions/{}", symbol)).await
    }

    pub async fn close_all_positions(
        &self,
        CloseAllPositions { cancel_orders }: CloseAllPositions,
    ) -> APIResult<CloseAllPositionsResponse> {
        let q = query![("cancel_orders", Some(cancel_orders))];
        self.perform_json(
            Method::DELETE,
            PerformOptions::<()> {
                path: format!("positions{}", q.to_string()),
                params: None,
                body: None,
            },
        )
        .await
    }

    pub async fn close_position(
        &self,
        ClosePosition {
            qty,
            percentage,
            symbol,
        }: ClosePosition,
    ) -> APIResult<Order> {
        let mut q = Query::new();
        if qty != 0.into() {
            q.add("qty", qty);
        }
        if percentage != 0.into() {
            q.add("percentage", percentage);
        }
        self.perform_json(
            Method::DELETE,
            PerformOptions::<()> {
                path: format!("positions/{}/{}", symbol, q.to_query(true)),
                params: None,
                body: None,
            },
        )
        .await
    }
    pub async fn get_clock(&self) -> APIResult<Clock> {
        self.get_json("clock").await
    }

    pub async fn get_calendar(
        &self,
        GetCalendar { start, end }: GetCalendar,
    ) -> APIResult<Vec<CalendarDay>> {
        let mut q = Query::new();
        if let Some(start) = start {
            q.add("start", start.date_naive().format("%Y-%m-%d"));
        }
        if let Some(end) = end {
            q.add("end", end.date_naive().format("%Y-%m-%d"));
        }

        self.get_json(format!("calendar{}", q.to_string())).await
    }

    pub async fn get_orders(
        &self,
        GetOrders {
            status,
            limit,
            after,
            until,
            direction,
            nested,
            side,
            symbols,
        }: GetOrders,
    ) -> APIResult<Vec<Order>> {
        let mut q = query![
            ("status", status),
            ("limit", limit),
            ("direction", direction),
            ("nested", nested),
            ("side", side)
        ];
        if let Some(symbols) = symbols {
            if symbols.len() > 0 {
                q.add("symbols", symbols.join(","));
            }
            if let Some(after) = after {
                q.add("after", format_rfc_3339_nano(after));
            }
            if let Some(after) = until {
                q.add("until", format_rfc_3339_nano(after));
            }
        }
        self.get_json(format!("orders{}", q.to_string())).await
    }

    pub async fn place_order(&self, params: PlaceOrder) -> APIResult<Order> {
        self.perform_json(
            Method::POST,
            PerformOptions {
                path: "orders".into(),
                params: None,
                body: Some(params),
            },
        )
        .await
    }
    pub async fn get_order(&self, id: String) -> APIResult<Order> {
        self.get_json(format!("orders/{}", id)).await
    }
    pub async fn get_order_by_client_order_id(&self, id: String) -> APIResult<Order> {
        let q = query![("client_order_id", Some(id))];
        self.get_json(format!("orders:by_client_order_id{}", q.to_string()))
            .await
    }
    pub async fn replace_order(&self, id: String, replace: ReplaceOrder) -> APIResult<Order> {
        self.perform_json(
            Method::PATCH,
            PerformOptions {
                path: format!("orders/{}", id),
                params: None,
                body: Some(replace),
            },
        )
        .await
    }

    pub async fn cancel_order(&self, id: String) -> APIResult<()> {
        self.perform_json(
            Method::DELETE,
            PerformOptions::<()> {
                body: None,
                path: format!("orders/{}", id),
                params: None,
            },
        )
        .await
    }
    pub async fn cancel_all_orders(&self) -> APIResult<()> {
        self.perform(Method::DELETE, "orders".to_string(), None)
            .await?;
        Ok(())
    }
    pub async fn get_assets(
        &self,
        GetAssets {
            status,
            asset_class,
            exchange,
        }: GetAssets,
    ) -> APIResult<Vec<Asset>> {
        let q = query![
            ("status", status),
            ("asset_class", asset_class),
            ("exchange", exchange)
        ];
        self.get_json(format!("assets{}", q.to_string())).await
    }

    pub async fn get_asset(&self, sym: String) -> APIResult<Asset> {
        self.get_json(format!("assets/{}", sym)).await
    }

    pub async fn get_announcements(
        &self,
        GetAnnouncements {
            ca_types,
            since,
            until,
            symbol,
            cusip,
            date_type,
        }: GetAnnouncements,
    ) -> APIResult<Vec<Announcement>> {
        let mut q = query![("symbol", symbol), ("cusip", cusip)];
        q.add_vec("ca_types", ca_types);
        q.add("since", format_rfc_3339_nano(since));
        q.add("until", format_rfc_3339_nano(until));
        q.add("date_type", date_type);
        self.get_json(format!("corporate_actions/announcements{}", q.to_string()))
            .await
    }

    pub async fn get_announcement(&self, id: String) -> APIResult<Announcement> {
        self.get_json(format!("corporate_actions/announcements/{}", id))
            .await
    }
    pub async fn get_watchlists(&self) -> APIResult<Vec<Watchlist>> {
        self.get_json("watchlists").await
    }
    pub async fn create_watchlist(&self, params: CreateWatchlist) -> APIResult<Watchlist> {
        self.perform_json(
            Method::POST,
            PerformOptions {
                path: "watchlists".to_string(),
                params: None,
                body: Some(params),
            },
        )
        .await
    }

    pub async fn get_watchlist(&self, id: String) -> APIResult<Watchlist> {
        self.get_json(format!("watchlists/{}", id)).await
    }

    pub async fn update_watchlist(
        &self,
        id: &str,
        params: UpdateWatchlist,
    ) -> APIResult<Watchlist> {
        self.perform_json(
            Method::PUT,
            PerformOptions {
                path: format!("watchlists/{}", id),
                params: None,
                body: Some(params),
            },
        )
        .await
    }

    pub async fn add_symbol_to_watchlist(
        &self,
        id: String,
        params: AddSymbolToWatchlist,
    ) -> APIResult<Watchlist> {
        self.perform_json(
            Method::POST,
            PerformOptions {
                path: format!("watchlists/{}", id),
                params: None,
                body: Some(params),
            },
        )
        .await
    }

    pub async fn remove_symbol_from_watchlist(
        &self,
        id: String,
        params: RemoveSymbolFromWatchlist,
    ) -> APIResult<Watchlist> {
        self.perform_json(
            Method::DELETE,
            PerformOptions::<()> {
                path: format!("watchlists/{}/{}", id, params.symbol),
                params: None,
                body: None,
            },
        )
        .await
    }

    pub async fn delete_watchlist(&self, id: String) -> APIResult<()> {
        self.perform_json(
            Method::DELETE,
            PerformOptions::<()> {
                path: format!("watchlists/{}", id),
                params: None,
                body: None,
            },
        )
        .await
    }

    fn url(&self, path: String) -> APIResult<Uri> {
        println!("getting uri");
        let base = self.opts.base_url.trim_end_matches("/");
        let path = path.trim_start_matches("/");
        let uri = format!("{}/{}", base, path);
        let res = match Uri::from_str(&uri) {
            Ok(u) => Ok(u),
            Err(e) => Err(APIError::default().with_message(format!("{}", e))),
        };
        println!("{:?}", res.clone());
        res
    }

    async fn get_json<T>(&self, path: impl ToString) -> APIResult<T>
    where
        T: DeserializeOwned,
    {
        let path = path.to_string();
        let result = self.perform(Method::GET, path, None).await?;
        let bytes = String::from_utf8(result.collect().await.unwrap().to_bytes().to_vec())
            .unwrap_or_default();
        let de: T = match serde_json::from_str(&bytes) {
            Ok(v) => v,
            Err(e) => {
                return Err(APIError::default().with_message(format!("failed to deserialize:{}", e)))
            }
        };

        Ok(de)
    }

    async fn perform_json<R, T>(&self, method: Method, opts: PerformOptions<R>) -> APIResult<T>
    where
        R: Serialize + Clone,
        T: DeserializeOwned,
    {
        let body = match opts.body {
            Some(bod) => Some(Bytes::from(serde_json::to_string(&bod).unwrap())),
            None => None,
        };

        let result = self.perform(method, opts.path, body).await?;

        let bytes = String::from_utf8(result.collect().await.unwrap().to_bytes().to_vec())
            .unwrap_or_default();
        println!("raw_body:{}", bytes);
        let de: T = match serde_json::from_str(&bytes) {
            Ok(v) => v,
            Err(e) => {
                return Err(APIError::default().with_message(format!("failed to deserialize:{}", e)))
            }
        };

        Ok(de)
    }

    async fn perform(
        &self,
        method: Method,
        path: String,
        body: Option<Bytes>,
    ) -> APIResult<Response<Incoming>> {
        let req_body = match body {
            Some(b) => b,
            None => Bytes::new(),
        };

        let req = match Request::builder()
            .uri(self.url(path).unwrap())
            .method(method)
            .header(HDR_API_KEY_ID, self.opts.key.clone())
            .header(HDR_API_SECRET, self.opts.secret.clone())
            .body(Full::new(req_body))
        {
            Ok(r) => r,
            Err(e) => return Err(APIError::default().with_message(format!("{}", e))),
        };

        let client: Client<HttpsConnector<HttpConnector>, Full<Bytes>> =
            Client::builder(hyper_util::rt::TokioExecutor::new()).build(HttpsConnector::new());

        let res = match client.request(req).await {
            Ok(r) => {
                println!("success:{:?}", r);
                r
            }
            Err(e) => {
                println!("failed to send request:{}", e);
                return Err(APIError::default().with_message(format!("{}", e)));
            }
        };

        Ok(res)
    }
}
