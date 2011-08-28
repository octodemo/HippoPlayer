#include "HippoLua.h"
#include "graphics/gui/HippoGui.h"
#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

lua_State* g_luaState = 0;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void HippoLua_updateScript()
{
	HippoGui_begin();
	lua_getglobal(g_luaState, "update");
	lua_pcall(g_luaState, 0, 0, 0);
	HippoGui_end();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern void Hippo_quit();

static int luaQuit()
{
	Hippo_quit();
	return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const luaL_Reg hippoLib[] =
{
	{ "quit", luaQuit },
	{ 0, 0 },
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int registerHippoLib(lua_State* state)
{
	luaL_newlib(state, hippoLib);
	return 1;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void HippoLua_registerLuaFunctions(struct lua_State* luaState)
{
	luaL_requiref(luaState, "hippo", registerHippoLib, 1);
    lua_pop(luaState, 1);  
}
