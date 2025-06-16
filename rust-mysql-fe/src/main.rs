use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
struct User {
    id: i32,
    name: String,
}

#[function_component]
fn App() -> Html {
    let users = use_state(|| vec![]);

    {
        let users = users.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) = Request::get("http://localhost:8080/users")
                    .send()
                    .await
                {
                    if let Ok(fetched) = response.json::<Vec<User>>().await {
                        users.set(fetched);
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div>
            <p class="text-center">
                 <img src="/logo.gif" alt="logo" />
            </p>
            <ul>
                { for users.iter().map(|user| html! {
                    <li>{ format!("{} - {}", user.id, user.name) }</li>
                }) }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}