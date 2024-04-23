use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "ike", script = "sound_win3", category = ACMD_SOUND )]
unsafe fn ike_sound_win3(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 1 {
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win03"));
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_special_h01"));
        }
    frame(agent.lua_state_agent, 78.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_jump01"));
    }
    frame(agent.lua_state_agent, 89.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_special_h04"));
    }
    frame(agent.lua_state_agent, 124.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_special_h06"));
    }
}
    else {
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_special_h01"));
        }
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win03"));
        }
        frame(agent.lua_state_agent, 78.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_jump01"));
        }
        frame(agent.lua_state_agent, 89.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_special_h04"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_special_h06"));
        }
    }
                    }
pub fn install() {
    smashline::install_acmd_scripts!(
     ike_sound_win3
    );
}
