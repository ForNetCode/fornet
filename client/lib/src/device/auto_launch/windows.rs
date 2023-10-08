use anyhow::ensure;
use auto_launch_extra::*;

pub struct AutoLaunch {
    inner: auto_launch_extra::AutoLaunch
}

impl AutoLaunch {
    pub fn new(app_name:String, path:String) -> anyhow::Result<Self> {
        let auto = AutoLaunchBuilder::new()
            .set_app_name(&app_name)
            .set_app_path(&path)
            .set_use_launch_agent(true)
            .build().map_err(anyhow::Error::from)?;
        Ok(AutoLaunch {
            inner:auto
        })
    }

    pub fn is_enabled(&self) -> anyhow::Result<bool> {
        self.inner.is_enabled().map_err(anyhow::Error::from)
    }
    pub fn enable(&self) -> anyhow::Result<()> {
        self.inner.enable().map_err(anyhow::Error::from)
    }

    pub fn disable(&self) -> anyhow::Result<()> {
        self.inner.disable().map_err(anyhow::Error::from)
    }
}