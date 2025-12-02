use std::fs;
use std::path::PathBuf;

use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

use crate::domain::ports::{LockfileData, LockfilePort};

pub struct LeagueLockfileProvider {
    explicit_path: Option<PathBuf>,
}

impl LeagueLockfileProvider {
    pub fn new(explicit_path: Option<PathBuf>) -> Self {
        Self { explicit_path }
    }

    fn find_lockfile_path(&self) -> anyhow::Result<PathBuf> {
        // 1. Explicit override
        if let Some(ref p) = self.explicit_path {
            if p.exists() {
                return Ok(p.clone());
            }
        }

        // 2. Scan processes
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
        );

        sys.refresh_processes(ProcessesToUpdate::All, true);

        for (_pid, proc_) in sys.processes() {
            let name = proc_.name().to_string_lossy();
            if name.contains("LeagueClientUx.exe") {
                if let Some(exe) = proc_.exe() {
                    if let Some(dir) = exe.parent() {
                        let lf = dir.join("lockfile");
                        if lf.exists() {
                            return Ok(lf);
                        }
                    }
                }
            }
        }

        // 3. Fallbacks
        let fallbacks = [
            PathBuf::from("C:/Riot Games/League of Legends/lockfile"),
            dirs::home_dir()
                .unwrap_or_default()
                .join("AppData/Local/Riot Games/League of Legends/lockfile"),
        ];

        for p in fallbacks {
            if p.exists() {
                return Ok(p);
            }
        }

        Err(anyhow::anyhow!("LCU lockfile not found"))
    }
}

impl LockfilePort for LeagueLockfileProvider {
    fn read_lockfile(&self) -> anyhow::Result<LockfileData> {
        let path = self.find_lockfile_path()?;
        let content = fs::read_to_string(&path)?.trim().to_string();

        // Format: <name>:<pid>:<port>:<password>:<protocol>
        let mut parts = content.split(':');

        let _name = parts.next().unwrap_or(""); // ignored
        let _pid = parts.next().unwrap_or(""); // ignored
        let port = parts.next().unwrap_or("0").parse::<u16>()?;
        let password = parts.next().unwrap_or("").to_string();

        Ok(LockfileData { port, password })
    }
}
