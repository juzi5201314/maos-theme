use once_cell::sync::Lazy;
use seed::fetch::{fetch, FetchError, Method, Request};
use serde::de::DeserializeOwned;
use serde::Serialize;
use web_sys::RequestCredentials;
use web_sys::RequestMode;

static BASE_URL: Lazy<String> = Lazy::new(|| {
    String::from(if let Some(url) = option_env!("BASE_URL") {
        url
    } else {
        "/api/"
    })
});

pub async fn get<T>(path: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned + 'static,
{
    let res: Result<T, FetchError> = try {
        fetch(
            Request::new(format!("{}{}", &*BASE_URL, path))
                .mode(RequestMode::Cors)
                .credentials(RequestCredentials::Include)
                .method(Method::Get),
        )
        .await?
        .check_status()?
        .json::<T>()
        .await?
    };

    res.map_err(|err| anyhow::Error::msg(format!("fetch error: {:?}", err)))
}

pub async fn post<D, T>(path: &str, data: &D) -> seed::fetch::Result<T>
where
    D: Serialize,
    T: DeserializeOwned + 'static,
{
    fetch(
        Request::new(format!("{}{}", &*BASE_URL, path))
            .method(Method::Post)
            .mode(RequestMode::Cors)
            .credentials(RequestCredentials::Include)
            .json(data)?,
    )
    .await?
    .check_status()?
    .json::<T>()
    .await
}
