use leptos::prelude::*;

#[component]
pub fn Clock() -> impl IntoView {
    view! {
        <div class="timer">"00:00"</div>
    }
}
