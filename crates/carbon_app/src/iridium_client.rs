pub fn get_client(gdl_base_api: String) -> reqwest_middleware::ClientBuilder {
    use reqwest::{Request, Response};
    use reqwest_middleware::{Middleware, Next};

    use crate::managers::modplatforms::modrinth::MODRINTH_API_BASE;

    struct AddHeaderMiddleware {
        gdl_api_base_host: url::Url,
    };

    let gdl_api_base_host = url::Url::parse(&gdl_base_api).unwrap();

    #[async_trait::async_trait]
    impl Middleware for AddHeaderMiddleware {
        async fn handle(
            &self,
            mut req: Request,
            _extensions: &mut axum::http::Extensions,
            next: Next<'_>,
        ) -> reqwest_middleware::Result<Response> {
            let curseforge_api_base = url::Url::parse(env!(
                "CURSEFORGE_API_BASE",
                "missing curseforge env api base"
            ))
            .unwrap();

            if req.url().host_str() == curseforge_api_base.host_str() {
                req.headers_mut().insert(
                    "x-api-key",
                    option_env!("CURSEFORGE_API_KEY").unwrap().parse().unwrap(),
                );

                req.headers_mut()
                    .insert("Content-Type", "application/json".parse().unwrap());

                req.headers_mut()
                    .insert("Accept", "application/json".parse().unwrap());
            }

            let modrinth_api_base = url::Url::parse(MODRINTH_API_BASE).unwrap();

            if req.url().host_str() == modrinth_api_base.host_str() {
                req.headers_mut()
                    .insert("Content-Type", "application/json".parse().unwrap());

                req.headers_mut()
                    .insert("Accept", "application/json".parse().unwrap());
            }

            // Continue with the modified request.
            next.run(req, _extensions).await
        }
    }

    let client = reqwest::Client::builder()
        .user_agent(format!(
            "{} {}",
            env!("USER_AGENT_PREFIX"),
            env!("APP_VERSION")
        ))
        .build()
        .unwrap();
    reqwest_middleware::ClientBuilder::new(client).with(AddHeaderMiddleware { gdl_api_base_host })
}
