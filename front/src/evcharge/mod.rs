use leptos::{component, IntoView, view};
use leptos_router::Route;

use crate::Navbar;

#[component(transparent)]
pub fn EvChargeRoutes() -> impl IntoView {
    view! {
        <Route path=".evcharge" view=EvChargeHome>
            <Route path="index" view=EvChargeHome />
        </Route>
    }
}

#[component]
pub fn EvChargeHome() -> impl IntoView {
    view! {
        <Navbar />
        <p class="text-9xl text-sol-100">Hello!</p>
    }
}
