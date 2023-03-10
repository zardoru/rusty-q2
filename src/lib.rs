mod callbacks;
mod q2;

use std::ffi::c_char;
use std::os::raw::c_void;
use std::panic;
use std::ptr::null;

#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

type ShortVec = [i16; 3];
type Qstr = *const c_char;

#[repr(C)]
#[derive(Clone)]
pub struct UserCmd {
    pub msec: u8,
    pub buttons: u8,
    pub angles: ShortVec,
    pub forwardmove: i16,
    pub sidemove: i16,
    pub upmove: i16,
    pub impulse: u8,
    pub lightlevel: u8,
}

#[repr(C)]
#[derive(Clone)]
pub struct PMoveState {
    pub pm_type: i32,
    pub origin: ShortVec,
    pub velocity: ShortVec,
    pub pm_flags: u8,
    pub pm_time: u8,
    pub gravity: i16,
    pub delta_angles: ShortVec
}

#[repr(C)]
pub struct EntityState {
    pub number: i32,
    pub origin: Vec3,
    pub angles: Vec3,
    pub old_origin: Vec3,
    pub modelindex: i32,
    pub modelindex2: i32,
    pub modelindex3: i32,
    pub modelindex4: i32,
    pub frame: i32,
    pub skinnum: i32,
    pub effects: u32,
    pub renderfx: i32,
    pub solid: i32,
    pub sound: i32,
    pub event: i32
}

#[repr(C)]
pub struct PlayerState {
    pub pmove: PMoveState,
    pub viewangles: Vec3,
    pub viewoffset: Vec3,
    pub kick_angles: Vec3,
    pub gunangles: Vec3,
    pub gunoffset: Vec3,
    pub gunindex: i32,
    pub gunframe: i32,
    pub blend: [f32; 4],
    pub fov: f32,
    pub rdflags: i32,
    pub stats: [i32; 32]
}

#[repr(C)]
pub struct Client {
    pub ps: PlayerState,
    pub ping: i32
}

#[repr(C)]
pub struct Edict {
    pub s: EntityState,
    pub client: *mut Client,
    pub inuse: i32,
    pub linkcount: i32,
    pub area_prev: *mut i32,
    pub area_next: *mut i32,
    pub num_clusters: i32,
    pub clusternums: [i32; 16],
    pub headnode: i32,
    pub areanum: i32,
    pub areanum2: i32,
    pub svflags: i32,
    pub mins: Vec3,
    pub maxs: Vec3,
    pub absmin: Vec3,
    pub absmax: Vec3,
    pub size: Vec3,
    pub solid: i32,
    pub clipmask: i32,
    pub owner: *mut Edict
}

