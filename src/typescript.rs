use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Replay")]
    pub type Replay;
}

#[wasm_bindgen(typescript_custom_section)]
pub const TYPESCRIPT: &'static str = r#"
// Type polyfills
type i8 = number
type i32 = number
type i64 = number
type u8 = number
type u16 = number
type u32 = number
type u64 = number
type f32 = number
type bool = boolean
type Option<T> = T | null
type Vec<T> = Array<T>

/* ********************** src/models.rs ********************** */

// The structure that a rocket league replay is parsed into.
export type Replay = {
    header_size: i32
    header_crc: u32
    major_version: i32
    minor_version: i32
    net_version: Option<i32>
    game_type: string

    // Could use a map to represent properties but I don't want to assume that duplicate keys
    // can't exist, so to be safe, use a traditional vector.
    properties: { [key: string]: HeaderProp }
    content_size: i32
    content_crc: u32
    network_frames: Option<NetworkFrames>
    levels: Vec<string>
    keyframes: Vec<KeyFrame>
    debug_info: Vec<DebugInfo>
    tick_marks: Vec<TickMark>
    packages: Vec<string>
    objects: Vec<string>
    names: Vec<string>
    class_indices: Vec<ClassIndex>
    net_cache: Vec<ClassNetCache>
}

// The frames decoded from the network data
export type NetworkFrames = {
    frames: Array<Frame>
}

// In Rocket league replays, there are tickmarks that typically represent a significant event in
// the game (eg. a goal). The tick mark is placed before the event happens so there is a ramp-up
// time. For instance, a tickmark could be at frame 396 for a goal at frame 441. At 30 fps, this
// would be 1.5 seconds of ramp up time.
export type TickMark = {
    description: string
    frame: i32
}

// Keyframes as defined by the video compression section in the [wikipedia][] article, are the
// main frames that are derived from in the following frame data. The key frames decoded will
// match up with the frames decoded from the network data.
//
// [wikipedia]: https://en.wikipedia.org/wiki/Key_frame#Video_compression
export type KeyFrame = {
    time: f32
    frame: i32
    position: i32
}

// All the interesting data are stored as properties in the header, properties such as:
//
// - When and who scored a goal
// - Player stats (goals, assists, score, etc).
// - Date and level played on
//
// A property can be a number, string, or a more complex object such as an array containing
// additional properties.
export type HeaderProp = any

// Debugging info stored in the replay if debugging is enabled.
export type DebugInfo = {
    frame: i32
    user: string
    text: string
}

// A mapping between an object's name and its index. Largely redundant
export type ClassIndex = {
    class: string
    index: i32
}

// A mapping between an object (that's an attribute)'s index and what its id will be when encoded
// in the network data
export type CacheProp = {
    object_ind: i32
    stream_id: i32
}

// Contains useful information when decoding the network stream
export type ClassNetCache = {
    object_ind: i32
    parent_id: i32
    cache_id: i32
    properties: Array<CacheProp>
}


/* ********************** src/network/models.rs ********************** */

export type Vector3f = {
    x: f32
    y: f32
    z: f32
}

// An object's current vector
export type Vector3i = {
    x: i32
    y: i32
    z: i32
}

export type Quaternion = {
    x: f32
    y: f32
    z: f32
    w: f32
}

// An object's current rotation
export type Rotation = {
    yaw: Option<i8>
    pitch: Option<i8>
    roll: Option<i8>
}

// Notifies that an actor has had one of their properties updated (most likely their rigid body
// state (location / rotation) has changed)
export type UpdatedAttribute = {
    // The actor that had an attribute updated
    actor_id: ActorId

    // The attribute stream id that was decoded
    stream_id: StreamId

    // The attribute's object id
    object_id: ObjectId

    // The actual data from the decoded attribute
    attribute: Attribute
}

// Contains the time and any new information that occurred during a frame
export type Frame = {
    // The time in seconds that the frame is recorded at
    time: f32

    // Time difference between previous frame
    delta: f32

    // List of new actors seen during the frame
    new_actors: Vec<NewActor>

    // List of actor id's that are deleted / destroyed
    deleted_actors: Vec<ActorId>

    // List of properties updated on the actors
    updated_actors: Vec<UpdatedAttribute>
}

