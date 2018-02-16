use reqwest::get;
use reqwest::StatusCode::NotFound;
use percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};

pub use reqwest::Result;

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
    pub difino: String,
    pub pludifinoj: Vec<Pludifino>,
    pub ekzemploj: Vec<Ekzemplo>,
    pub tradukoj: Vec<Traduko>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pludifino {
    pub difino: String,
    pub ekzemploj: Vec<Ekzemplo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ekzemplo {
    pub ekzemplo: String,
    pub fonto: Option<String>,
}

pub fn trovi<S>(vorto: S) -> Result<Trovo>
where
    S: AsRef<str>
{
    let vorto = utf8_percent_encode(vorto.as_ref(), PATH_SEGMENT_ENCODE_SET);
    let url = format!("http://www.simplavortaro.org/api/v1/trovi/{}", vorto);
    
    get(&url)?.error_for_status()?.json()
}

pub fn vorto<S>(vorto: S) -> Result<Option<Vorto>>
where
    S: AsRef<str>
{
    let vorto = utf8_percent_encode(vorto.as_ref(), PATH_SEGMENT_ENCODE_SET);
    let url = format!("http://www.simplavortaro.org/api/v1/vorto/{}", vorto);
    
    let res = get(&url)?;
    
    if res.status() == NotFound {
        return Ok(None)
    }

    res.error_for_status()?.json()
}
