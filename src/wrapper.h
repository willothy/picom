// This file contains the API wrapper for Rust-bindgen to use.

typedef struct {
} lua_State;

// The main entry point for picom.
int picom_run(int argc, char **argv);

// Retrieve a handle to the global Lua state.
lua_State *get_lua_state();
