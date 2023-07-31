use super::*;
use globals::*;
use smash::app::lua_bind::CancelModule::is_enable_cancel;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn aerial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR)
    && !fighter.is_motion_one_of(&[Hash40::new("attack_air_n_hold"), Hash40::new("attack_air_hi_hold"), Hash40::new("attack_air_lw_hold")])
    && VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) 
    && !fighter.is_in_hitlag() {
        let mut new_status = 0;
        let mut is_input_cancel = false;
        if fighter.is_cat_flag(Cat1::SpecialN) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_N;
        } else if fighter.is_cat_flag(Cat1::SpecialS) {
            if !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) && !VarModule::is_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL) {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
            }
        } else if fighter.is_cat_flag(Cat1::SpecialHi) {
            if !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) && !VarModule::is_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL) {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
            }
        } 
        if boma.status_frame() >= 15 { //universal 15f
            fighter.check_airdodge_cancel();
            if is_input_cancel {fighter.change_status_req(new_status, false); } //special cancel
            if !fighter.is_motion(Hash40::new("attack_air_lw")) {
                boma.check_jump_cancel(false);
                if boma.status_frame() >= 25 {CancelModule::enable_cancel(fighter.module_accessor); } //aerial cancel 25f
            }
        }
    }
}

unsafe fn jump_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) { //jump cancel fx/logic
    if fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) {
        if StatusModule::is_changing(boma) {VarModule::set_int(fighter.battle_object, vars::common::instance::DOUBLE_JUMP_FRAME, 0);}
        else {VarModule::inc_int(fighter.battle_object, vars::common::instance::DOUBLE_JUMP_FRAME);}
    } else if boma.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) 
    && fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == 2
    && StatusModule::is_changing(boma) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        if VarModule::get_int(fighter.battle_object, vars::common::instance::DOUBLE_JUMP_FRAME) <= 5 {
            if fighter.is_status_one_of(&[*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, *FIGHTER_STATUS_KIND_SPECIAL_HI]) {
                let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma) - 6.0, z: PostureModule::pos_z(boma) };
                PostureModule::set_pos(boma, &pos);
                EFFECT(fighter, Hash40::new("bayonetta_witchtime_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
                fighter.set_int(1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
            else {EffectModule::req_follow(fighter.module_accessor, Hash40::new("bayonetta_feather_twinkle"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);}
        } else if fighter.is_status_one_of(&[*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, *FIGHTER_STATUS_KIND_SPECIAL_HI]) {
            EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe fn nspecial_mechanics(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE) { //PM-like neutral-b canceling
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.is_cat_flag(Cat1::AirEscape)  {
                ControlModule::reset_trigger(boma);
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL, true);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }//drift
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.04);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.005);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
        }
    }
}

unsafe fn reset_flags(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let boma = fighter.boma();
    if (!fighter.is_situation(*SITUATION_KIND_AIR) && StatusModule::is_situation_changed(boma))//checks for (re)spawn or grounded state
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD,
                                  *FIGHTER_STATUS_KIND_REBIRTH,
                                  *FIGHTER_STATUS_KIND_WIN,
                                  *FIGHTER_STATUS_KIND_LOSE,
                                  *FIGHTER_STATUS_KIND_ENTRY]){
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 0);
        VarModule::set_int(boma.object(), vars::bayonetta::instance::FAIR_STATE, 0);
    }
    //resets flags if hit
    if StopModule::is_damage(boma) {
        if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) > 1 {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 1);
        }
    } 
    //resets hitflag
    if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) && StatusModule::is_changing(boma) {
        if !boma.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP) {VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT);}
    }
}

unsafe fn resources(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    //resource limit
    if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) >= 2 {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    } else {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    //hit-flag
    if !VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 6 {
        VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT);
    }
}

