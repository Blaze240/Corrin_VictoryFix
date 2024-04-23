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

unsafe extern "C" fn kamui_sound_win2a(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win02"));
        }
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win02_02"));
        }
        frame(agent.lua_state_agent, 70.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win02_03"));
        }
        frame(agent.lua_state_agent, 85.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_kamui_win02"));
        }
        frame(agent.lua_state_agent, 127.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win02_04"));
        }
    } else {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win02"));
        }
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win02_02"));
        }
        frame(agent.lua_state_agent, 70.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win02_03"));
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_kamui_win02"));
        }
        frame(agent.lua_state_agent, 127.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win02_04"));
        }
    }
}
pub fn install() {
    Agent::new("kamui")
        .sound_acmd("sound_win2a", kamui_sound_win2a)
        .install();
}
