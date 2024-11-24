use chrono::Duration;

use super::{
    entities::{
        Account, AccountActivity, AccountConfigurations, Announcement, Asset, CalendarDay, Clock,
        Order, PortfolioHistory, Position, Watchlist,
    },
    requests::{
        AddSymbolToWatchlist, CloseAllPositions, ClosePosition, CreateWatchlist,
        GetAccountActivities, GetAnnouncements, GetAssets, GetCalendar, GetOrders,
        GetPortfolioHistory, PlaceOrder, RemoveSymbolFromWatchlist, ReplaceOrder, UpdateWatchlist,
    },
    responses::{APIError, CloseAllPositionsResponse},
};

type APIResult<T> = Result<T, APIError>;

struct ClientOptions {
    pub key: String,
    pub secret: String,
    pub broker_key: String,
    pub broker_secret: String,
    pub oauth: String,
    pub base_url: String,
    pub max_retry: isize,
    pub retry_delay: Duration,
}

pub struct RestClientV2 {
    opts: ClientOptions,
}

impl RestClientV2 {
    pub async fn get_account(&self) -> APIResult<Account> {
        todo!()
    }
    pub async fn get_account_configurations(&self) -> APIResult<AccountConfigurations> {
        todo!()
    }
    pub async fn update_account_configurations(&self) -> APIResult<AccountConfigurations> {
        todo!()
    }
    pub async fn get_account_activities(
        &self,
        params: GetAccountActivities,
    ) -> APIResult<Vec<AccountActivity>> {
        todo!()
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
        todo!()
    }
    pub async fn get_positions(&self) -> APIResult<Vec<Position>> {
        todo!()
    }

    pub async fn get_position(&self, symbol: &str) -> APIResult<Position> {
        todo!()
    }
    pub async fn close_all_positions(
        &self,
        params: CloseAllPositions,
    ) -> APIResult<CloseAllPositionsResponse> {
        todo!()
    }
    pub async fn close_position(
        &self,
        ClosePosition {
            qty,
            percentage,
            symbol,
        }: ClosePosition,
    ) -> APIResult<Order> {
        todo!()
    }
    pub async fn get_clock(&self) -> APIResult<Clock> {
        todo!()
    }

    pub async fn get_calendar(&self, params: GetCalendar) -> APIResult<Vec<CalendarDay>> {
        todo!()
    }

    pub async fn get_orders(&self, params: GetOrders) -> APIResult<Vec<Order>> {
        todo!()
    }

    pub async fn place_order(&self, params: PlaceOrder) -> APIResult<Order> {
        todo!()
    }
    pub async fn get_order(&self, id: String) -> APIResult<Order> {
        todo!()
    }
    pub async fn get_order_by_client_order_id(&self, id: String) -> APIResult<Order> {
        todo!()
    }
    pub async fn replace_order(&self, id: String, replace: ReplaceOrder) -> APIResult<Order> {
        todo!()
    }

    pub async fn cancel_order(&self, id: String) -> APIResult<()> {
        todo!()
    }
    pub async fn cancel_all_orders(&self) -> APIResult<()> {
        todo!()
    }
    pub async fn get_assets(&self, params: GetAssets) -> APIResult<Vec<Asset>> {
        todo!()
    }

    pub async fn get_asset(&self, sym: String) -> APIResult<Asset> {
        todo!()
    }

    pub async fn get_announcements(
        &self,
        params: GetAnnouncements,
    ) -> APIResult<Vec<Announcement>> {
        todo!()
    }

    pub async fn get_announcement(&self, id: String) -> APIResult<Announcement> {
        todo!()
    }
    pub async fn get_watchlists(&self) -> APIResult<Vec<Watchlist>> {
        todo!()
    }
    pub async fn create_watchlist(&self, params: CreateWatchlist) -> APIResult<Watchlist> {
        todo!()
    }

    pub async fn get_watchlist(&self, id: String) -> APIResult<Watchlist> {
        todo!()
    }

    pub async fn update_watchlist(&self, params: UpdateWatchlist) -> APIResult<Watchlist> {
        todo!()
    }

    pub async fn add_symbol_to_watchlist(
        &self,
        id: String,
        params: AddSymbolToWatchlist,
    ) -> APIResult<Watchlist> {
        todo!()
    }

    pub async fn remove_symbol_from_watchlist(
        &self,
        id: String,
        params: RemoveSymbolFromWatchlist,
    ) -> APIResult<Watchlist> {
        todo!()
    }

    pub async fn delete_watchlist(&self, id: String) -> APIResult<()> {
        todo!()
    }
}

pub trait WsClient {}
