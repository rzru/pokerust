use crate::http::Http;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

pub async fn fetch_external<T, R: 'static>(data: &[T], fetch_url: fn(&T) -> String) -> Vec<R>
where
    R: DeserializeOwned + Send + Debug,
{
    let mut res = vec![];
    let (tx, mut rx) = mpsc::channel(32);

    for item in data {
        let url = fetch_url(item);
        let tx = tx.clone();
        spawn_fetcher(url, tx).await;
    }

    drop(tx);

    while let Some(message) = rx.recv().await {
        res.push(message)
    }

    res
}

pub async fn spawn_fetcher<T: 'static>(url: String, tx: Sender<T>)
where
    T: DeserializeOwned + Send + Debug,
{
    tokio::spawn(async move {
        let http = Http::new();
        let data = http.get(&url).await;

        if let Some(bytes) = data {
            let fetched = serde_json::from_slice(&bytes).unwrap();

            tx.send(fetched).await.unwrap();
        }
    });
}
