use std::path::{Path, PathBuf};

use leptos::{component, IntoView, template, view, For, IntoAttribute, Children, server, ServerFnError, Await, ErrorBoundary, Suspense, SignalGet};
use serde::{Deserialize, Serialize};

use crate::front::Navbar;

#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectSpecification {
    name: String,
    svg: String,
    desc: String,
    href: String,
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = leptos::create_resource(
        ||(),
        |_| async move {
            get_projects().await
        }
    );

    view! {
        <Navbar />
        <div class="max-w-8xl mx-auto">
            <ul class="grid grid-cols-3 mt-32 gap-x-6 gap-y-7 place-items-center">
                <Suspense
                    fallback = move || view! {}
                >
                    <ErrorBoundary
                        fallback=|_| view! { <p>Errors</p>}
                    >
                        {
                            projects
                                .get()
                                .map(|projects| projects
                                    .map(|projects| projects
                                        .into_iter()
                                        .map(|proj| view! { <ProjectCard project = proj /> })
                                        .collect::<Vec<_>>()
                                    )
                                )
                        }
                    </ErrorBoundary>
                </Suspense>
            </ul>
        </div>
    }
}

pub const TESLACHARGE_PATH: &str = include_str!("../embed/projects/teslacharge.path.txt");

#[component]
fn ProjectCard(project: ProjectSpecification) -> impl IntoView {
    view! {
        <li class="group relative w-full h-full bg-night-200 hover:bg-night-100 border-t-2 border-t-night-50 hover:shadow-xl hover:shadow-night-400 transition ease-in-out rounded-md">
            <a href={project.href}>
                <div class="grid grid-cols-1 grid-rows-2 aspect-square relative w-full">
                    <div>
                        <svg class="inset-0 mx-auto w-full h-full pt-10 fill-sol-100" viewBox="0 0 32 32">
                            <path d={project.svg} />
                        </svg>
                    </div>
                    <div class="self-end py-4 mx-auto w-10/12">
                        <h2 class="text-4xl text-center w-full font-mono text-earth-200">{project.name}</h2>
                        <p class="text-base font-mono text-mercury-200 pt-5">{project.desc}</p>
                    </div>
                </div>
            </a>
        </li>
    }
}

#[server(GetProjectsList, "/api")]
async fn get_projects() -> Result<Vec<ProjectSpecification>, ServerFnError> {
    Ok(vec![
        ProjectSpecification {
            name: String::from("Tesla Solar Charging"),
            svg: String::from(TESLACHARGE_PATH),
            desc: String::from("Optimize solar charging"),
            href: String::from("/"),
        }
    ])
}
