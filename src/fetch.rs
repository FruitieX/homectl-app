use std::{collections::HashMap, rc::Rc};

use yew_functional::{use_effect_with_deps, use_ref, use_state};

use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    Callback,
};
use yew_services::fetch::{FetchService, Method, Request, Response};

use crate::log;

const BASE_URI: &str = "http://localhost:45289";

pub fn use_fetch<T: 'static, B>(
    method: Method,
    endpoint: &str,
    body: Option<B>,
    extra_headers: Option<HashMap<String, String>>,
) -> Rc<T>
where
    T: for<'a> Deserialize<'a> + Default,
    B: serde::Serialize,
{
    let (state, set_state) = use_state(T::default);

    let fetch_task = use_ref(|| None);

    let uri = format!("{}{}", BASE_URI, endpoint);

    let body = match body {
        Some(body) => {
            let json = serde_json::to_string(&body).unwrap();
            Some(json)
        }
        None => None,
    };

    use_effect_with_deps(
        move |_| {
            let request_callback =
                Callback::from(move |response: Response<Json<Result<T, anyhow::Error>>>| {
                    let Json(data) = response.into_body();

                    match data {
                        Ok(data) => set_state(data),
                        Err(e) => {
                            log!("Fetch error: {:?}", e);
                        }
                    }
                });

            let mut request_builder = Request::builder().method(method).uri(uri);

            if let Some(headers) = extra_headers {
                for (header, value) in headers.iter() {
                    request_builder = request_builder.header(header, value);
                }
            }

            let task = match body {
                Some(body) => {
                    let request = request_builder
                        .header("Content-Type", "application/json")
                        .body(Json(&body))
                        .expect("Could not build request.");

                    FetchService::fetch(request, request_callback)
                }
                None => {
                    let request = request_builder
                        .body(Nothing)
                        .expect("Could not build request.");

                    FetchService::fetch(request, request_callback)
                }
            }
            .expect("Failed to start request");

            // Store the task so it isn't canceled immediately
            {
                let mut fetch_task = fetch_task.borrow_mut();
                *fetch_task = Some(task);
            };

            move || {
                fetch_task.take();
            }
        },
        (),
    );

    state
}
