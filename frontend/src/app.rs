use anyhow::{anyhow, Result};
use leptos::*;

fn get_path_for(endpoint: &str) -> Result<String> {
    let window = web_sys::window().ok_or(anyhow!("cannot retrieve window object"))?;
    let host = window.location().origin().map_err(|_| anyhow!("cannot get origin"))?;
    Ok(format!("{}/{}", host, endpoint.trim_start_matches('/')))
}

async fn fetch_message() -> Result<String> {
    let response = reqwest::get(get_path_for("/api/message")?).await?;
    if response.status() != 200 {
        return Err(anyhow!("Failed to fetch message, error code {}.", response.status().as_str()));
    }
    let body = response.text().await?;
    let body = serde_json::from_str::<shared::MessageDto>(&body)?;
    Ok(body.text)
}

#[component]
pub fn App() -> impl IntoView {
    let message = create_resource(|| (), |_| async move {
        fetch_message().await.map_err(|e| e.to_string())
    });

    view! {
        <div>
            <h1>{ "Hello, World!" }</h1>
        </div>
        <Suspense fallback=move || view!{ <p> "Loading..." </p> }>
            { move || match message().unwrap_or(Ok("Loading ...".to_string())) {
                Ok(text) => view!{ <p>{ text }</p> },
                Err(e) => view!{ <p style:color="red" >{ e }</p> },
            }}
        </Suspense>
    }
}
