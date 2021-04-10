#[derive(Clone, Debug)]
pub struct Application {
    /// destination server address
    destination: String,
}

impl Application {
    pub fn new(addr: &str) -> anyhow::Result<Self> {
        Ok(Self {
            destination: addr.parse()?,
        })
    }
    pub fn dest(&self) -> &str {
        self.destination.as_str()
    }
}
