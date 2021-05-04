pub trait Connection {
    fn write(&self, json: serde_json::Value) -> std::result::Result<(),()>;
    fn read(&self, json: serde_json::Value) -> std::result::Result<(),()>;
}

pub struct IdleConnection {}

impl Connection for IdleConnection {
    fn write(&self, _json: serde_json::Value) -> std::result::Result<(),()> {
        Ok(())
    }

    fn read(&self, _json: serde_json::Value) -> std::result::Result<(),()> {
        Ok(())
    }
}