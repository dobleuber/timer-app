use leptos::prelude::*;

use crate::app::clock::Clock;
use crate::app::controls::Controls;
use crate::app::rounds::Rounds;
use crate::app::settings::Settings;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Tabata Timer!"</h1>
        <h2>"Hola mundo!"</h2>
        <div class="timer-container">
            <Clock/>
            <Rounds />
            <Controls />
        </div>
        <div class="settings">
            <Settings />
        </div>
    }
}
