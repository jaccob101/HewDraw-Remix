use super::*;

// FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT

unsafe extern "C" fn special_lw_jump_squat_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);
    }
    return 0.into();
}

unsafe extern "C" fn special_lw_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK);
    1.into()
}

unsafe extern "C" fn special_lw_walk_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK);
    1.into()
}

unsafe extern "C" fn special_lw_fall_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK);
    1.into()
}

unsafe extern "C" fn special_lw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT1);

    fighter.sub_change_motion_by_situation(Hash40::new("special_lw").into(), Hash40::new("special_air_lw").into(), false.into());

    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_attack_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue{
    let charge_frames = VarModule::get_int(fighter.battle_object, vars::dedede::instance::JET_CHARGE_FRAMES);
    let mut charge_level = charge_frames as f32 / 30.0;

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw").into(), Hash40::new("special_air_lw").into(), true.into());

        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }

    if charge_level > 1.0{
        MotionModule::set_rate(fighter.module_accessor, charge_level as f32 * 1.4);

        if VarModule::is_flag(fighter.battle_object, vars::dedede::status::SPECIAL_LW_CONTINUE_SPIN){
            VarModule::set_flag(fighter.battle_object, vars::dedede::instance::CONTINUE_JET_SPIN, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("dedede_final_jet"), false, true);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
            VarModule::set_int(fighter.battle_object, vars::dedede::instance::JET_CHARGE_FRAMES, charge_frames - 30);
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER){
                let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER);
                let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
                let article_boma = sv_battle_object::module_accessor(object_id);
    
                MotionModule::change_motion(article_boma, Hash40::new("attack"), 0.0, 1.0, false, 0.0, false, false);
    
            }
            VarModule::set_flag(fighter.battle_object, vars::dedede::status::SPECIAL_LW_CONTINUE_SPIN, false);

        }
    }

    if MotionModule::is_end(fighter.module_accessor){
        if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_AIR{
            fighter.change_status((*FIGHTER_STATUS_KIND_FALL_SPECIAL).into(), false.into()); 
        }
        else{
            fighter.change_status((*FIGHTER_STATUS_KIND_WAIT).into(), false.into()); 
        }
        return 0.into();
    }     


    return 0.into();
}

unsafe extern "C" fn special_lw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue{
    VarModule::set_flag(fighter.battle_object, vars::dedede::status::SPECIAL_LW_CONTINUE_SPIN, false);
    VarModule::set_flag(fighter.battle_object, vars::dedede::instance::CONTINUE_JET_SPIN, false);
    VarModule::set_int(fighter.battle_object, vars::dedede::instance::JET_CHARGE_FRAMES, 0);
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT, special_lw_jump_squat_exec);
    agent.status(Main, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, special_lw_attack_main);
    agent.status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, special_lw_attack_end);
    agent.status (Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT, special_lw_wait_pre);
    agent.status (Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WALK, special_lw_walk_pre);
    agent.status (Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL, special_lw_fall_pre);
}