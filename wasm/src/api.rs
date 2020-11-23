use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use crate::types::CatalogData;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_catalog_data(userid: i32, pri: i32, sec: i32, callback: FetchCallback<Vec<CatalogData>>) -> FetchTask {
    let req = Request::get(format!("json/{}.{}.{}.json", userid, pri, sec))
        .header("X-ACNHC-API-KEY", "foo")
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}
