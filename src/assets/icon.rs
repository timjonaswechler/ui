use bevy::prelude::*;

const COLS: u32 = 20;
const ROWS: u32 = 16;
#[derive(Resource)]
pub struct IconAtlases {
    pub texture_16: Handle<Image>,
    pub texture_24: Handle<Image>,
    pub texture_32: Handle<Image>,
    pub texture_64: Handle<Image>,
    pub layout_16: Handle<TextureAtlasLayout>,
    pub layout_24: Handle<TextureAtlasLayout>,
    pub layout_32: Handle<TextureAtlasLayout>,
    pub layout_64: Handle<TextureAtlasLayout>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconSize {
    Small = 16,
    Medium = 24,
    Large = 32,
    ExtraLarge = 64,
}

impl IconSize {
    pub fn as_val(&self) -> Val {
        Val::Px(*self as u8 as f32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconId {
    // UI Navigation Icons (0-19)
    Accessibility = 0,
    ActivityLog = 1,
    AlignBaseline = 2,
    AlignBottom = 3,
    AlignCenterHorizontally = 4,
    AlignCenterVertically = 5,
    AlignLeft = 6,
    AlignRight = 7,
    AlignTop = 8,
    AllSides = 9,
    Angle = 10,
    Archive = 11,
    ArrowBottomLeft = 12,
    ArrowBottomRight = 13,
    ArrowDown = 14,
    ArrowLeft = 15,
    ArrowRight = 16,
    ArrowTopLeft = 17,
    ArrowTopRight = 18,
    ArrowUp = 19,

    // UI Elements (20-39)
    AspectRatio = 20,
    Avatar = 21,
    Backpack = 22,
    Badge = 23,
    BarChart = 24,
    Bell = 25,
    BlendingMode = 26,
    BookmarkFilled = 27,
    Bookmark = 28,
    BorderAll = 29,
    BorderBottom = 30,
    BorderDashed = 31,
    BorderDotted = 32,
    BorderLeft = 33,
    BorderNone = 34,
    BorderRight = 35,
    BorderSolid = 36,
    BorderSplit = 37,
    BorderStyle = 38,
    BorderTop = 39,

    // Layout & Controls (40-59)
    BorderWidth = 40,
    BoxModel = 41,
    Box = 42,
    Button = 43,
    Calendar = 44,
    Camera = 45,
    CardStackMinus = 46,
    CardStackPlus = 47,
    CardStack = 48,
    CaretDown = 49,
    CaretLeft = 50,
    CaretRight = 51,
    CaretSort = 52,
    CaretUp = 53,
    ChatBubble = 54,
    CheckCircled = 55,
    Check = 56,
    Checkbox = 57,
    ChevronDown = 58,
    ChevronLeft = 59,

    // Actions & UI (60-79)
    ChevronRight = 60,
    ChevronUp = 61,
    CircleBackslash = 62,
    Circle = 63,
    ClipboardCopy = 64,
    Clipboard = 65,
    Clock = 66,
    Code = 67,
    ColorWheel = 68,
    ColumnSpacing = 69,
    Columns = 70,
    Commit = 71,
    Component1 = 72,
    Component2 = 73,
    ComponentBoolean = 74,
    ComponentInstance = 75,
    ComponentNone = 76,
    ComponentPlaceholder = 77,
    Container = 78,
    Cookie = 79,

    // File & Data (80-99)
    Copy = 80,
    CornerBottomLeft = 81,
    CornerBottomRight = 82,
    CornerTopLeft = 83,
    CornerTopRight = 84,
    Corners = 85,
    CountdownTimer = 86,
    CounterClockwiseClock = 87,
    Crop = 88,
    Cross1 = 89,
    Cross2 = 90,
    CrossCircled = 91,
    Crosshair1 = 92,
    Crosshair2 = 93,
    CrumpledPaper = 94,
    Cube = 95,
    CursorArrow = 96,
    CursorText = 97,
    Dash = 98,
    Dashboard = 99,

    // System & Media (100-119)
    Desktop = 100,
    Dimensions = 101,
    Disc = 102,
    DiscordLogo = 103,
    DividerHorizontal = 104,
    DividerVertical = 105,
    DotFilled = 106,
    Dot = 107,
    DotsHorizontal = 108,
    DotsVertical = 109,
    DoubleArrowDown = 110,
    DoubleArrowLeft = 111,
    DoubleArrowRight = 112,
    DoubleArrowUp = 113,
    Download = 114,
    DragHandleDots1 = 115,
    DragHandleDots2 = 116,
    DragHandleHorizontal = 117,
    DragHandleVertical = 118,
    DrawingPinFilled = 119,

    // Drawing & Design (120-139)
    DrawingPin = 120,
    DropdownMenu = 121,
    EnterFullScreen = 122,
    Enter = 123,
    EnvelopeClosed = 124,
    EnvelopeOpen = 125,
    Eraser = 126,
    ExclamationTriangle = 127,
    ExitFullScreen = 128,
    Exit = 129,
    ExternalLink = 130,
    EyeClosed = 131,
    EyeNone = 132,
    EyeOpen = 133,
    Face = 134,
    FigmaLogo = 135,
    FileMinus = 136,
    FilePlus = 137,
    FileText = 138,
    File = 139,

    // Font & Typography (140-159)
    FontBold = 140,
    FontFamily = 141,
    FontItalic = 142,
    FontRoman = 143,
    FontSize = 144,
    FontStyle = 145,
    Frame = 146,
    Gear = 147,
    GithubLogo = 148,
    Globe = 149,
    Grid = 150,
    Group = 151,
    Half1 = 152,
    Half2 = 153,
    HamburgerMenu = 154,
    Hand = 155,
    Heading = 156,
    HeartFilled = 157,
    Heart = 158,
    Height = 159,

    // User & Social (160-179)
    HobbyKnife = 160,
    Home = 161,
    IdCard = 162,
    Image = 163,
    InfoCircled = 164,
    Input = 165,
    Keyboard = 166,
    LapTimer = 167,
    Laptop = 168,
    Layers = 169,
    Layout = 170,
    LetterCaseCapitalize = 171,
    LetterCaseLowercase = 172,
    LetterCaseToggle = 173,
    LetterCaseUppercase = 174,
    LetterSpacing = 175,
    LightningBolt = 176,
    LineHeight = 177,
    Link1 = 178,
    Link2 = 179,

    // Links & Actions (180-199)
    LinkBreak1 = 180,
    LinkBreak2 = 181,
    LinkNone1 = 182,
    LinkNone2 = 183,
    ListBullet = 184,
    LockClosed = 185,
    LockOpen1 = 186,
    LockOpen2 = 187,
    Loop = 188,
    MagicWand = 189,
    MagnifyingGlass = 190,
    Margin = 191,
    MaskOff = 192,
    MaskOn = 193,
    MinusCircled = 194,
    Minus = 195,
    Mix = 196,
    MixerHorizontal = 197,
    MixerVertical = 198,
    Mobile = 199,

    // Motion & Tools (200-219)
    Moon = 200,
    Move = 201,
    Opacity = 202,
    OpenInNewWindow = 203,
    Overline = 204,
    Padding = 205,
    PaperPlane = 206,
    Pause = 207,
    Pencil1 = 208,
    Pencil2 = 209,
    Person = 210,
    PieChart = 211,
    Pilcrow = 212,
    PinBottom = 213,
    PinLeft = 214,
    PinRight = 215,
    PinTop = 216,
    Play = 217,
    PlusCircled = 218,
    Plus = 219,

    // Questions & Controls (220-239)
    QuestionMarkCircled = 220,
    QuestionMark = 221,
    Quote = 222,
    Radiobutton = 223,
    Reader = 224,
    Reload = 225,
    Reset = 226,
    Resume = 227,
    Rocket = 228,
    RotateCounterClockwise = 229,
    RowSpacing = 230,
    Rows = 231,
    RulerHorizontal = 232,
    RulerSquare = 233,
    Scissors = 234,
    Section = 235,
    SewingPinFilled = 236,
    SewingPin = 237,
    ShadowInner = 238,
    ShadowNone = 239,

    // Shadow & Share (240-259)
    ShadowOuter = 240,
    Shadow = 241,
    Share1 = 242,
    Share2 = 243,
    Shuffle = 244,
    Size = 245,
    Slash = 246,
    Slider = 247,
    SpaceBetweenHorizontally = 248,
    SpaceBetweenVertically = 249,
    SpaceEvenlyHorizontally = 250,
    SpaceEvenlyVertically = 251,
    SpeakerLoud = 252,
    SpeakerModerate = 253,
    SpeakerOff = 254,
    SpeakerQuiet = 255,
    Square = 256,
    Stack = 257,
    StarFilled = 258,
    Star = 259,

    // Status & Controls (260-279)
    Stop = 260,
    Stopwatch = 261,
    StretchHorizontally = 262,
    StretchVertically = 263,
    Strikethrough = 264,
    Sun = 265,
    Switch = 266,
    Symbol = 267,
    Table = 268,
    Target = 269,
    TextAlignBottom = 270,
    TextAlignCenter = 271,
    TextAlignJustify = 272,
    TextAlignLeft = 273,
    TextAlignMiddle = 274,
    TextAlignRight = 275,
    TextAlignTop = 276,
    TextNone = 277,
    Text = 278,
    ThickArrowDown = 279,

    // Arrows & Navigation (280-299)
    ThickArrowLeft = 280,
    ThickArrowRight = 281,
    ThickArrowUp = 282,
    Timer = 283,
    Tokens = 284,
    TrackNext = 285,
    TrackPrevious = 286,
    Transform = 287,
    Trash = 288,
    TriangleDown = 289,
    TriangleLeft = 290,
    TriangleRight = 291,
    TriangleUp = 292,
    Underline = 293,
    Update = 294,
    Upload = 295,
    ValueNone = 296,
    Value = 297,
    Video = 298,
    ViewGrid = 299,

    // Final Icons (300-305)
    ViewHorizontal = 300,
    ViewNone = 301,
    ViewVertical = 302,
    Width = 303,
    ZoomIn = 304,
    ZoomOut = 305,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Icon {
    pub id: IconId,
    pub size: IconSize,
    pub tint: Color,
}

#[derive(Bundle)]
pub struct IconBundle {
    pub icon: Icon,
    pub image: ImageNode,
    pub node: Node,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            id: IconId::Home,
            size: IconSize::Medium,
            tint: Color::WHITE,
        }
    }
}

impl Icon {
    pub fn new(id: IconId) -> Self {
        Self {
            id,
            size: IconSize::Medium,
            tint: Color::WHITE,
        }
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, color: Color) -> Self {
        self.tint = color;
        self
    }

    pub fn small(id: IconId) -> Self {
        Self::new(id).size(IconSize::Small)
    }

    pub fn medium(id: IconId) -> Self {
        Self::new(id).size(IconSize::Medium)
    }

    pub fn large(id: IconId) -> Self {
        Self::new(id).size(IconSize::Large)
    }
    pub fn extra_large(id: IconId) -> Self {
        Self::new(id).size(IconSize::ExtraLarge)
    }

    pub fn bundle(self, atlases: &IconAtlases) -> IconBundle {
        let (texture, layout) = match self.size {
            IconSize::Small => (atlases.texture_16.clone(), atlases.layout_16.clone()),
            IconSize::Medium => (atlases.texture_24.clone(), atlases.layout_24.clone()),
            IconSize::Large => (atlases.texture_32.clone(), atlases.layout_32.clone()),
            IconSize::ExtraLarge => (atlases.texture_64.clone(), atlases.layout_64.clone()),
        };

        IconBundle {
            icon: self,
            image: ImageNode::from_atlas_image(
                texture,
                TextureAtlas {
                    layout,
                    index: self.id as usize,
                },
            )
            .with_color(self.tint),
            node: Node {
                width: self.size.as_val(),
                height: self.size.as_val(),
                ..default()
            },
        }
    }

    pub fn spawn(self, commands: &mut Commands, atlases: &IconAtlases) -> Entity {
        commands.spawn(self.bundle(atlases)).id()
    }
}

pub struct IconBuilder;

impl IconBuilder {
    pub fn new(id: IconId) -> Icon {
        Icon::new(id)
    }

    pub fn small(id: IconId) -> Icon {
        Icon::small(id)
    }

    pub fn medium(id: IconId) -> Icon {
        Icon::medium(id)
    }

    pub fn large(id: IconId) -> Icon {
        Icon::large(id)
    }

    pub fn extra_large(id: IconId) -> Icon {
        Icon::extra_large(id)
    }
}

pub fn load_icon_atlases(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Create layouts for each icon size
    let layout_16 = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        COLS,
        ROWS,
        None,
        None,
    ));
    let layout_24 = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(24, 24),
        COLS,
        ROWS,
        None,
        None,
    ));
    let layout_32 = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        COLS,
        ROWS,
        None,
        None,
    ));
    let layout_64 = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        COLS,
        ROWS,
        None,
        None,
    ));

    commands.insert_resource(IconAtlases {
        texture_16: asset_server.load("ui/icons/generic/texture_atlas_20x16_16px.png"),
        texture_24: asset_server.load("ui/icons/generic/texture_atlas_20x16_24px.png"),
        texture_32: asset_server.load("ui/icons/generic/texture_atlas_20x16_32px.png"),
        texture_64: asset_server.load("ui/icons/generic/texture_atlas_20x16_64px.png"),
        layout_16,
        layout_24,
        layout_32,
        layout_64,
    });
}

pub fn icon_interaction_system(
    mut query: Query<(&Interaction, &mut ImageNode), (Changed<Interaction>, With<Icon>)>,
) {
    for (interaction, mut image_node) in &mut query {
        match interaction {
            Interaction::Hovered => {
                image_node.color = Color::srgb(0.8, 0.8, 1.0);
            }
            Interaction::Pressed => {
                image_node.color = Color::srgb(0.6, 0.6, 0.8);
            }
            Interaction::None => {
                image_node.color = Color::WHITE;
            }
        }
    }
}
