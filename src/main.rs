#![allow(unused_unsafe)]

use std::rc::Rc;

use yew::prelude::*;
use yew_functional::{function_component, use_effect_with_deps, use_ref, use_state};

use serde::Deserialize;
use yew::format::{Json, Nothing};
use yew_material::{MatList, MatListItem};
use yew_services::fetch::{FetchService, Method, Request, Response};

mod log;

#[derive(Properties, Clone, PartialEq)]
pub struct TestProps {
    pub value: Rc<i32>,
}

#[function_component(Test)]
fn test(props: &TestProps) -> Html {
    html! {
        <div>
            { &props.value }
        </div>
    }
}

#[derive(Clone, Deserialize)]
struct Device {
    name: String,
}

#[derive(Clone, Default, Deserialize)]
struct DevicesResponse {
    devices: Vec<Device>,
}

const BASE_URI: &str = "http://localhost:45289";

fn use_fetch<T: 'static>(method: Method, endpoint: &str) -> Rc<T>
where
    T: for<'a> Deserialize<'a> + Default,
{
    let (state, set_state) = use_state(T::default);

    let fetch_task = use_ref(|| None);

    let uri = format!("{}{}", BASE_URI, endpoint);

    use_effect_with_deps(
        move |_| {
            let request = Request::builder()
                .method(method)
                .uri(uri)
                .body(Nothing)
                .expect("Could not build request.");

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

            let task =
                FetchService::fetch(request, request_callback).expect("Failed to start request");

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

fn fetch_devices() -> Rc<Vec<Device>> {
    let devices = use_fetch::<DevicesResponse>(Method::GET, "/devices")
        .devices
        .clone();

    Rc::new(devices)
}

#[function_component(App)]
fn app() -> Html {
    let (counter, set_counter) = use_state(|| 0);

    let onclick = {
        let counter = Rc::clone(&counter);
        Callback::from(move |_| set_counter(*counter + 1))
    };

    let devices = fetch_devices();
    let devices_list: Html = devices
        .iter()
        .map(|device| {
            html! {<MatListItem>{device.name.clone()}</MatListItem> }
        })
        .collect();

    html! {
        <div>
            <button onclick={onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value:" }</b>
                <Test value={counter} />
            </p>
            <h2>{"Devices:"}</h2>
            <MatList>
                {devices_list}
            </MatList>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
