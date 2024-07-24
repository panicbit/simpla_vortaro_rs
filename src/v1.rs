use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{Client, StatusCode};

pub use reqwest::Result;

use crate::USER_AGENT;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trovo {
    pub preciza: Vec<String>,
    pub malpreciza: Vec<String>,
    pub vortfarado: Vec<Vortfarado>,
    pub tradukoj: Vec<Traduko>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vortfarado {
    pub partoj: Vec<Parto>,
    pub rezulto: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parto {
    pub vorto: Option<String>,
    pub parto: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Traduko {
    pub kodo: String,
    pub vorto: Option<String>,
    pub lingvo: String,
    pub traduko: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vorto {
    pub vorto: String,
    pub difinoj: Vec<Difino>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Difino {
    pub difino: Option<String>,
    pub pludifinoj: Vec<Pludifino>,
    pub ekzemploj: Vec<Ekzemplo>,
    pub tradukoj: Vec<Traduko>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pludifino {
    pub difino: Option<String>,
    pub ekzemploj: Vec<Ekzemplo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ekzemplo {
    pub ekzemplo: String,
    pub fonto: Option<String>,
}

pub async fn trovi<S>(vorto: S) -> Result<Trovo>
where
    S: AsRef<str>,
{
    let vorto = utf8_percent_encode(vorto.as_ref(), NON_ALPHANUMERIC);
    let url = format!("http://www.simplavortaro.org/api/v1/trovi/{}", vorto);

    let kliento = Client::builder().user_agent(USER_AGENT).build()?;
    let respondo = kliento.get(&url).send().await?.error_for_status()?;

    let trovo = respondo.json::<Trovo>().await?;

    Ok(trovo)
}

pub async fn vorto<S>(vorto: S) -> Result<Option<Vorto>>
where
    S: AsRef<str>,
{
    let vorto = utf8_percent_encode(vorto.as_ref(), NON_ALPHANUMERIC);
    let url = format!("http://www.simplavortaro.org/api/v1/vorto/{}", vorto);

    let kliento = Client::builder().user_agent(USER_AGENT).build()?;
    let respondo = kliento.get(&url).send().await?;

    if respondo.status() == StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let respondo = respondo.error_for_status()?;
    let vorto = respondo.json::<Option<Vorto>>().await?;

    Ok(vorto)
}
