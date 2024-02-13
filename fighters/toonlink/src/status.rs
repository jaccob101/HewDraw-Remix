use super::*;
use globals::*;
 

pub fn install() {
    install_status_scripts!(
        //attack_air_pre,
        //attack_air_main,
        attack_air_exec,
        pre_specialhi,
        pre_specialhi_end,
        specialhi_end
    );
}

pub fn install_custom() {
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_toonlink"),
        statuses::toonlink::float_start,
        StatusInfo::new()
            .with_pre(float_start_pre)
            .with_main(float_start_main)
            .with_end(float_start_end)
    );
}

#[status_script(agent = "toonlink", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, vars::toonlink::instance::float_frame) < 120
    && VarModule::get_int(fighter.battle_object, vars::toonlink::instance::float_frame) > 0 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        float_movement(fighter);
        VarModule::dec_int(fighter.battle_object, vars::toonlink::instance::float_frame);
    }
    0.into()
}

unsafe extern "C" fn float_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn float_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
    //WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    let speed_y = 0.0;
    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
        let speed_y = 0.35;
    }
    if VarModule::get_int(fighter.battle_object, vars::toonlink::instance::float_frame) == 120 {
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, speed_y);
    }
    
    fighter.sub_shift_status_main(L2CValue::Ptr(float_start_main_loop as *const () as _))
}

unsafe extern "C" fn float_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) {
        VarModule::set_int(fighter.battle_object, vars::toonlink::instance::float_frame, 0);
    }
    0.into()
}

unsafe extern "C" fn float_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
    //    return 1.into();
    //}
    VarModule::dec_int(fighter.battle_object, vars::toonlink::instance::float_frame);
    if VarModule::get_int(fighter.battle_object, vars::toonlink::instance::float_frame) < 119 {float_movement(fighter); }
    //if fighter.is_button_trigger(Buttons::Attack) {aerial_attack(fighter); }
    if VarModule::get_int(fighter.battle_object, vars::toonlink::instance::float_frame) == 0 {fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into()); }
    0.into()
}

unsafe extern "C" fn float_movement(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_y = {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)
    };
    let speed_x = {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)
    };
    let move_y = speed_y as f32 + 0.09 * fighter.stick_y();
    let move_x = speed_x as f32 + 0.09 * fighter.stick_x();
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, move_x.clamp(-1.15, 1.15), move_y.clamp(-1.15, 1.15));
    0.into()
}

//unsafe extern "C" fn aerial_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
//    if CancelModule::is_enable_cancel(fighter.module_accessor) || fighter.is_motion(Hash40::new("fall")) {
//        let control = ControlModule::get_attack_air_kind(fighter.module_accessor);
//        match control {
//            1 => MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_n"), 0.0, 1.0, false, 0.0, false, false),
//            2 => MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f"), 0.0, 1.0, false, 0.0, false, false),
//            3 => MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_b"), 0.0, 1.0, false, 0.0, false, false),
//            4 => MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_hi"), 0.0, 1.0, false, 0.0, false, false),
//            5 => MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_lw"), 0.0, 1.0, false, 0.0, false, false),
//        };
//        
//
//    }
//}

#[status_script(agent = "toonlink", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //if fighter.is_prev_status(*statuses::toonlink::float_start) {
    //    kinetic = *FIGHTER_KINETIC_TYPE_MOTION_AIR;
    //} else {
    //    kinetic = *FIGHTER_KINETIC_TYPE_MOTION_FALL;
    //}
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT,
        (*FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_ATTACK)
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u64,
        0,
        0,
        0
    );
    0.into()
}

#[status_script(agent = "toonlink", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, vars::toonlink::instance::float_frame) == 120
    || VarModule::get_int(fighter.battle_object, vars::toonlink::instance::float_frame) == 0
    //fighter.global_table[PREV_STATUS_KIND] != FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT
    //&& fighter.global_table[PREV_STATUS_KIND] != FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START
    && fighter.global_table[PREV_STATUS_KIND] != statuses::toonlink::float_start {
        fighter.sub_attack_air_common(false.into());
        //MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
        return fighter.main_shift(normal_main_loop);
    }

    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    let fighter_log_attack_kind = match motion_kind {
        ::utils::hash40!("attack_air_n") => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N,
        ::utils::hash40!("attack_air_f") => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F,
        ::utils::hash40!("attack_air_b") => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B,
        ::utils::hash40!("attack_air_lw") => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW,
        ::utils::hash40!("attack_air_hi") => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI,
        _ => {
            fighter.sub_attack_air_common(false.into());
            //MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
            return fighter.main_shift(attack_air_main_status);
        }
    };
    smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, fighter_log_attack_kind);
    let _ = fighter.status_AttackAir_Main_common();
    WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    fighter.main_shift(default_main_loop)
}

