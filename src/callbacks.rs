use crate::{Client, Edict, GE, Qstr, UserCmd, PMove, Trace, Vec3};
use crate::q2::{Cvar, dprintf, free_tags, TAG_GAME, tag_malloc, trace, point_contents, pmove};

static mut CLIENTS: *mut Client = 0 as *mut Client;
static mut MAXCLIENTS: isize = 0;

fn get_client(i: isize) -> *mut Client {
    unsafe {
        if i < MAXCLIENTS && i >= 0 {
            CLIENTS.offset(i)
        } else {
            0 as *mut Client
        }
    }
}

pub extern fn init() {
    dprintf("===== Rust InitGame =====\n");

    let mut max_clients = Cvar::new("max_clients", "1", 16);
    let cnt = max_clients.value_i32();
    unsafe {
        MAXCLIENTS = cnt as isize;
        GE.edicts = tag_malloc::<Edict>(GE.max_edicts, TAG_GAME);
        CLIENTS = tag_malloc::<Client>(cnt, TAG_GAME);
        GE.num_edicts = cnt + 1;
    }
}

pub extern fn shutdown() {
    dprintf("===== Rust Shutdown =====\n");

    free_tags(TAG_GAME);
}

pub extern fn write_game(fname: Qstr, autosave: i32) {}
pub extern fn read_game(fname: Qstr) {}

pub extern fn write_level(filename: Qstr) {}
pub extern fn read_level(filename: Qstr) {}

pub extern fn spawn_entities(mapname: Qstr, entstr: Qstr, spawnpt: Qstr) {}


pub extern fn client_connect (ent: *mut Edict, userinfo: Qstr) -> i32 {
    let cl = unsafe { &mut *ent };
    dprintf("Client connected.");
    cl.client = get_client(0);
    cl.get_client().unwrap().ps.fov = 90.0;

    1
}

pub extern fn client_begin(ent: *mut Edict) {}

pub extern fn client_userinfo_changed(ent: *mut Edict, userinfo: Qstr) {}

pub extern fn client_disconnect(ent: *mut Edict) {}
pub extern fn client_command(ent: *mut Edict) {}

static mut IGNORE: *mut Edict = 0 as *mut Edict;
pub extern fn pm_trace(start: *const Vec3, end: *const Vec3, mins: *const Vec3, maxs: *const Vec3) -> Trace {
    unsafe { 
        trace(&*start, &*end, &*mins, &*maxs, IGNORE, 1 | 2 | 0x10000 | 0x2000000)
    }
}

pub extern fn pm_pointcontents(point: *const Vec3) -> i32 {
    unsafe { point_contents(&*point) }
}

pub extern fn client_think(ent: *mut Edict, ucmd: *mut UserCmd) {
    let ps = unsafe { &mut (*(*ent).client).ps };

    ps.pmove.pm_type = 0;
    ps.pmove.gravity = 800;

    let mut p = PMove::new(
        &ps.pmove, 
        unsafe { &(*ucmd) },
        pm_trace,
        pm_pointcontents
    );

    unsafe { 
        IGNORE = ent;
    }

    pmove(&mut p);
    ps.pmove = p.s;
}

pub extern fn run_frame() {}

pub extern fn server_command() {}