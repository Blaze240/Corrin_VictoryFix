use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

unsafe extern "C" fn kamui_sound_win1a(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 53.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01"));
        }
        frame(agent.lua_state_agent, 79.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01_02"));
        }
        frame(agent.lua_state_agent, 90.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_kamui_win01"));
        }
        frame(agent.lua_state_agent, 127.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01_03"));
        }
    } else {
        frame(agent.lua_state_agent, 53.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01"));
        }
        frame(agent.lua_state_agent, 79.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01_02"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_kamui_win01"));
        }
        frame(agent.lua_state_agent, 127.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win01_03"));
        }
    }
}

pub fn install() {
    Agent::new("kamui")
        .sound_acmd("sound_win1a", kamui_sound_win1a)
        .install();
}