unsafe fn forward_air(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let control_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) as *mut smash::app::KineticEnergy;
    if boma.is_motion_one_of(&[Hash40::new("attack_air_f"), Hash40::new("attack_air_f2")]) {
        //fair exact input from frame 12
        if boma.get_aerial() == Some(AerialKind::Fair) { 
            if boma.is_motion(Hash40::new("attack_air_f")) && boma.motion_frame() > 14.0 {
                MotionModule::change_motion(fighter.module_accessor, smash::phx::Hash40::new("attack_air_f2"), 1.0, 1.0, false, 0.0, false, false);
            }
            if boma.is_motion(Hash40::new("attack_air_f2")) && boma.motion_frame() >= 12.0 {
                MotionModule::change_motion(fighter.module_accessor, smash::phx::Hash40::new("attack_air_f3"), 1.0, 1.0, false, 0.0, false, false);
            }
        } 
        //cut speed n drift on-hit
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 6 {
            smash::app::lua_bind::KineticEnergy::mul_speed(control_energy, &Vector3f::new(0.708, 1.0, 1.0)); 
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.23);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.025);
        }
    }
    //fair cancels on-hit
    if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) {
        if boma.is_motion(Hash40::new("attack_air_f3")) {
            if boma.motion_frame() >= 30.0 {CancelModule::enable_cancel(fighter.module_accessor);} //cancels at 25f
        } else if boma.is_motion(Hash40::new("attack_air_f2")) {
            if boma.motion_frame() >= 21.0 {CancelModule::enable_cancel(fighter.module_accessor);} //cancels at 20f
        } else if boma.is_motion(Hash40::new("attack_air_f")) {
            if boma.motion_frame() >= 23.0 {CancelModule::enable_cancel(fighter.module_accessor);} //cancels at 20f
        }
    }
}

unsafe fn abk(fighter: &mut smash::lua2cpp::L2CFighterCommon, frame: f32) {
    let boma = fighter.boma();
    let pos = PostureModule::lr(boma);
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) {
        let anglestick = VarModule::get_float(fighter.battle_object, vars::bayonetta::status::ABK_ANGLE);
        joint_rotator(fighter, frame, Hash40::new("top"), Vector3f{x: -19.5*anglestick, y:90.0*pos, z:0.0}, 10.0, 13.0, 43.0, 43.0);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.020);
        if boma.status_frame() <= 7 { 
            VarModule::set_float(fighter.battle_object, vars::bayonetta::status::ABK_ANGLE, boma.left_stick_y());
        }
        //trajectory change
        if boma.status_frame() > 7 {
            if boma.status_frame() <= 25 && !fighter.is_in_hitlag() {
                KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f::new( -0.4 * anglestick * pos, 0.0 , 0.0));
                let angle = if pos < 0.0 {
                    25.0 - anglestick *14.0
                } else {
                    -25.0 + anglestick *14.0
                };
                let angle = angle.to_radians();
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, angle);
                app::sv_kinetic_energy::set_angle(fighter.lua_state_agent);
            } //dabk input
        } else if fighter.is_button_on(Buttons::Attack | Buttons::Catch) {VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK); }
        //dabk transition
        if boma.status_frame() == 6 && VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D);
            StatusModule::change_status_force(boma, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, true);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        } //dabk frame 
    } else if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D) && fighter.is_prev_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) && boma.status_frame() == 1 {
        MotionModule::set_frame_sync_anim_cmd(boma, 6.0, true, false, false);
    }
}

unsafe fn heel_slide_off(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {GroundModule::set_correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK)); }
}

unsafe fn branching_ftilt_jab(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if fighter.is_motion(Hash40::new("attack_s3_s")) && fighter.motion_frame() >= 28.0 {
        if fighter.is_cat_flag(Cat1::AttackHi3 | Cat1::SpecialN | Cat1::SpecialHi) {
            MotionModule::change_motion(fighter.module_accessor, smash::phx::Hash40::new("attack_s3_s3"), 0.0, 1.0, false, 0.0, false, false);
        } else if fighter.is_cat_flag(Cat1::AttackS3 | Cat1::AttackN) {
            MotionModule::change_motion(fighter.module_accessor, smash::phx::Hash40::new("attack_s3_s2"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK) && !fighter.is_cat_flag(Cat1::AttackLw3 | Cat1::AttackHi3 | Cat1::AttackS3 | Cat1::Catch) {
        //hold jab
        if (fighter.is_button_on(Buttons::Attack) && ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK as u8) > 5) 
        && (fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) || fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100))
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
        }
        if fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100) { //rapid/jab3
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
            } else if fighter.is_cat_flag(Cat1::AttackN) && !fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK_100, false);
            }
        }
    } 
}

