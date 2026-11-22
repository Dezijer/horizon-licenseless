//! Offline policy for CLI Play.
use super::*;

/// Build the command for an offline Horizon launch.
pub fn client_as(tag: &str, who: Option<&str>) -> Result<Command, String> {
    if who.is_some() {
        return Err(crate::i18n::tr(
            "This Horizon fork plays offline and does not use a Microsoft account.",
        )
        .into());
    }
    if let Some(error) = game_architecture_launch_error(tag) {
        return Err(error);
    }
    let developer = crate::developer_offline::enabled(tag);
    if developer {
        crate::developer_offline::require_capability(tag)?;
    }
    need_prepared(tag)?;
    play_step("checking the game's assets");
    complete_assets(tag);
    crate::browser::horizon::before_play(&game_home());
    let mut cmd = client_command(tag);
    if developer {
        crate::developer_offline::mark(&mut cmd);
        eprintln!("{}", crate::i18n::tr(crate::developer_offline::NOTICE));
    }
    append_game_args(&mut cmd);
    Ok(cmd)
}
