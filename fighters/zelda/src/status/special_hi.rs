use super::*;

unsafe extern "C" fn special_hi_2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3 {
        //re-uses waveland window logic
        let init_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_Y); //teleport direction
        let pos = *PostureModule::pos(fighter.module_accessor); //top bone (bottom of ecb w/o shifting)
        let bot_snap = &Vector2f::new(pos.x, pos.y + ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_hi.bot_snap"));
        let top_snap = &Vector2f::new(pos.x, pos.y + ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_hi.top_snap"));
        
        //let upper_bound_offset_y = VarModule::get_float(fighter.battle_object, vars::common::instance::ECB_CENTER_Y_OFFSET); //center of ECB
        //let upper_bound_y = pos.y + upper_bound_offset_y; //pos of ecb's center
        //let snap_window = if init_speed_y <= 0.0 {upper_bound_offset_y} else {(upper_bound_offset_y).max(ParamModule::get_float(fighter.battle_object, ParamType::Common, "waveland_distance_threshold"))}; //caps rising tp snap to 6 units
        //let lower_bound = Vector2f::new(pos.x, upper_bound_y - snap_window); //bottom ecb for snap
        let ground_pos_any = &mut Vector2f::zero();
        let ground_pos_stage = &mut Vector2f::zero();
        let is_touch_any = GroundModule::line_segment_check(fighter.module_accessor, top_snap, bot_snap, &Vector2f::zero(), ground_pos_any, true);
        let is_touch_stage = GroundModule::line_segment_check(fighter.module_accessor, top_snap, bot_snap, &Vector2f::zero(), ground_pos_stage, false);
        let can_snap = !(is_touch_any == 0 as *const *const u64 || (is_touch_stage != 0 as *const *const u64 && init_speed_y > 0.0)); //avoid snapping to stage from below
        if can_snap { // pretty sure it returns a pointer, at least it defo returns a non-0 value if success
            PostureModule::set_pos(fighter.module_accessor, &Vector3f::new(pos.x, ground_pos_any.y + 0.1, pos.z));
            GroundModule::attach_ground(fighter.module_accessor, true);
        }
        if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
        //GroundModule::attach_ground(fighter.module_accessor, true);
            PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
    }
    0.into()
}

unsafe extern "C" fn special_hi_3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        GroundModule::attach_ground(fighter.module_accessor, true);
        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    ControlModule::clear_command(fighter.module_accessor, true);
    smashline::original_status(Main, fighter, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, special_hi_2_end);
    agent.status(Main, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, special_hi_3_main);
}