use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use std::collections::BTreeMap;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_data(callback: FetchCallback<BTreeMap<String, String>>) -> FetchTask {
    let req = Request::get(format!("/foo"))
        .header("X-ACNHC-API-KEY", "foo")
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}
