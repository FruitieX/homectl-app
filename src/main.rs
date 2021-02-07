#![allow(unused_unsafe)]

use std::{rc::Rc};

use fetch::use_fetch;
use yew::prelude::*;
use yew_functional::{function_component, use_state};

use serde::Deserialize;
use yew_material::{MatList, MatListItem};
use yew_services::fetch::Method;

mod fetch;
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

fn fetch_devices() -> Rc<Vec<Device>> {
    let response: Rc<DevicesResponse> = use_fetch(Method::GET, "/devices", None::<()>, None);

    Rc::new(response.devices.clone())
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
