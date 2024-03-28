use anyhow::Result;
use leptos::*;

async fn fetch_message() -> Result<String> {
    let response = reqwest::get("http://localhost:8080/api/message").await?;
    let body = response.text().await?;
    let body = serde_json::from_str::<shared::MessageDto>(&body)?;
    Ok(body.text)
}

#[component]
pub fn App() -> impl IntoView {
    let message = create_resource(|| (), |_| async move {
        fetch_message().await.unwrap_or_else(|e| e.to_string())
    });

    view! {
        <div>
            <h1>{ "Hello, World!" }</h1>
        </div>
        <Suspense fallback=move || view!{ <p> "Loading..." </p> }>
            <p>{ move || message.get() }</p>
        </Suspense>
    }
}