impl Edict {
    pub fn get_client(&mut self) -> Option<&mut Client> {
        if self.client != std::ptr::null_mut() {
            unsafe { Some (&mut *self.client) }
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct Plane {
    normal: Vec3,
    dist: f32,
    type_bits: i32
}

#[repr(C)]
pub struct Trace {
    allsolid: i32,
    startsolid: i32,
    fraction: f32,
    endpos: Vec3,
    plane: Plane,
    surface: *mut i32,
    contents: i32,
    entity: *mut Edict
}

type TraceFunc = extern fn(*const Vec3, *const Vec3, *const Vec3, *const Vec3) -> Trace;
type PointContentsFunc = extern fn (*const Vec3) -> i32;

#[repr(C)]
pub struct PMove {
    s: PMoveState,
    cmd: UserCmd,
    snapinitial: i32,
    numtouch: i32,
    touchents: [*mut Edict; 32],
    viewangles: Vec3,
    viewheight: f32,
    mins: Vec3,
    maxs: Vec3,
    groundentity: *mut Edict,
    watertype: i32,
    waterlevel: i32,
    trace: TraceFunc,
    pointcontents: PointContentsFunc
}

impl PMove {
    pub fn new(
        state: &PMoveState, 
        cmd: &UserCmd, 
        trace: extern fn (*const Vec3, *const Vec3, *const Vec3, *const Vec3) -> Trace,
        pointcontents: extern fn (point: *const Vec3) -> i32) -> PMove {
        PMove {
            s: state.clone(),
            cmd: cmd.clone(),
            trace: trace,
            snapinitial: 0,
            pointcontents: pointcontents,
            groundentity: 0 as *mut Edict,
            numtouch: 0,
            touchents: [0 as *mut Edict; 32],
            mins: Vec3::zero(),
            maxs: Vec3::zero(),
            viewangles: Vec3::zero(),
            viewheight: 0.0,
            waterlevel: 0,
            watertype: 0
        }
    }
}

#[repr(C)]
pub struct Cvar {
    pub name: Qstr,
    pub string: Qstr,
    pub latched_string: Qstr,
    pub flags: i32,
    pub modified: i32,
    pub value: f32
}

type IndexFunc = extern fn (Qstr) -> i32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GameImport {
    bprintf: extern fn (i32, Qstr),
    dprintf: extern fn (Qstr),
    cprintf: extern fn (*const Edict, i32, Qstr),
    centerprintf: extern fn (*const Edict, Qstr),
    sound: extern fn (*const Edict, i32, usize, f32, f32, f32),
    positioned_sound: extern fn (*const Vec3, *const Edict, i32, usize, f32, f32, f32),
    configstring: extern fn (i32, Qstr),
    error:  extern fn (Qstr),
    modelindex: IndexFunc,
    soundindex: IndexFunc,
    imageindex: IndexFunc,
    setmodel: extern fn (*const Edict, Qstr),
    trace: extern fn (*const Vec3, *const Vec3, *const Vec3, *const Vec3, *const Edict, i32) -> Trace,
    pointcontents: extern fn(*const Vec3) -> i32,
    in_pvs: extern fn(*const Vec3, *const Vec3) -> i32,
    in_phs: extern fn(*const Vec3, *const Vec3) -> i32,
    set_area_portal_state: extern fn (i32, i32),
    areas_connected: extern fn(i32, i32) -> i32,
    linkentity: extern fn(*const Edict),
    unlinkentity: extern fn(*const Edict),
    box_edicts: extern fn(*const Vec3, *const Vec3, *const *const Edict, i32, i32) -> i32,
    pmove: extern fn(*mut PMove),
    multicast: extern fn(*const Vec3, i32),
    unicast: extern fn(*const Edict, i32),
    write_char: extern fn(i32),
    write_byte: extern fn(i32),
    write_short: extern fn(i32),
    write_long: extern fn(i32),
    write_float: extern fn(f32),
    write_string: extern fn(Qstr),
    write_pos: extern fn(*const Vec3),
    write_dir: extern fn (*const Vec3),
    write_angle: extern fn(f32),
    tag_malloc: extern fn(i32, i32) -> *const c_void,
    tag_free: extern fn(*const c_void),
    free_tags: extern fn(i32),
    cvar: extern fn(Qstr, Qstr, i32) -> *const Cvar,
    cvar_set: extern fn(Qstr, Qstr) -> *const Cvar,
    cvar_forceset: extern fn(Qstr, Qstr) -> *const Cvar,
    argc: extern fn() -> i32,
    argv: extern fn(i32) -> Qstr,
    args: extern fn() -> Qstr,
    add_command_string: extern fn(Qstr),
    debug_graph: extern fn(f32, i32)
}

#[repr(C)]
pub struct GameExport {
    pub apiversion: i32,
    pub init: extern fn(),
    pub shutdown: extern fn(),
    pub spawn_entities: extern fn(Qstr, Qstr, Qstr),

    pub write_game: extern fn(Qstr, i32),
    pub read_game: extern fn(Qstr),

    pub write_level: extern fn(Qstr),
    pub read_level: extern fn(Qstr),


    pub client_connect: extern fn(*mut Edict, Qstr) -> i32,
    pub client_begin: extern fn(*mut Edict),
    pub client_userinfo_changed: extern fn(*mut Edict, Qstr),
    pub client_disconnect: extern fn (*mut Edict),
    pub client_command: extern fn(*mut Edict),
    pub client_think: extern fn(*mut Edict, *mut UserCmd),

    pub run_frame: extern fn(),
    pub server_command: extern fn(),
    pub edicts: *mut Edict,
    pub edict_size: i32, // std::mem::sizeof<Edict>()
    pub num_edicts: i32,
    pub max_edicts: i32
}



impl GameExport {
    const fn new() -> GameExport {
        GameExport {
            apiversion: 3,
            init: callbacks::init,
            shutdown: callbacks::shutdown,
            spawn_entities: callbacks::spawn_entities,
            write_game: callbacks::write_game,
            read_game: callbacks::read_game,
            write_level: callbacks::write_level,
            read_level: callbacks::read_level,
            client_connect: callbacks::client_connect,
            client_begin: callbacks::client_begin,
            client_userinfo_changed: callbacks::client_userinfo_changed,
            client_disconnect: callbacks::client_disconnect,
            client_command: callbacks::client_command,
            client_think: callbacks::client_think,
            run_frame: callbacks::run_frame,
            server_command: callbacks::server_command,
            edicts: 0 as *mut Edict,
            edict_size: std::mem::size_of::<Edict>() as i32,
            num_edicts: 0,
            max_edicts: 1024, // pretty much never changes
        }
    }
}

static mut GI: Option<GameImport> = None;
static mut GE: GameExport = GameExport::new();

#[no_mangle]
pub extern fn GetGameAPI(gi: *mut GameImport) -> *const GameExport {
    assert_ne!(gi as *const GameImport, null());
    panic::set_hook(Box::new(|pi| {
        q2::error(format!("error: {}", pi))
    }));
    unsafe {
        /* copy it out because this will not be valid forever lol */
        GI = Some((*gi).clone());

        /* quake 2 is extremely single threaded, so... sorry rust. */
        &GE as *const GameExport
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // #[test]
//     // fn it_works() {
//     //     let result = add(2, 2);
//     //     assert_eq!(result, 4);
//     // }
// }
