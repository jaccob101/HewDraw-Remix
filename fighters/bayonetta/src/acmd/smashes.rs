
use super::*;

#[acmd_script( agent = "bayonetta", scripts = ["game_attacks4","game_attacks4lw", "game_attacks4hi"], category = ACMD_GAME, low_priority )]
unsafe fn bayonetta_attack_s4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 5, 0, true);
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            ArticleModule::generate_article(boma, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM, false, -1);
        }
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE_RANGE(fighter, 22.0, 68.0, 42.0);
    if is_excute(fighter) {
        AttackModule::set_power_mul_status(boma, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
}

pub fn install() {
    install_acmd_scripts!(
        bayonetta_attack_s4_game
);
}

