use anyhow::{Result, Ok, anyhow};
use clap::Parser;
use xdiff::{cli::{Args, Action, RunArgs}, process_error_output, DiffConfig, highlight_text, diff_text};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    // let (output, output1, output2)= diff_text("hhhh1 hello\n1234", "hhhh2 hello\n1234")?;
    // // 终端高亮输出
    // let stdout = std::io::stdout();
    // let mut stdout = stdout.lock();
    // if atty::is(atty::Stream::Stdout) {
    //     writeln!(stdout, "--------------------------------------------------------------------------------")?;
    //     write!(stdout, "{}", highlight_text(&output1, "yaml", None)?)?;
    //     // write!(stdout, "---\n{}", output)?;
    //     writeln!(stdout, "--------------------------------------------------------------------------------")?;
    //     write!(stdout, "{}", highlight_text(&output2, "yaml", None)?)?;
    // } else {
    //     write!(stdout, "{}", output)?;
    // }

    // Ok(())
    let args = Args::parse();
    let result: std::result::Result<(), anyhow::Error> = match args.action {
        Action::Run(args) => run(args).await,
        Action::Parse => parse().await,
        _ => panic!("Not implemented"),
    };
    process_error_output(result)
}

async fn run(args: RunArgs) -> anyhow::Result<()> {
    let config_file = args.config.unwrap_or_else(|| "./xdiff.yaml".to_string());
    let config = DiffConfig::load_yaml(&config_file).await?;
    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow!("Profile {} not found in config file {}", args.profile, config_file)
    })?;
    let extra_args = args.extra_params.into();
    let output = profile.diff(&extra_args).await?;

    // 终端高亮输出
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    if atty::is(atty::Stream::Stdout) {
        writeln!(stdout, "--------------------------------------------------------------------------------")?;
        write!(stdout, "{}", highlight_text(&output, "yaml", None)?)?;
        // write!(stdout, "---\n{}", output)?;
    } else {
        write!(stdout, "{}", output)?;
    }

    Ok(())
}

async fn parse() -> anyhow::Result<()> {
  
    Ok(())
}