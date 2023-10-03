use js_sys::JsString;
use leptos::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Event, FileReader, HtmlDivElement};
mod image_processing;
use image_processing::{color_filter, decode_raw_image, matrices::MATRICES};

#[component]
fn NavBar() -> impl IntoView {
    let (shown, set_shown) = create_signal(true);
    let (thing_I_like, set_things) = create_signal(0usize);
    const THINGS_I_LIKE: [&'static str; 3] = ["parrots", "cockatoos", "macaws"];
    const COLORS: [&'static str; 3] = ["#ba2f33", "#3bb477", "#3a44bf"];
    view! {
    <nav class="nav-title">
        <div class="explanation">
          <a class="flex-pic" href="https://github.com/carrascomj/dalted">
            <picture class="flex-pic"
              ><img src="gh.svg" alt="github link" class="gh" />
            </picture>
          </a>
          <p class="exp-text">
            Is your image color-friendly?
          </p>
        </div>
        <img src="logo.svg" class="main"/>
        <div class="about">
            <p><a on:click=move |ev|{
                ev.prevent_default();
                set_shown.update(|s| {*s=!*s});
                set_things.update(|i| {*i=if *i < THINGS_I_LIKE.len() - 1 {*i + 1} else { 0}})
            }
            >About</a></p>
        </div>
    </nav>
    <div class="explanation abouting" style:display=move || if shown.get() {"none"} else {"block"}>
        <p> Color blindness simulation </p>
        <p>
            <img class="profile-pic" src="https://avatars.githubusercontent.com/u/46683255?s=96&v=4" alt="github profile picture"/>
            Hi! I am <a href="https://github.com/carrascomj">@carrascomj</a> at Github, this is a client-side webapp using WASM
            with <a href="https://leptos-rs.github.io/leptos">leptos</a>.
            Something I like: <em style:color=move || {COLORS[thing_I_like.get()]}>{move || {THINGS_I_LIKE[thing_I_like.get()]}}</em>.
        </p>
    </div>
    }
}

/// A list of images that to be filled and a form.
#[component]
fn App() -> impl IntoView {
    let (msg, set_msg) = create_signal("Send an Image!");
    let (shared_img, set_image): (ReadSignal<Option<image::DynamicImage>>, _) = create_signal(None);
    // create the file input and file reader to use in the input button
    let file_input = create_node_ref::<html::Input>();
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
        view! {
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

    view! {
        // navegation bar bar
        <NavBar/>
        // form for image submission
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
        // displayed images
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
        App,
    );
}
