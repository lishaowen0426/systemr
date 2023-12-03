use crate::linux::oci;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", content = "payload")]
pub enum Message {
    Connected(ConnectedPayload),
    Disconnected,
    Spec(SpecPayload),
}

impl TryFrom<&[u8]> for Message {
    type Error = serde_json::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let p: Self = serde_json::from_slice(value)?;
        Ok(p)
    }
}

impl Message {
    pub fn send<W>(&self, to: W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        let b = serde_json::to_writer(to, self)
            .map_err(|_: serde_json::Error| std::io::Error::other("send message failed"))?;
        Ok(b)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectedPayload {
    #[serde(default)]
    root: PathBuf,

    #[serde(default)]
    log: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecPayload {
    spec: oci::Spec,
}

#[cfg(test)]
mod tests {
    use crate::linux::cap;
    use serde_json::json;

    use super::*;
    #[test]
    fn json() {
        {
            let data: &[u8] = r#"
            {
                "action": "Connected",
                "payload": {"root": "/run/runc", "log":""}
            }"#
            .as_bytes();

            let p = Message::try_from(data).unwrap();
            assert!(matches!(p, Message::Connected(_)));
        }
    }

    #[test]
    fn json_cap() {
        let c = cap::Capability::CAP_CHOWN;

        let j = json!({
            "capabilities": c
        });

        println!("{}", j.to_string())
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Path {
        root: std::path::PathBuf,
    }

    #[test]
    fn json_path() {
        let p = "/run/runc";

        let j = json!({
            "root": p
        });

        let pp: Path = serde_json::from_str(&j.to_string()).unwrap();
        println!("{:?}", pp);
    }
}
