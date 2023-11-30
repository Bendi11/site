use leptos::{component, IntoView, view, create_signal, SignalSet, SignalUpdate};
use leptos_meta::{Stylesheet, Title};
use leptos_router::{Router, Routes, Route};

#[component]
pub fn Site() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="dist/bkl-dev.css" />
        <Title text="Ben's Place" />

        <Router>
            <main>
                <Routes>
                    <Route path="" view=Home />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let (val, set_val) = create_signal(0);

    view! {
        <h1><b>BOLD HEADER</b></h1>
        <div>
            <span> Hello, World! YAAAY </span>
            <span>"The Value is: " {val}</span> 
            <div>
                <button on:click = move |_| set_val.set(0)>"Clear"</button>
                <button on:click = move |_| set_val.update(|v| *v += 1)>"Add One"</button>
                <button on:click = move |_| set_val.update(|v| *v -= 1)>"Subtract One"</button>
            </div>
        </div>
    }
}
