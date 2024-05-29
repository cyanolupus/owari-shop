use serde_json::{from_str, Value};

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

    pub fn create_html(&self, title_suffix: String) -> String {
        let html = include_str!("../static/index.html.tmpl");
        html.replace(
            "{{ .Title }}",
            &format!("{}{}", self.get_title(), title_suffix),
        )
        .replace("{{ .Message }}", &self.get_message())
        .replace("{{ .Host }}", &self.host)
        .replace("{{ .Emoji }}", &self.get_emoji())
        .replace("{{ .Domain }}", &self.domain)
    }

    fn get_title(&self) -> String {
        self.get_3tuple().0
    }

    fn get_message(&self) -> String {
        self.get_3tuple().1
    }

    fn get_emoji(&self) -> String {
        self.get_3tuple().2
    }

    fn get_3tuple(&self) -> (String, String, String) {
        let json_str = include_str!("../static/3tuples.json");
        let json: Value = from_str(json_str).unwrap_or_default();

        let default_3tuple = match json.get("default") {
            Some(value) => {
                let message = value
                    .get("message")
                    .unwrap_or(&Value::Null)
                    .as_str()
                    .unwrap_or_default();
                let emoji = value
                    .get("emoji")
                    .unwrap_or(&Value::Null)
                    .as_str()
                    .unwrap_or_default();
                (
                    self.decoded_subdomain.clone(),
                    format!("{}{}", self.decoded_subdomain, message),
                    emoji.to_string(),
                )
            }
            _ => (
                self.decoded_subdomain.clone(),
                format!("{}おわりが売ってる", self.decoded_subdomain),
                "✅".to_string(),
            ),
        };

        match json.get(&self.decoded_subdomain) {
            Some(value) => {
                let title = value
                    .get("title")
                    .unwrap_or(&Value::Null)
                    .as_str()
                    .unwrap_or(default_3tuple.0.as_str());
                let message = value
                    .get("message")
                    .unwrap_or(&Value::Null)
                    .as_str()
                    .unwrap_or(default_3tuple.1.as_str());
                let emoji = value
                    .get("emoji")
                    .unwrap_or(&Value::Null)
                    .as_str()
                    .unwrap_or(default_3tuple.2.as_str());
                (title.to_string(), message.to_string(), emoji.to_string())
            }
            _ => default_3tuple,
        }
    }
}
