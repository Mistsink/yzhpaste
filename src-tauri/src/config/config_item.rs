use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CfgHotkeys {
    pub active: String,
}

impl CfgHotkeys {
    pub fn new() -> Self {
        Self {
            // todo windows 快捷键不一样，需要兼容
            active: "CmdOrCtrl+Shift+C".into(),
        }
    }
}
