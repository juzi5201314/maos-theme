use seed::fetch::{fetch, Status};
use serde::de::DeserializeOwned;
use seed::browser::fetch::FetchError;
use once_cell::sync::Lazy;

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
        fetch(format!("{}{}", &*BASE_URL, path)).await?.check_status()?.json::<T>().await?
    };

    res.map_err(|err| anyhow::Error::msg(format!("fetch error: {:?}", err)))
}
