#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use axum::{
        http::{Request, Response},
        Router,
    };
    use blog::{app::*, server::utils::db::init_db};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::trace::{self, MakeSpan, OnRequest, OnResponse, TraceLayer};
    use tracing::{info, Level, Span};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    #[derive(Clone)]
    struct FilteredMakeSpan {
        inner: trace::DefaultMakeSpan,
    }

    impl FilteredMakeSpan {
        fn new() -> Self {
            Self {
                inner: trace::DefaultMakeSpan::new().level(Level::INFO),
            }
        }
    }

    impl<B> MakeSpan<B> for FilteredMakeSpan {
        fn make_span(&mut self, request: &Request<B>) -> Span {
            let path = request.uri().path();

            // Skip span creation for /pkg/* routes by returning an empty span
            if path.starts_with("/pkg/") {
                return tracing::Span::none();
            }

            if path.starts_with("/favicon.ico") {
                return tracing::Span::none();
            }

            // Use the default implementation for all other routes
            self.inner.make_span(request)
        }
    }

    #[derive(Clone)]
    struct FilteredOnRequest {
        inner: trace::DefaultOnRequest,
    }

    impl FilteredOnRequest {
        fn new() -> Self {
            Self {
                inner: trace::DefaultOnRequest::new().level(Level::INFO),
            }
        }
    }

    impl<B> OnRequest<B> for FilteredOnRequest {
        fn on_request(&mut self, request: &Request<B>, _span: &Span) {
            let path = request.uri().path();
            if path.starts_with("/pkg/") {
                return;
            }
            self.inner.on_request(request, _span);
        }
    }

    #[derive(Clone)]
    struct FilteredOnResponse {
        inner: trace::DefaultOnResponse,
    }

    impl FilteredOnResponse {
        fn new() -> Self {
            Self {
                inner: trace::DefaultOnResponse::new().level(Level::INFO),
            }
        }
    }

    impl<B> OnResponse<B> for FilteredOnResponse {
        fn on_response(self, response: &Response<B>, latency: Duration, span: &Span) {
            // We can't easily access the request path here, but if a span is none(),
            // we know it came from a filtered route
            if span.is_none() {
                return;
            }
            self.inner.on_response(response, latency, span);
        }
    }

    // Initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing database");
    init_db().await.expect("Failed to initialize database");
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Set up the HTTP request trace middleware
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(FilteredMakeSpan::new())
        .on_request(FilteredOnRequest::new())
        .on_response(FilteredOnResponse::new());

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(trace_layer)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
