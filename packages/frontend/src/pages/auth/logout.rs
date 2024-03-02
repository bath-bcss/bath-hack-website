use gloo_console::log;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{data::auth::sign_out, router::Route};

#[function_component(LogoutPage)]
pub fn logout_page() -> Html {
    let navigator = use_navigator().expect_throw("Navigator not found");

    use_effect_with((), |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let result = sign_out().await;
            match result {
                Err(e) => log!(e.to_string()),
                Ok(_) => navigator.replace(&Route::Login)
            }
        })
    });

    html!{
        <p>{"Signing out..."}</p>
    }
}
