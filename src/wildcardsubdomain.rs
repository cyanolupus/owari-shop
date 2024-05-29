use serde_json::{from_str, Value};

struct ThreeTuple {
    title: String,
    message: String,
    emoji: String,
}

impl ThreeTuple {
    fn new(title: String, message: String, emoji: String) -> ThreeTuple {
        ThreeTuple {
            title,
            message,
            emoji,
        }
    }

    fn get_str_from_json(json: &Value, key: &str) -> Option<String> {
        json.get(key)
            .and_then(|value| value.as_str().map(|s| s.to_string()))
    }

    fn get_from_json(json: &Value, key: &str) -> ThreeTuple {
        let default = ThreeTuple::get_default_from_json(json);
        match json.get(key) {
            Some(value) => ThreeTuple::new(
                Self::get_str_from_json(value, "title")
                    .unwrap_or(format!("{}{}", key, default.title)),
                Self::get_str_from_json(value, "message")
                    .unwrap_or(format!("{}{}", key, default.message)),
                Self::get_str_from_json(value, "emoji").unwrap_or(default.emoji),
            ),
            _ => ThreeTuple::new(
                format!("{}{}", key, default.title),
                format!("{}{}", key, default.message),
                default.emoji,
            ),
        }
    }

    fn get_default_from_json(json: &Value) -> ThreeTuple {
        let value = json.get("default").unwrap();
        ThreeTuple::new(
            Self::get_str_from_json(value, "title").unwrap_or_default(),
            Self::get_str_from_json(value, "message").unwrap_or_default(),
            Self::get_str_from_json(value, "emoji").unwrap_or_default(),
        )
    }
}

impl Clone for ThreeTuple {
    fn clone(&self) -> Self {
        ThreeTuple {
            title: self.title.clone(),
            message: self.message.clone(),
            emoji: self.emoji.clone(),
        }
    }
}

pub struct Hostdata {
    domain: String,
    host: String,
    pub decoded_subdomain: String,
    three_tuple: ThreeTuple,
}

impl Hostdata {
    pub fn new(host: String, domain: String) -> Hostdata {
        let pattern = format!(".{}", domain);
        let subdomain = if host.contains(&pattern) {
            host.replace(&pattern, "")
        } else {
            "".to_string()
        };
        let decoded_subdomain = Self::decode(subdomain.clone());

        let json_str = include_str!("../static/3tuples.json");
        let json: Value = from_str(json_str).unwrap_or_default();
        let three_tuple = ThreeTuple::get_from_json(&json, &decoded_subdomain);

        Hostdata {
            domain,
            host,
            decoded_subdomain,
            three_tuple,
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
            &format!("{}{}", self.three_tuple.title, title_suffix),
        )
        .replace("{{ .Message }}", &self.three_tuple.message)
        .replace("{{ .Host }}", &self.host)
        .replace("{{ .Emoji }}", &self.three_tuple.emoji)
        .replace("{{ .Domain }}", &self.domain)
    }
}