// A replay encodes a list of objects that appear in the network data. The index of an object in
// this list is used as a key in many places: reconstructing the attribute hierarchy and new
// actors in the network data.
export type ObjectId = i32

// A `StreamId` is an attribute's object id in the network data. It is a more compressed form of
// the object id. Whereas the an object id might need to take up 9 bits, a stream id may only take
// up 6 bits.
export type StreamId = i32

// An actor in the network data stream. Could identify a ball, car, etc. Ids are not unique
// across a replay (eg. an actor that is destroyed may have its id repurposed).
export type ActorId = i32

// Information for a new actor that appears in the game
export type NewActor = {
    // The id given to the new actor
    actor_id: ActorId

    // An name id
    name_id: Option<i32>

    // The actor's object id.
    object_id: ObjectId

    // The initial trajectory of the new actor
    initial_trajectory: Trajectory
}

// Contains the optional location and rotation of an object when it spawns
export type Trajectory = {
    location: Option<Vector3i>
    rotation: Option<Rotation>
}


/* ********************** src/network/attributes.rs ********************** */

// IMPORTANT: Only one is present at a time, due to rust enum
// The attributes for updated actors in the network data.
//
// The vast majority of attributes in the network data are rigid bodies. As a performance
// improvent, any attribute variant larger than the size of a rigid body is moved to the heap (ie:
// `Box::new`). This change increased throughput by 40%.
export type Attribute = {
    Boolean?: bool
    Byte?: u8
    AppliedDamage?: AppliedDamage
    DamageState?: DamageState
    CamSettings?: CamSettings
    ClubColors?: ClubColors
    Demolish?: Demolish
    DemolishExtended?: DemolishExtended
    DemolishFx?: DemolishFx
    Enum?: u16
    Explosion?: Explosion
    ExtendedExplosion?: ExtendedExplosion
    FlaggedByte?: [bool, u8]
    ActiveActor?: ActiveActor
    Float?: f32
    GameMode?: [u8, u8]
    Int?: i32

    Int64?: i64
    Loadout?: Loadout
    TeamLoadout?: TeamLoadout
    Location?: Vector3f
    MusicStinger?: MusicStinger
    PlayerHistoryKey?: u16
    Pickup?: Pickup
    PickupNew?: PickupNew

    QWord?: u64
    Welded?: Welded
    Title?: [bool, bool, u32, u32, u32, u32, u32, bool]
    TeamPaint?: TeamPaint
    RigidBody?: RigidBody
    String?: String
    UniqueId?: UniqueId
    Reservation?: Reservation
    PartyLeader?: Option<UniqueId>
    PrivateMatch?: PrivateMatchSettings
    LoadoutOnline?: Vec<Vec<Product>>
    LoadoutsOnline?: LoadoutsOnline
    StatEvent?: StatEvent
    Rotation?: Rotation
    RepStatTitle?: RepStatTitle
    PickupInfo?: PickupInfo
    Impulse?: Impulse
    ReplicatedBoost?: ReplicatedBoost
    LogoData?: LogoData
}

export type ActiveActor = {
    active: bool
    actor: ActorId
}

export type CamSettings = {
    fov: f32
    height: f32
    angle: f32
    distance: f32
    stiffness: f32
    swivel: f32
    transition: Option<f32>
}

export type ClubColors = {
    blue_flag: bool
    blue_color: u8
    orange_flag: bool
    orange_color: u8
}

export type AppliedDamage = {
    id: u8
    position: Vector3f
    damage_index: i32
    total_damage: i32
}

export type DamageState = {
    // State of the dropshot tile (0 - undamaged, 1 - damaged, 2 - destroyed)
    tile_state: u8

    // True if damaged
    damaged: bool

    // Player actor that inflicted the damage
    offender: ActorId

    // Position of the ball at the time of the damage
    ball_position: Vector3f

    // True for the dropshot tile that was hit by the ball (center tile of the damage area)
    direct_hit: bool
    unknown1: bool
}

export type Demolish = {
    attacker_flag: bool
    attacker: ActorId
    victim_flag: bool
    victim: ActorId
    attack_velocity: Vector3f
    victim_velocity: Vector3f
}

