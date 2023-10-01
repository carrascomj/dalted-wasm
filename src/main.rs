use js_sys::JsString;
use leptos::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Event, FileReader, HtmlDivElement};
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

/// A list of images that will be filled with images.
#[component]
fn App(cx: Scope) -> impl IntoView {
    let (msg, set_msg) = create_signal(cx, "Send an Image!");
    let (shared_img, set_image): (ReadSignal<Option<image::DynamicImage>>, _) =
        create_signal(cx, None);
    // create the file input and file reader to use in the input button
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

    // once shared_img changes, compute each image transformation
    let imgs = MATRICES.map(|mat| {
        view! { cx,
                <picture class="flex-pic">
                <img class="flex-img" src={move || shared_img.with(|im|
                    {
                    String::from("data:image/png;base64,") + im.as_ref().ok_or(0)
                        .map(|i| color_filter(i, mat).unwrap_or_default())
                        .unwrap_or(String::from("")).as_str()
                })
                }/>
                </picture>
        }
    });

    // form to submit the image and img reactive frames
    view! { cx,

        <div class="user-sweep" id="box">
        <div class="upload-btn-wrapper">

        <p>{msg}</p>
        <form>
            <input type="file" node_ref=file_input
                on:change=move |ev| {
                    ev.prevent_default();
                    let files = file_input().unwrap().files().map(|x| x.get(0));
                    if let Some(Some(file)) = files {
                         set_msg("Processing!");
                         filereader.read_as_binary_string(&file).unwrap();                    }
                    }
            />

        </form>
        </div>
        </div>
        <div class="portrait">
            {imgs}
        </div>
    }
}

fn main() {
    leptos::mount_to(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("entrypoint")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap()
            .into(),
        |cx| view! { cx, <App/> },
    );
}