unsafe extern "C" fn normal_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //// Moved above is_enable_cancel for readability concerns
    //let can_shoot_item = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_AIR);
    //let can_attack_air = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    //let is_trigger_opt = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0; // are we pressing the attack button in any capacity?
    //if CancelModule::is_enable_cancel(fighter.module_accessor)
    //// We are pressing the A button and can either shoot an item or can do an aerial
    //&& (is_trigger_opt && ((can_shoot_item && fighter.sub_is_item_shoot_air().get_bool()) || can_attack_air))
    //&& WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON)
    ////&& WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT) if float available
    //&& ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    //&& fighter.global_table[STICK_Y].get_f32() <= WorkModule::get_param_float(fighter.module_accessor, smash::hash40("common"), smash::hash40("squat_stick_y"))
    //&& fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    //&& !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_JUMP_FROM_WATER)

    if VarModule::get_int(fighter.battle_object, vars::toonlink::instance::float_frame) == 120 
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    && fighter.global_table[STICK_Y].get_f32() <= WorkModule::get_param_float(fighter.module_accessor, smash::hash40("common"), smash::hash40("squat_stick_y"))
    {
        if CancelModule::is_enable_cancel(fighter.module_accessor) { 
            fighter.change_to_custom_status(statuses::toonlink::float_start, true, false);
            return 1.into();
        } else {
            float_movement(fighter);
        }
        //fighter.change_status(FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START.into(), true.into());
    }
    if fighter.status_AttackAir_Main_common().get_bool() {
        return 0.into();
    }
    fighter.sub_air_check_superleaf_fall_slowly();
    if !fighter.global_table[IS_STOPPING].get_bool() {
        fighter.sub_attack_air_uniq_process_exec_fix_pos();
    }
    //if !fighter.global_table[IS_STOPPING].get_bool() {
    //    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
    //}
    0.into()
}

// Default reimplementation of the main loop for an aerial
// No special functionality
unsafe extern "C" fn default_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    let _ = fighter.status_AttackAir_Main_common();
    WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    0.into()
}

pub unsafe extern "C" fn attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(L2CValue::Bool(false));
    //MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_status_loop as *const () as _))
}

pub unsafe extern "C" fn attack_air_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            fighter.sub_attack_air_uniq_process_exec_fix_pos();
        }
        //if !fighter.global_table[IS_STOPPING].get_bool() {
        //    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        //}
        0.into()
    }
    else {
        1.into()
    }
}

// FIGHTER_STATUS_KIND_SPECIAL_HI //

#[status_script(agent = "toonlink", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_specialhi(fighter: &mut L2CFighterCommon, arg: u64) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        let start_speed = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let start_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_x_mul");
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_speed * start_x_mul);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    let mask_flag = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64
    } else {
        0 as u64
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        0,
        0,
        0,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask_flag,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

// FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END //

unsafe extern "C" fn link_situation_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 1.into()
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            return 1.into()
        }
        if fighter.global_table[PREV_SITUATION_KIND] != SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            return 1.into()
        }
    }
    return 0.into()
}

#[status_script(agent = "toonlink", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask_flag,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

#[status_script(agent = "toonlink", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_end_Main as *const () as _))
}

unsafe extern "C" fn specialhi_end_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut motion_value = 0.55;


    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if link_situation_helper(fighter).get_bool() {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                if !StatusModule::is_changing(fighter.module_accessor) {
                    fighter.change_status(
                        L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                        L2CValue::Bool(false)
                    );
                    return 1.into()
                }
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
                }
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                fighter.shift(L2CValue::Ptr(sub_specialhi_end_Main as *const () as _));
                return 0.into()
            }
        }
        if frame < 46.0 {
            if stick_x != 0.0 {
                KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f { x: (motion_value * stick_x.signum()), y: 0.0, z: 0.0});
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_FALL_SPECIAL),
                    L2CValue::Bool(false)
                );
                return 1.into()
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                    L2CValue::Bool(false)
                );
                return 1.into()
            }
        }
    }
    return 0.into()
}

unsafe extern "C" fn sub_specialhi_end_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);

    GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
        if !MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                if link_situation_helper(fighter).get_bool() {
                    fighter.change_status(
                        L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                        L2CValue::Bool(true)
                    );
                    return 1.into()
                }
            }
            else {
                if fighter.sub_transition_group_check_air_cliff().get_bool() {
                    return 1.into()
                }
            }
        }
        else {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_FALL_SPECIAL),
                L2CValue::Bool(true)
            );
            return 1.into()
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) && MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                L2CValue::Bool(false)
            );
            return 1.into()
        }
        if link_situation_helper(fighter).get_bool() && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                L2CValue::Bool(true)
            );
            return 1.into()
        }
    }
    return 0.into()
}