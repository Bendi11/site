mod front;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use leptos_axum::LeptosRoutes;
    use axum::{Router, routing::post};
    use front::Site;
    use tower::ServiceBuilder;
    use tower_http::{services::ServeDir, compression::CompressionLayer};

    let conf = leptos::get_configuration(None).await.unwrap();
    let routes = leptos_axum::generate_route_list(Site);
    
    let opts = conf.leptos_options;
    let addr = opts.site_addr;

    let app = Router::new()
        .fallback_service(ServiceBuilder::new()
            .layer(CompressionLayer::new())
            .service(ServeDir::new(opts.site_root.clone()))
        )
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&opts, routes, Site)
        .with_state(opts);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {}
