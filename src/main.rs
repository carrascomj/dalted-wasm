use js_sys::JsString;
use leptos::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Event, FileReader};
mod image_processing;
use image_processing::{color_filter, decode_raw_image, matrices::MATRICES};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Example(String);

// Iteration is a very common task in most applications.
// So how do you take a list of data and render it in the DOM?
// This example will show you the two ways:
// 1) for mostly-static lists, using Rust iterators
// 2) for lists that grow, shrink, or move items, using <For/>

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList/>
    }
}

/// A list of counters, without the ability
/// to add or remove any.
#[component]
fn StaticList(cx: Scope) -> impl IntoView {
    let (shared_img, set_image): (ReadSignal<Option<image::DynamicImage>>, _) =
        create_signal(cx, None);
    // create counter signals that start at incrementing numbers
    let file_input = create_node_ref::<html::Input>(cx);
    let filereader = FileReader::new().unwrap().dyn_into::<FileReader>().unwrap();
    let onload = Closure::wrap(Box::new(move |event: Event| {
        let element = event.target().unwrap().dyn_into::<FileReader>().unwrap();
        let data = element.result().unwrap();
        let file_string: JsString = data.dyn_into::<JsString>().unwrap();
        let file_vec: Vec<u8> = file_string.iter().map(|x| x as u8).collect();
        let image = decode_raw_image(&file_vec).ok();
        set_image(image)
    }) as Box<dyn FnMut(_)>);
    filereader.set_onloadend(Some(onload.as_ref().unchecked_ref()));
    onload.forget();

    // when you have a list that doesn't change, you can
    // manipulate it using ordinary Rust iterators
    // and collect it into a Vec<_> to insert it into the DOM
    let imgs = MATRICES.map(|mat| {
        view! { cx,
            <li>
                <picture>
                <img src={move || shared_img.with(|im|
                {
                log!("Processing!");
                String::from("data:image/png;base64,") + im.as_ref().ok_or(0)
                    .map(|i| color_filter(i, mat).unwrap_or_default())
                    .unwrap_or(String::from("")).as_str()
                })
                }/>
                </picture>
            </li>
        }
    });

    // Note that if `counter_buttons` were a reactive list
    // and its value changed, this would be very inefficient:
    // it would rerender every row every time the list changed.
    view! { cx,
        <form>
            <input type="file" node_ref=file_input
                on:change=move |ev| {
                    ev.prevent_default();
                    let files = file_input().unwrap().files().map(|x| x.get(0));
                    if let Some(Some(file)) = files {
                         filereader.read_as_binary_string(&file).unwrap();                    }
                    }
            />
            <input type="submit"/>
        </form>
        <ul>{imgs}</ul>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
