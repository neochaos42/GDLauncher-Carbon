// allow dead code during development to keep warning outputs meaningful
#![allow(warnings)]
#![allow(dead_code)]

use crate::managers::{
    java::{
        discovery::{Discovery, RealDiscovery},
        java_checker::RealJavaChecker,
    },
    App, AppInner,
};
use serde_json::Value;
use std::{path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub mod api;
mod app_version;
pub mod cache_middleware;
pub mod domain;
mod error;
pub mod iridium_client;
mod livenesstracker;
pub mod managers;
mod platform;
// mod pprocess_keepalive;
mod base_api_override;
mod logger;
mod once_send;
mod runtime_path_override;
mod util;

pub fn main() {
    // pprocess_keepalive::init();
    #[cfg(debug_assertions)]
    {
        let mut args = std::env::args();
        if args.any(|arg| arg == "--generate-ts-bindings") {
            crate::api::build_rspc_router()
                .config(
                    rspc::Config::new().export_ts_bindings(
                        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                            .parent()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .join("packages")
                            .join("core_module")
                            .join("bindings.d.ts"),
                    ),
                )
                .build();

            // exit process with ok status
            std::process::exit(0);
        }
    }

    #[cfg(feature = "production")]
    #[cfg(not(test))]
    let sentry_session_id = &uuid::Uuid::new_v4().to_string();

    #[cfg(feature = "production")]
    #[cfg(not(test))]
    let _guard = {
        let s = sentry::init((
            env!("CORE_MODULE_DSN"),
            sentry::ClientOptions {
                release: Some(app_version::APP_VERSION.into()),
                ..Default::default()
            },
        ));

        sentry::configure_scope(|scope| {
            scope.set_tag("gdl_session_id", &sentry_session_id);
        });

        s
    };

    let x = 1;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(256)
        .build()
        .unwrap()
        .block_on(async {
            daedalus::Branding::set_branding(daedalus::Branding::new(
                "gdlauncher".to_string(),
                "".to_string(),
            ))
            .expect("Branding not to fail");

            #[cfg(feature = "production")]
            iridium::startup_check();

            info!("Initializing runtime path");
            let runtime_path = runtime_path_override::get_runtime_path_override().await;
            let base_api_override = base_api_override::get_base_api_override().await;

            let _guard = logger::setup_logger(&runtime_path).await;

            info!("Starting Carbon App v{}", app_version::APP_VERSION);

            #[cfg(feature = "production")]
            #[cfg(not(test))]
            info!("Sentry Session Id: {}", sentry_session_id);

            info!("Runtime path: {}", runtime_path.display());

            info!("Scanning ports");

            let init_time = std::time::Instant::now();

            let listener = if cfg!(debug_assertions) {
                TcpListener::bind("127.0.0.1:4650").await.unwrap()
            } else {
                get_available_port().await
            };

            info!(
                "Found port: {:?} in {:?}",
                listener.local_addr(),
                init_time.elapsed()
            );

            start_router(runtime_path, base_api_override, listener).await;
        });
}

async fn get_available_port() -> TcpListener {
    info!("Scanning for available port");
    for port in 1025..65535 {
        let conn = TcpListener::bind(format!("127.0.0.1:{port}")).await;
        match conn {
            Ok(listener) => return listener,
            Err(_) => continue,
        }
    }

    info!("No available port found");

    panic!("No available port found");
}

async fn start_router(runtime_path: PathBuf, base_api_override: String, listener: TcpListener) {
    info!("Starting router");
    let (invalidation_sender, _) = tokio::sync::broadcast::channel(1000);

    let router: Arc<rspc::Router<App>> = crate::api::build_rspc_router().build().arced();

    // We disable CORS because this is just an example. DON'T DO THIS IN PRODUCTION!
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = AppInner::new(invalidation_sender, runtime_path, base_api_override).await;

    let auto_manage_java_system_profiles = app
        .settings_manager()
        .get_settings()
        .await
        .unwrap()
        .auto_manage_java_system_profiles;

    crate::managers::java::JavaManager::scan_and_sync(
        auto_manage_java_system_profiles,
        &app.prisma_client,
        &RealDiscovery::new(app.settings_manager().runtime_path.clone()),
        &RealJavaChecker,
    )
    .await
    .expect("Failed to scan and sync java system profiles");

    let app1 = app.clone();
    let app2 = app.clone();
    let rspc_axum_router: axum::Router<Arc<AppInner>> = rspc_axum::endpoint(router, move || app);

    let app = axum::Router::new()
        .nest("/", crate::api::build_axum_vanilla_router())
        .nest("/rspc", rspc_axum_router)
        .layer(cors)
        .with_state(app1);

    let port = listener.local_addr().unwrap().port();

    // As soon as the server is ready, notify via stdout
    tokio::spawn(async move {
        let mut counter = 0;
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(200));
        let reqwest_client = reqwest::Client::new();
        loop {
            counter += 1;
            // If we've waited for 40 seconds, give up
            if counter > 200 {
                panic!("Server failed to start in time");
            }

            interval.tick().await;
            let res = reqwest_client
                .get(format!("http://127.0.0.1:{port}/health"))
                .send()
                .await;

            if res.is_ok() {
                info!("_STATUS_:READY|{port}");
                println!("_STATUS_:READY|{port}");
                break;
            }
        }
    });

    let _app = app2.clone();
    tokio::spawn(async move {
        _app.meta_cache_manager().launch_background_tasks().await;
        _app.clone()
            .instance_manager()
            .launch_background_tasks()
            .await;
    });

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
struct TestEnv {
    tmpdir: PathBuf,
    //log_guard: tracing_appender::non_blocking::WorkerGuard,
    app: App,
    invalidation_recv: tokio::sync::broadcast::Receiver<api::InvalidationEvent>,
}

#[cfg(test)]
impl TestEnv {
    async fn restart_in_place(&mut self) {
        let (invalidation_sender, _) = tokio::sync::broadcast::channel(200);
        self.app = AppInner::new(
            invalidation_sender,
            self.tmpdir.clone(),
            crate::util::base_api::get_base_api_env!(),
        )
        .await;
    }
}

#[cfg(test)]
impl std::ops::Deref for TestEnv {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

// #[cfg(test)]
// impl Drop for TestEnv {
//     fn drop(&mut self) {
//         let _ = std::fs::remove_dir_all(&self.tmpdir);
//     }
// }

#[cfg(test)]
async fn setup_managers_for_test() -> TestEnv {
    let temp_dir = tempdir::TempDir::new("carbon_app_test").unwrap();
    let temp_path = dunce::canonicalize(temp_dir.into_path()).unwrap();
    //let log_guard = logger::setup_logger(&temp_path).await;
    println!("Test RTP: {}", temp_path.to_str().unwrap());
    let (invalidation_sender, invalidation_recv) = tokio::sync::broadcast::channel(200);

    TestEnv {
        tmpdir: temp_path.clone(),
        // log_guard,
        invalidation_recv,
        app: AppInner::new(
            invalidation_sender,
            temp_path,
            crate::util::base_api::get_base_api_env!(),
        )
        .await,
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_eq_display {
    ($a:expr, $b:expr) => {
        if $a != $b {
            panic!(
                "Assertion failed: left == right\nleft:\n{a_val}\nright:\n{b_val}",
                a_val = $a,
                b_val = $b,
            );
        }
    };
}

#[macro_export]
macro_rules! mirror_into {
    ($a:path, $b:path, |$value:ident| $expr:expr) => {
        impl From<$a> for $b {
            fn from($value: $a) -> Self {
                use $a as Other;

                $expr
            }
        }

        impl From<$b> for $a {
            fn from($value: $b) -> Self {
                use $b as Other;

                $expr
            }
        }
    };
}

#[cfg(test)]
mod test {
    use crate::get_available_port;

    #[tokio::test]
    async fn test_router() {
        let tcp_listener = get_available_port().await;
        let port = &tcp_listener.local_addr().unwrap().port();
        let temp_dir = tempdir::TempDir::new("carbon_app_test").unwrap();
        let server = tokio::spawn(async move {
            super::start_router(
                temp_dir.into_path(),
                crate::util::base_api::get_base_api_env!(),
                tcp_listener,
            )
            .await;
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("http://127.0.0.1:{port}",))
            .send()
            .await
            .unwrap();
        let resp_code = resp.status();
        let resp_body = resp.text().await.unwrap();

        assert_eq!(resp_code, 200);
        assert_eq!(resp_body, "Hello 'rspc'!");

        server.abort();
    }
}
