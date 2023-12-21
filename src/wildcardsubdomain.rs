
use punycode;

pub struct Hostdata {
    pub subdomain: String,
    pub domain: String,
    pub host: String,
    pub decoded_subdomain: String,
}

impl Hostdata {
    pub fn new(host: String, domain: String) -> Hostdata {
        if host.contains(&format!(".{}", domain)) {
            let subdomain = host.replace(&format!(".{}", domain), "");
            Hostdata {
                subdomain: subdomain.clone(),
                domain,
                host,
                decoded_subdomain: Self::decode(subdomain),
            }
        } else {
            Hostdata {
                subdomain: "".to_string(),
                domain,
                host,
                decoded_subdomain: "".to_string(),
            }
        }
    }

    fn decode(subdomain: String) -> String {
        if subdomain.contains("xn--") {
            punycode::decode(&subdomain.replace("xn--", "")).unwrap_or_default()
        } else {
            subdomain
        }
    }

    pub fn create_html(&self) -> String {
        let html = include_str!("../static/index.html.tmpl");
        html.replace("{{ .Title }}", &self.get_title())
            .replace("{{ .Message }}", &self.get_message())
            .replace("{{ .Host }}", &self.host)
    }

    fn get_title(&self) -> String {
        let subdomain = match self.decoded_subdomain.as_str() {
            "jinsei" => "人生",
            "konnendomo" => "今年度も",
            "kotoshimo" => "今年も",
            "kyoumo" => "今日も",
            "" => "",
            _ => &self.decoded_subdomain,
        };
        format!("{}おわりや", subdomain)
    }

    fn get_message(&self) -> String {
        match self.decoded_subdomain.as_str() {
            "jinsei" => "もうだめ".to_string(),
            "konnendomo" => "おめでとうございます".to_string(),
            "kotoshimo" => "あけましておめでとうございます".to_string(),
            "kyoumo" => "一日お疲れ様でした".to_string(),
            "" => "おわりが売ってる".to_string(),
            decoded_subdomain => format!("{}おわりが売ってる", decoded_subdomain),
        }
    }
}
