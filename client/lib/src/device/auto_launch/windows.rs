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
            .build()?;
        Ok(AutoLaunch{
            inner:auto
        })
    }

    pub fn is_enabled(&self) -> anyhow::Result<bool> {
        Ok(self.inner.is_enabled()?)
    }
    pub fn enable(&self) -> anyhow::Result<()> {
        Ok(self.inner.enable()?)
    }

    pub fn disable(&self) -> anyhow::Result<()> {
        Ok(self.inner.disable()??)
    }
}