unsafe fn bat_within_air_motion(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN) && fighter.is_situation(*SITUATION_KIND_AIR) {
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, boma.left_stick_y() * 0.44);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, boma.left_stick_x() * 0.44);
    }
}

unsafe fn joint_rotator(fighter: &mut L2CFighterCommon, frame: f32, joint: Hash40, rotation_amount: Vector3f, start_frame: f32, bend_frame: f32, return_frame: f32, straight_frame: f32) {
    let lua_state = fighter.lua_state_agent;
    let end_frame = MotionModule::end_frame(fighter.boma());
    let max_rotation = rotation_amount;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    if frame >= start_frame && frame < return_frame {
        // this has to be called every frame, or you snap back to the normal joint angle
        // interpolate to the respective waist bend angle
        let calc_x_rotate = max_rotation.x * (frame / (bend_frame - start_frame));
        let calc_y_rotate = max_rotation.y * (frame / (bend_frame - start_frame));
        let calc_z_rotate = max_rotation.z * (frame / (bend_frame - start_frame));
        let mut x_rotation = 0.0;
        let mut y_rotation = 0.0;
        let mut z_rotation = 0.0;
        if max_rotation.x < 0.0 {
            x_rotation = calc_x_rotate.clamp(max_rotation.x, 0.0);
        }
        else {
            x_rotation = calc_x_rotate.clamp(0.0, max_rotation.x);
        }
        if max_rotation.y < 0.0 {
            y_rotation = calc_y_rotate.clamp(max_rotation.y, 0.0);
        }
        else {
            y_rotation = calc_y_rotate.clamp(0.0, max_rotation.y);
        }
        if max_rotation.z < 0.0 { 
            z_rotation = calc_z_rotate.clamp(max_rotation.z, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_rotation.z);
        }
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(fighter.boma(), joint, &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    } else if frame >= return_frame && frame < straight_frame {
        // linear interpolate back to normal
        let calc_x_rotate = max_rotation.x *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let calc_y_rotate = max_rotation.y *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let calc_z_rotate = max_rotation.z *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let mut x_rotation = 0.0;
        let mut y_rotation = 0.0;
        let mut z_rotation = 0.0;
        if max_rotation.x < 0.0 {
            x_rotation = calc_x_rotate.clamp(max_rotation.x, 0.0);
        }
        else {
            x_rotation = calc_x_rotate.clamp(0.0, max_rotation.x);
        }
        if max_rotation.y < 0.0 {
            y_rotation = calc_y_rotate.clamp(max_rotation.y, 0.0);
        }
        else {
            y_rotation = calc_y_rotate.clamp(0.0, max_rotation.y);
        }
        if max_rotation.z < 0.0 { 
            z_rotation = calc_z_rotate.clamp(max_rotation.z, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_rotation.z);
        }
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(fighter.boma(), joint, &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    aerial_cancels(fighter, boma);
    jump_cancels(fighter, boma);
    nspecial_mechanics(fighter, boma);
    reset_flags(fighter, boma);
    resources(fighter, boma);
    forward_air(fighter, boma);
    abk(fighter, frame);
    heel_slide_off(fighter, boma);
    branching_ftilt_jab(fighter);
    bat_within_air_motion(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_BAYONETTA)]
pub unsafe fn bayonetta_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    bayonetta_frame(fighter);
}
pub unsafe fn bayonetta_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.frame);
    }
}
