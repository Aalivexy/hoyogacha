mod gacha_record;
mod response;
use url::Url;

pub use gacha_record::*;
pub use response::*;

pub(crate) fn check_url(url: Url) -> Option<Url> {
    let response = minreq::get(url.clone()).send().ok()?;
    let json: serde_json::Value = response.json().ok()?;
    if json["retcode"].as_i64() == Some(0) {
        Some(filter_url(url))
    } else {
        None
    }
}

pub(crate) fn filter_url(mut url: Url) -> Url {
    let query = url
        .query_pairs()
        .filter(|(key, _)| {
            matches!(
                key.as_ref(),
                "authkey" | "authkey_ver" | "sign_type" | "game_biz" | "lang"
            )
        })
        .fold(
            url::form_urlencoded::Serializer::new(String::new()),
            |mut serializer, (key, value)| {
                serializer.append_pair(&key, &value);
                serializer
            },
        )
        .finish();
    url.set_query(Some(&query));
    url
}

pub(crate) fn fetch_gacha_log(mut url: Url) -> Result<ResponseData, Box<dyn std::error::Error>> {
    let query = url.query().ok_or("No query found")?.to_string();
    let response = minreq::get(url.clone()).send()?.json::<Response>()?;
    if response.retcode != 0 {
        return Err(response.message.into());
    }

    let mut all_data = response.data.ok_or("No data found")?;
    let mut end_id = all_data.list.last().map(|record| record.id.clone());

    while let Some(id) = end_id {
        url.set_query(Some(&format!("{query}&end_id={id}")));

        let response = minreq::get(url.clone()).send()?.json::<Response>()?;

        if response.retcode != 0 {
            return Err(response.message.into());
        }

        if let Some(mut data) = response.data {
            end_id = data.list.last().map(|record| record.id.clone());
            all_data.list.append(&mut data.list);
        } else {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    all_data.size = all_data.list.len().to_string();
    all_data.page = Some("1".to_string());
    Ok(all_data)
}
