use std::collections::HashMap;
use futures::SinkExt;
use jsonwebtoken::DecodingKey;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    alg: String,
    kty: String,
    n: String,
    e: String,
}

pub async fn fetch_jwk_set(keycloak_url: &str) -> Result<HashMap<String, DecodingKey>, Box<dyn std::error::Error>> {
    let url = format!("{}/protocol/openid-connect/certs", keycloak_url);
    let client = Client::new();
    let res: JwkSet = client.get(&url).send().await?.json().await?;
    println!("{:#?}", res);
    let mut keys = HashMap::new();
    for key in res.keys {
        if key.alg == "RS256" {
            let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e).expect("Couldn't create decoding key");
            keys.insert(key.kid, decoding_key);
        }
    }

    Ok(keys)
}