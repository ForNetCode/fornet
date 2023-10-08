use std::fs;
use std::io::Write;
use std::path::PathBuf;
use cmd_lib::{run_cmd, run_fun};

// This is for macos root
// https://github.com/mach-kernel/launchk/blob/master/xpc-sys/README.md
pub struct AutoLaunch {
    app_name: String,
    path: String,
}

const LAUNCH_ROOT_PATH:&str = "/Library/LaunchDaemons";

impl AutoLaunch {
    pub fn new(app_name:String, path:String) -> anyhow::Result<Self> {
        Ok(AutoLaunch {app_name, path})
    }
    pub fn is_enabled(&self) -> anyhow::Result<bool> {

        let app_name = &self.app_name;
        let result = run_fun!(launchctl list | grep $app_name);

        if result.is_ok() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn enable(&self) -> anyhow::Result<()> {

        let  file_path = PathBuf::from(LAUNCH_ROOT_PATH).join(format!("{}.plist", self.app_name));

        let mut file = fs::File::options().create(true).write(true).open(&file_path)?;
        write!(file,r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>Label</key>
	<string>{}</string>
	<key>UserName</key>
	<string>root</string>
	<key>ProgramArguments</key>
	<array>
		<string>{}</string>
	</array>
	<key>StandardOutPath</key>
	<string>/dev/null</string>
	<key>StandardErrorPath</key>
	<string>/dev/null</string>
</dict>
</plist>"#, self.app_name, self.path)?;
        //cmd_debug!(launchctl load -w $file_path);
        Ok(run_cmd!(launchctl load -w $file_path)?)
    }

    pub fn disable(&self) -> anyhow::Result<()> {
        let app_name = &self.app_name;
        Ok(run_cmd!(launchctl unload $app_name)?)
    }

}


