use super::{CommonConfig, Draft};
use crate::{
    core::{global::GLOBAL, sysopt, database::SqliteDB},
    log_err,
    utils::{dirs, json_util}, events::on_records_update_delete,
};
use anyhow::Result;
use once_cell::sync::OnceCell;
use std::fs;

pub struct Config {
    common_config: Draft<CommonConfig>,
}

impl Config {
    pub fn global() -> &'static Config {
        static CONFIG: OnceCell<Config> = OnceCell::new();

        CONFIG.get_or_init(|| Config {
            common_config: Draft::from(CommonConfig::new()),
        })
    }

    pub fn common() -> Draft<CommonConfig> {
        Self::global().common_config.clone()
    }

    /// 初始化配置
    pub fn init_config() -> Result<()> {
        log_err!(dirs::app_home_dir().map(|app_dir| {
            if !app_dir.exists() {
                let _ = fs::create_dir_all(&app_dir);
            }
        }));
        log_err!(dirs::app_data_dir().map(|app_dir| {
            if !app_dir.exists() {
                let _ = fs::create_dir_all(&app_dir);
            }
        }));
        // log_err!(dirs::app_data_img_dir().map(|app_dir| {
        //     if !app_dir.exists() {
        //         let _ = fs::create_dir_all(&app_dir);
        //     }
        // }));
        log_err!(dirs::config_path().map(|path| {
            if !path.exists() {
                log_err!(json_util::save(&path, &CommonConfig::template()));
            }
        }));
        Ok(())
    }
}

/// 修改通用配置文件的入口
pub async fn modify_common_config(patch: CommonConfig) -> Result<()> {
    Config::common().draft().patch_config(patch.clone());

    let auto_launch = patch.enable_auto_launch;
    let language = patch.language;
    let theme_mode = patch.theme_mode;
    let hotkeys = patch.hotkeys;
    let record_limit_days = patch.record_limit_days;

    match {
        if auto_launch.is_some() {
            // sysopt::Sysopt::global().update_launch().map_err(|e| {
            //     // 在这里处理错误，e 是错误值
            //     println!("An error occurred: {}", e);
            //     e  // 确保返回错误，以便 `?` 可以工作
            // })?;
        }

        if language.is_some() {
            // TODO
        }

        if theme_mode.is_some() {
            // todo send msg to frontend
        }

        if hotkeys.is_some() {
            GLOBAL.lock().load_shortcut_manager();
        }

        if record_limit_days.is_some() {
            let _ = SqliteDB::new().delete_older_than_days(record_limit_days.unwrap() as i64);
            on_records_update_delete();
        }

        <Result<()>>::Ok(())
    } {
        Ok(()) => {
            Config::common().apply();
            Config::common().data().save_file()?;
            Ok(())
        }
        Err(err) => {
            Config::common().discard();
            Err(err)
        }
    }
}

#[test]
fn test_config() {
    log_err!(Config::init_config());
    let common = Config::common();
    println!("common: {:?}", common.data());
}
