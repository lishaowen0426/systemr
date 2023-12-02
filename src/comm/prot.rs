use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectedPayload {
    #[serde(default)]
    root: String,

    #[serde(default)]
    log: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", content = "payload")]
pub enum Message {
    Connected(ConnectedPayload),
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

#[cfg(test)]
mod tests {
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
}
