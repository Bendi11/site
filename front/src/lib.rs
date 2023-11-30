use leptos::{component, IntoView, view, create_signal, SignalSet, SignalUpdate, IntoAttribute};
use leptos_meta::{Stylesheet, Title, Body};
use leptos_router::{Router, Routes, Route};

use evcharge::EvChargeRoutes;

mod evcharge;

#[component]
pub fn Site() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="dist/bkl-dev.css" />
        <Title text="Ben's Place" />
        <Body class="dark:bg-night-300" />
        <Router>
            <main>
                <Routes>
                    <EvChargeRoutes />
                    <Route path="" view=Homepage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <Navbar/>

        <div class="max-w-7xl mx-auto">
        </div>
    }
}

const GITHUB_PATH: &'static str = include_str!("./embed/github.path.txt");

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="sticky top-0 flex-none w-full z-40 py-3 border-b border-b-night-100 bg-night-400">
            <div class="max-w-7xl mx-auto">
                //Flexbox that actually positions the navbar elements
                <div class="relative flex items-center">
                    <a href="/">
                        <img src="/img/icon.png" alt="Cat Icon" class="h-16 aspect-square rounded-full transition ease-in-out duration-400 hover:ring-2 hover:ring-offset-2 hover:ring-offset-night-400 hover:ring-sol-100" />
                    </a>
                    <div class="w-7"/>
                    <div>
                        <p class="text-4xl text-sol-100 font-mono select-none dro-shadow-2xl">"Ben's Place"</p>
                        <table class="w-full h-1 -translate-y-0">
                            <tr>
                                <td class="w-1/5 bg-sol-100 rounded-l-lg"/>
                                <td class="w-1/5 bg-mercury-400"/>
                                <td class="w-1/5 bg-venus-200"/>
                                <td class="w-1/5 bg-earth-100"/>
                                <td class="w-1/5 bg-mars-300 rounded-r-lg"/>
                            </tr>
                        </table>
                    </div>
                    
                    //Right side navbar content
                    <div class="relative hidden lg:flex items-center ml-auto">
                        <div class="h-full hover:bg-night-100">
                            <a class="hover:text-mercury-100 text-mercury-400 font-mono" href="/projects">"Projects"</a>
                        </div>
                        <a class="ml-10 block" href="https://github.com/bendi11">
                            <span class="sr-only">"Github icon"</span>
                            <svg class="w-9 h-9 fill-mercury-400 hover:fill-mercury-100" viewBox="0 0 16 16">
                                <path d={GITHUB_PATH} />
                            </svg>
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