export type DemolishExtended = {
    attacker_pri: ActiveActor, // player replication info
    self_demo: ActiveActor
    self_demolish: bool
    goal_explosion_owner: ActiveActor
    attacker: ActiveActor
    victim: ActiveActor
    attacker_velocity: Vector3f
    victim_velocity: Vector3f
}

export type DemolishFx = {
    custom_demo_flag: bool
    custom_demo_id: i32
    attacker_flag: bool
    attacker: ActorId
    victim_flag: bool
    victim: ActorId
    attack_velocity: Vector3f
    victim_velocity: Vector3f
}

export type Explosion = {
    flag: bool
    actor: ActorId
    location: Vector3f
}

export type ExtendedExplosion = {
    explosion: Explosion
    unknown1: bool
    secondary_actor: ActorId
}

export type Loadout = {
    version: u8
    body: u32
    decal: u32
    wheels: u32
    rocket_trail: u32
    antenna: u32
    topper: u32
    unknown1: u32
    unknown2: Option<u32>
    engine_audio: Option<u32>
    trail: Option<u32>
    goal_explosion: Option<u32>
    banner: Option<u32>
    product_id: Option<u32>
}

export type TeamLoadout = {
    blue: Loadout
    orange: Loadout
}

export type StatEvent = {
    unknown1: bool
    object_id: i32
}

export type MusicStinger = {
    flag: bool
    cue: u32
    trigger: u8
}

export type Pickup = {
    instigator: Option<ActorId>
    picked_up: bool
}

export type PickupNew = {
    instigator: Option<ActorId>
    picked_up: u8
}

export type Welded = {
    active: bool
    actor: ActorId
    offset: Vector3f
    mass: f32
    rotation: Rotation
}

export type TeamPaint = {
    team: u8
    primary_color: u8
    accent_color: u8
    primary_finish: u32
    accent_finish: u32
}

export type RigidBody = {
    sleeping: bool
    location: Vector3f
    rotation: Quaternion
    linear_velocity: Option<Vector3f>
    angular_velocity: Option<Vector3f>
}

export type UniqueId = {
    system_id: u8
    remote_id: RemoteId
    local_id: u8
}

export type PsyNetId = {
    online_id: u64
    unknown1: Vec<u8>
}

export type SwitchId = {
    online_id: u64
    unknown1: Vec<u8>
}

export type Ps4Id = {
    online_id: u64
    name: String
    unknown1: Vec<u8>
}

// IMPORTANT: Only one is present at a time, due to rust enum
export type RemoteId = {
    PlayStation?: Ps4Id
    PsyNet?: PsyNetId
    SplitScreen?: u32

    Steam?: u64
    Switch?: SwitchId

    Xbox?: u64

    QQ?: u64
    Epic?: String
}

export type Reservation = {
    number: u32
    unique_id: UniqueId
    name: Option<String>
    unknown1: bool
    unknown2: bool
    unknown3: Option<u8>
}

export type PrivateMatchSettings = {
    mutators: String
    joinable_by: u32
    max_players: u32
    game_name: String
    password: String
    flag: bool
}

export type Product = {
    unknown: bool
    object_ind: ObjectId
    value: ProductValue
}

export type LoadoutsOnline = {
    blue: Vec<Vec<Product>>
    orange: Vec<Vec<Product>>
    unknown1: bool
    unknown2: bool
}

// IMPORTANT: Only one is present at a time, due to rust enum
export type ProductValue = "NoColor" | "Absent" | {
    OldColor?: u32
    NewColor?: u32
    OldPaint?: u32
    NewPaint?: u32
    Title?: string
    SpecialEdition?: u32
    OldTeamEdition?: u32
    NewTeamEdition?: u32
}

export type RepStatTitle = {
    unknown: bool
    name: String
    unknown2: bool
    index: u32
    value: u32
}

export type PickupInfo = {
    active: bool
    actor: ActorId
    items_are_preview: bool
    unknown: bool
    unknown2: bool
}

export type Impulse = {
    compressed_rotation: i32
    speed: f32
}

export type ReplicatedBoost = {
    grant_count: u8
    boost_amount: u8
    unused1: u8
    unused2: u8
}

export type LogoData = {
    logo_id: u32
    swap_colors: bool
}"#;
