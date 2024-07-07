use std::{collections::HashMap, fs::read_to_string, hash::Hash, vec};

use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    private::{Privatable, Private, UnPrivatable},
    Error,
};

#[derive(Debug)]
pub struct Config {
    pub remote_server_password: Private<String>,
    pub remote_server_name: String,
    pub remote_server_port: u16,
    pub forward_ports: HashMap<String, PortForward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForward {
    pub local_port: u16,
    pub remote_port: u16,
    pub forward_type: ForwardType,
}

pub async fn load_config() -> Result<Config, crate::Error> {
    let config: toml::Table = match toml::from_str(
        tokio::fs::read_to_string("./frpc.toml")
            .await
            .unwrap()
            .as_str(),
    ) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            panic!()
        }
    };

    let mut common = None;
    let forwarded_ports = config
        .into_iter()
        .map(|(section, properties)| {
            if section == "common" {
                common = Some(properties);
                return Ok((section, None));
            }
            let section_ref = &section;
            Ok((
                section_ref.to_owned(),
                Some(PortForward {
                    local_port: properties
                        .get("local_port")
                        .ok_or_else(|| {
                            crate::Error::ConfigError(format!("local port for {section} missing"))
                        })?
                        .as_integer()
                        .ok_or_else(|| {
                            crate::Error::ConfigError(format!("local port for {section} is NaN"))
                        })? as u16,
                    remote_port: properties
                        .get("remote_port")
                        .ok_or_else(|| {
                            crate::Error::ConfigError(format!("remote port for {section} missing"))
                        })?
                        .as_integer()
                        .ok_or_else(|| {
                            crate::Error::ConfigError(format!(
                                "local port for {section} isnt a number"
                            ))
                        })? as u16,
                    forward_type: ForwardType::from(
                        properties
                            .get("type")
                            .ok_or_else(|| {
                                crate::Error::ConfigError(format!("type for {section} missing"))
                            })?
                            .as_str()
                            .ok_or_else(|| {
                                crate::Error::ConfigError(format!(
                                    "type for {section} isnt a string"
                                ))
                            })?,
                    )?,
                }),
            ))
        })
        .filter_map(
            |x: Result<(String, Option<PortForward>), crate::Error>| match x {
                Ok((section, Some(forward))) => Some(Ok((section, forward))),
                Err(rest) => Some(Err(rest)),
                _ => None,
            },
        )
        .collect::<Result<HashMap<_, _>, _>>()?;

    let common_section = common.ok_or(crate::Error::ConfigError("no common section".to_owned()))?;

    Ok(Config {
        remote_server_password: common_section
            .get("token")
            .ok_or(crate::Error::ConfigError("common.token missing".to_owned()))?
            .as_str()
            .ok_or_else(|| crate::Error::ConfigError("common.token isn't a string".to_owned()))?
            .to_owned()
            .make_private(),
        remote_server_name: common_section
            .get("server_addr")
            .ok_or(crate::Error::ConfigError("common.server_addr".to_owned()))?
            .as_str()
            .ok_or_else(|| {
                crate::Error::ConfigError("common.server_addr isn't a string".to_owned())
            })?
            .to_owned(),
        remote_server_port: common_section
            .get("server_port")
            .ok_or(crate::Error::ConfigError("common.server_port".to_owned()))?
            .as_integer()
            .ok_or_else(|| {
                crate::Error::ConfigError("common.server_port isn't a number".to_owned())
            })? as u16,
        forward_ports: forwarded_ports,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForwardType {
    Tcp,
    Udp,
}

impl ForwardType {
    pub fn from(s: &str) -> Result<Self, Error> {
        match s.to_lowercase().as_str() {
            "tcp" => Ok(ForwardType::Tcp),
            "udp" => Ok(ForwardType::Udp),
            _ => Err(Error::ConfigError(format!("unknown forward type {s}"))),
        }
    }
}

impl Config {
    pub async fn save(&self) {
        let mut config = toml::map::Map::new();
        let mut common = toml::map::Map::new();
        common.insert(
            "token".to_owned(),
            toml::Value::String(self.remote_server_password.clone().make_unprivate()),
        );
        common.insert(
            "server_addr".to_owned(),
            toml::Value::String(self.remote_server_name.clone()),
        );
        common.insert(
            "server_port".to_owned(),
            toml::Value::Integer(i64::from(self.remote_server_port)),
        );
        common.insert(
            "authenticate_heartbeats".to_owned(),
            toml::Value::Boolean(true),
        );
        common.insert(
            "authenticate_new_work_conns".to_owned(),
            toml::Value::Boolean(true),
        );
        common.insert("tls_enable".to_owned(), toml::Value::Boolean(true));

        config.insert("common".to_owned(), toml::Value::Table(common));

        self.forward_ports.iter().for_each(|(name, port_forward)| {
            let mut port = toml::map::Map::new();
            port.insert(
                "local_port".to_owned(),
                toml::Value::Integer(i64::from(port_forward.local_port)),
            );
            port.insert(
                "remote_port".to_owned(),
                toml::Value::Integer(i64::from(port_forward.remote_port)),
            );
            port.insert(
                "type".to_owned(),
                toml::Value::String(match port_forward.forward_type {
                    ForwardType::Tcp => "tcp".to_owned(),
                    ForwardType::Udp => "udp".to_owned(),
                }),
            );
            port.insert(
                "local_ip".to_owned(),
                toml::Value::String("127.0.0.1".to_owned()),
            );
            port.insert("use_encryption".to_owned(), toml::Value::Boolean(true));
            config.insert(name.clone(), toml::Value::Table(port));
        });
        // write to file
        let result = toml::Value::Table(config);
        let toml_str = toml::to_string(&result).unwrap();

        tokio::fs::write("frpc.toml", toml_str).await.unwrap();
    }
}
