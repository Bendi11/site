use leptos::{component, IntoView, view, create_signal, SignalSet, SignalUpdate};
use leptos_meta::{Stylesheet, Title, Body};
use leptos_router::{Router, Routes, Route};

#[component]
pub fn Site() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="dist/bkl-dev.css" />
        <Title text="Ben's Place" />
        <Body class="dark:bg-night-400" />
        <Router>
            <main>
                <Routes>
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
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="sticky top-0 flex-none w-full z-40 py-3 border-b border-b-night-100">
            <div class="max-w-7xl mx-auto">
                //Flexbox that actually positions the navbar elements
                <div class="relative flex items-center">
                    <a href="/">
                        <img src="/img/icon.png" alt="Cat Icon" class="h-16 aspect-square rounded-full" />
                    </a>
                    <div class="w-7"/>
                    <div>
                        <p class="text-4xl text-sol-100 font-mono">"Ben's Place"</p>
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
                        <a class="ml-10 block text-mercury-400 hover:text-mercury-100" href="https://github.com/bendi11">
                            <svg class="w-9 h-9" viewBox="0 0 16 16" fill="currentColor">
                                <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
                            </svg>
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
