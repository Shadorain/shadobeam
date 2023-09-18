use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use better_panic::Settings;
use directories::ProjectDirs;
use log::error;
use log4rs::{
    append::rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
        RollingFileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use super::Tui;

const LOG_WINDOW_SIZE: u64 = 1000 * 1024;
const LOG_STORE_COUNT: u32 = 8;

pub fn initialize_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        match Tui::new() {
            Ok(t) => {
                if let Err(r) = t.exit() {
                    error!("Unable to exit Terminal: {r:?}");
                }
            }
            Err(r) => error!("Unable to exit Terminal: {r:?}"),
        }
        Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .create_panic_handler()(panic_info);
        std::process::exit(1);
    }));
}

pub fn version() -> String {
    format!(
        "{}\n\nAuthors: {}",
        env!("CARGO_PKG_NAME"),
        clap::crate_authors!(),
    )
}

pub fn get_data_dir() -> Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("SHADOBEAM_DATA") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = ProjectDirs::from("com", "shadorain", "shadobeam") {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        return Err(anyhow!("Unable to find data directory for Shadobeam"));
    };
    Ok(directory)
}

pub fn initialize_logging() -> Result<()> {
    let directory = get_data_dir()?;
    std::fs::create_dir_all(&directory)
        .context(format!("{:?} could not be created", &directory))?;
    let log_path = directory.join("logs/shadobeam.log");
    log4rs::init_config(
        Config::builder()
            .appender(
                Appender::builder().build(
                    "logfile",
                    Box::new(
                        RollingFileAppender::builder()
                            .append(true)
                            .encoder(Box::new(PatternEncoder::new(
                                "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}",
                            )))
                            .build(
                                log_path,
                                Box::new(CompoundPolicy::new(
                                    Box::new(SizeTrigger::new(LOG_WINDOW_SIZE)),
                                    Box::new(
                                        FixedWindowRoller::builder().base(0).build(
                                            &directory
                                                .join("logs/archive/shadobeam-{}.log.gz")
                                                .to_string_lossy(),
                                            LOG_STORE_COUNT,
                                        )?,
                                    ),
                                )),
                            )?,
                    ),
                ),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .build(log::LevelFilter::Debug),
            )?,
    )?;
    Ok(())
}
