use leptos::{component, IntoView, view};
use leptos_router::Route;

#[component(transparent)]
pub fn EvChargeRoutes() -> impl IntoView {
    view! {
        <Route path=".evcharge" view=EvChargeHome>
        </Route>
    }
}

#[component]
pub fn EvChargeHome() -> impl IntoView {
    view! {
        <p class="text-9xl text-sol-100">Hello!</p>
    }
}
