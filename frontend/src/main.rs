use leptos::*;

fn main() {
    let text = shared::get_greeting();
    mount_to_body(|| view! { <p>{text}</p> })
}
