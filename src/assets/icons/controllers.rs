#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod generic {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::{FontFamily, TextSize};

    pub struct Generic;

    impl Generic {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content)
                .family(FontFamily::GenericController)
                .size(TextSize::Base)
        }
    }

    // ===== GENERIC CONTROLLER ICONS =====
    pub const Button: &str = "\u{e000}";
    pub const ButtonCircle: &str = "\u{e001}";
    pub const ButtonCircleFill: &str = "\u{e002}";
    pub const ButtonCircleOutline: &str = "\u{e003}";
    pub const ButtonFinger: &str = "\u{e004}";
    pub const ButtonFingerPressed: &str = "\u{e005}";
    pub const ButtonPressed: &str = "\u{e006}";
    pub const ButtonSquare: &str = "\u{e007}";
    pub const ButtonSquareFill: &str = "\u{e008}";
    pub const ButtonSquareOutline: &str = "\u{e009}";
    pub const ButtonTriggerA: &str = "\u{e00a}";
    pub const ButtonTriggerAFill: &str = "\u{e00b}";
    pub const ButtonTriggerAOutline: &str = "\u{e00c}";
    pub const ButtonTriggerB: &str = "\u{e00d}";
    pub const ButtonTriggerBFill: &str = "\u{e00e}";
    pub const ButtonTriggerBOutline: &str = "\u{e00f}";
    pub const ButtonTriggerC: &str = "\u{e010}";
    pub const ButtonTriggerCFill: &str = "\u{e011}";
    pub const ButtonTriggerCOutline: &str = "\u{e012}";
    pub const Joystick: &str = "\u{e013}";
    pub const JoystickHorizontal: &str = "\u{e014}";
    pub const JoystickLeft: &str = "\u{e015}";
    pub const JoystickRed: &str = "\u{e016}";
    pub const JoystickRedHorizontal: &str = "\u{e017}";
    pub const JoystickRedLeft: &str = "\u{e018}";
    pub const JoystickRedRight: &str = "\u{e019}";
    pub const JoystickRight: &str = "\u{e01a}";
    pub const Stick: &str = "\u{e01b}";
    pub const StickDown: &str = "\u{e01c}";
    pub const StickHorizontal: &str = "\u{e01d}";
    pub const StickLeft: &str = "\u{e01e}";
    pub const StickPress: &str = "\u{e01f}";
    pub const StickRight: &str = "\u{e020}";
    pub const StickSide: &str = "\u{e021}";
    pub const StickUp: &str = "\u{e022}";
    pub const StickVertical: &str = "\u{e023}";
}

// ===== KEYBOARD & MOUSE ICONS =====
pub mod keyboard_mouse {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct KeyboardMouse;

    impl KeyboardMouse {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::KeyboardMouse)
        }
    }
    pub const Keyboard: &str = "\u{e000}";
    pub const Keyboard0: &str = "\u{e001}";
    pub const Keyboard0Outline: &str = "\u{e002}";
    pub const Keyboard1: &str = "\u{e003}";
    pub const Keyboard1Outline: &str = "\u{e004}";
    pub const Keyboard2: &str = "\u{e005}";
    pub const Keyboard2Outline: &str = "\u{e006}";
    pub const Keyboard3: &str = "\u{e007}";
    pub const Keyboard3Outline: &str = "\u{e008}";
    pub const Keyboard4: &str = "\u{e009}";
    pub const Keyboard4Outline: &str = "\u{e00a}";
    pub const Keyboard5: &str = "\u{e00b}";
    pub const Keyboard5Outline: &str = "\u{e00c}";
    pub const Keyboard6: &str = "\u{e00d}";
    pub const Keyboard6Outline: &str = "\u{e00e}";
    pub const Keyboard7: &str = "\u{e00f}";
    pub const Keyboard7Outline: &str = "\u{e010}";
    pub const Keyboard8: &str = "\u{e011}";
    pub const Keyboard8Outline: &str = "\u{e012}";
    pub const Keyboard9: &str = "\u{e013}";
    pub const Keyboard9Outline: &str = "\u{e014}";
    pub const KeyboardA: &str = "\u{e015}";
    pub const KeyboardAOutline: &str = "\u{e016}";
    pub const KeyboardAlt: &str = "\u{e017}";
    pub const KeyboardAltOutline: &str = "\u{e018}";
    pub const KeyboardAny: &str = "\u{e019}";
    pub const KeyboardAnyOutline: &str = "\u{e01a}";
    pub const KeyboardApostrophe: &str = "\u{e01b}";
    pub const KeyboardApostropheOutline: &str = "\u{e01c}";
    pub const KeyboardArrowDown: &str = "\u{e01d}";
    pub const KeyboardArrowDownOutline: &str = "\u{e01e}";
    pub const KeyboardArrowLeft: &str = "\u{e01f}";
    pub const KeyboardArrowLeftOutline: &str = "\u{e020}";
    pub const KeyboardArrowRight: &str = "\u{e021}";
    pub const KeyboardArrowRightOutline: &str = "\u{e022}";
    pub const KeyboardArrowUp: &str = "\u{e023}";
    pub const KeyboardArrowUpOutline: &str = "\u{e024}";
    pub const KeyboardArrows: &str = "\u{e025}";
    pub const KeyboardArrowsAll: &str = "\u{e026}";
    pub const KeyboardArrowsDown: &str = "\u{e027}";
    pub const KeyboardArrowsDownOutline: &str = "\u{e028}";
    pub const KeyboardArrowsHorizontal: &str = "\u{e029}";
    pub const KeyboardArrowsHorizontalOutline: &str = "\u{e02a}";
    pub const KeyboardArrowsLeft: &str = "\u{e02b}";
    pub const KeyboardArrowsLeftOutline: &str = "\u{e02c}";
    pub const KeyboardArrowsNone: &str = "\u{e02d}";
    pub const KeyboardArrowsRight: &str = "\u{e02e}";
    pub const KeyboardArrowsRightOutline: &str = "\u{e02f}";
    pub const KeyboardArrowsUp: &str = "\u{e030}";
    pub const KeyboardArrowsUpOutline: &str = "\u{e031}";
    pub const KeyboardArrowsVertical: &str = "\u{e032}";
    pub const KeyboardArrowsVerticalOutline: &str = "\u{e033}";
    pub const KeyboardAsterisk: &str = "\u{e034}";
    pub const KeyboardAsteriskOutline: &str = "\u{e035}";
    pub const KeyboardB: &str = "\u{e036}";
    pub const KeyboardBOutline: &str = "\u{e037}";
    pub const KeyboardBackspace: &str = "\u{e038}";
    pub const KeyboardBackspaceIcon: &str = "\u{e039}";
    pub const KeyboardBackspaceIconAlternative: &str = "\u{e03a}";
    pub const KeyboardBackspaceIconAlternativeOutline: &str = "\u{e03b}";
    pub const KeyboardBackspaceIconOutline: &str = "\u{e03c}";
    pub const KeyboardBackspaceOutline: &str = "\u{e03d}";
    pub const KeyboardBracketClose: &str = "\u{e03e}";
    pub const KeyboardBracketCloseOutline: &str = "\u{e03f}";
    pub const KeyboardBracketGreater: &str = "\u{e040}";
    pub const KeyboardBracketGreaterOutline: &str = "\u{e041}";
    pub const KeyboardBracketLess: &str = "\u{e042}";
    pub const KeyboardBracketLessOutline: &str = "\u{e043}";
    pub const KeyboardBracketOpen: &str = "\u{e044}";
    pub const KeyboardBracketOpenOutline: &str = "\u{e045}";
    pub const KeyboardC: &str = "\u{e046}";
    pub const KeyboardCOutline: &str = "\u{e047}";
    pub const KeyboardCapslock: &str = "\u{e048}";
    pub const KeyboardCapslockIcon: &str = "\u{e049}";
    pub const KeyboardCapslockIconOutline: &str = "\u{e04a}";
    pub const KeyboardCapslockOutline: &str = "\u{e04b}";
    pub const KeyboardCaret: &str = "\u{e04c}";
    pub const KeyboardCaretOutline: &str = "\u{e04d}";
    pub const KeyboardColon: &str = "\u{e04e}";
    pub const KeyboardColonOutline: &str = "\u{e04f}";
    pub const KeyboardComma: &str = "\u{e050}";
    pub const KeyboardCommaOutline: &str = "\u{e051}";
    pub const KeyboardCommand: &str = "\u{e052}";
    pub const KeyboardCommandOutline: &str = "\u{e053}";
    pub const KeyboardCtrl: &str = "\u{e054}";
    pub const KeyboardCtrlOutline: &str = "\u{e055}";
    pub const KeyboardD: &str = "\u{e056}";
    pub const KeyboardDOutline: &str = "\u{e057}";
    pub const KeyboardDelete: &str = "\u{e058}";
    pub const KeyboardDeleteOutline: &str = "\u{e059}";
    pub const KeyboardE: &str = "\u{e05a}";
    pub const KeyboardEOutline: &str = "\u{e05b}";
    pub const KeyboardEnd: &str = "\u{e05c}";
    pub const KeyboardEndOutline: &str = "\u{e05d}";
    pub const KeyboardEnter: &str = "\u{e05e}";
    pub const KeyboardEnterOutline: &str = "\u{e05f}";
    pub const KeyboardEquals: &str = "\u{e060}";
    pub const KeyboardEqualsOutline: &str = "\u{e061}";
    pub const KeyboardEscape: &str = "\u{e062}";
    pub const KeyboardEscapeOutline: &str = "\u{e063}";
    pub const KeyboardExclamation: &str = "\u{e064}";
    pub const KeyboardExclamationOutline: &str = "\u{e065}";
    pub const KeyboardF: &str = "\u{e066}";
    pub const KeyboardF1: &str = "\u{e067}";
    pub const KeyboardF10: &str = "\u{e068}";
    pub const KeyboardF10Outline: &str = "\u{e069}";
    pub const KeyboardF11: &str = "\u{e06a}";
    pub const KeyboardF11Outline: &str = "\u{e06b}";
    pub const KeyboardF12: &str = "\u{e06c}";
    pub const KeyboardF12Outline: &str = "\u{e06d}";
    pub const KeyboardF1Outline: &str = "\u{e06e}";
    pub const KeyboardF2: &str = "\u{e06f}";
    pub const KeyboardF2Outline: &str = "\u{e070}";
    pub const KeyboardF3: &str = "\u{e071}";
    pub const KeyboardF3Outline: &str = "\u{e072}";
    pub const KeyboardF4: &str = "\u{e073}";
    pub const KeyboardF4Outline: &str = "\u{e074}";
    pub const KeyboardF5: &str = "\u{e075}";
    pub const KeyboardF5Outline: &str = "\u{e076}";
    pub const KeyboardF6: &str = "\u{e077}";
    pub const KeyboardF6Outline: &str = "\u{e078}";
    pub const KeyboardF7: &str = "\u{e079}";
    pub const KeyboardF7Outline: &str = "\u{e07a}";
    pub const KeyboardF8: &str = "\u{e07b}";
    pub const KeyboardF8Outline: &str = "\u{e07c}";
    pub const KeyboardF9: &str = "\u{e07d}";
    pub const KeyboardF9Outline: &str = "\u{e07e}";
    pub const KeyboardFOutline: &str = "\u{e07f}";
    pub const KeyboardFunction: &str = "\u{e080}";
    pub const KeyboardFunctionOutline: &str = "\u{e081}";
    pub const KeyboardG: &str = "\u{e082}";
    pub const KeyboardGOutline: &str = "\u{e083}";
    pub const KeyboardH: &str = "\u{e084}";
    pub const KeyboardHOutline: &str = "\u{e085}";
    pub const KeyboardHome: &str = "\u{e086}";
    pub const KeyboardHomeOutline: &str = "\u{e087}";
    pub const KeyboardI: &str = "\u{e088}";
    pub const KeyboardIOutline: &str = "\u{e089}";
    pub const KeyboardInsert: &str = "\u{e08a}";
    pub const KeyboardInsertOutline: &str = "\u{e08b}";
    pub const KeyboardJ: &str = "\u{e08c}";
    pub const KeyboardJOutline: &str = "\u{e08d}";
    pub const KeyboardK: &str = "\u{e08e}";
    pub const KeyboardKOutline: &str = "\u{e08f}";
    pub const KeyboardL: &str = "\u{e090}";
    pub const KeyboardLOutline: &str = "\u{e091}";
    pub const KeyboardM: &str = "\u{e092}";
    pub const KeyboardMOutline: &str = "\u{e093}";
    pub const KeyboardMinus: &str = "\u{e094}";
    pub const KeyboardMinusOutline: &str = "\u{e095}";
    pub const KeyboardN: &str = "\u{e096}";
    pub const KeyboardNOutline: &str = "\u{e097}";
    pub const KeyboardNumlock: &str = "\u{e098}";
    pub const KeyboardNumlockOutline: &str = "\u{e099}";
    pub const KeyboardNumpadEnter: &str = "\u{e09a}";
    pub const KeyboardNumpadEnterOutline: &str = "\u{e09b}";
    pub const KeyboardNumpadPlus: &str = "\u{e09c}";
    pub const KeyboardNumpadPlusOutline: &str = "\u{e09d}";
    pub const KeyboardO: &str = "\u{e09e}";
    pub const KeyboardOOutline: &str = "\u{e09f}";
    pub const KeyboardOption: &str = "\u{e0a0}";
    pub const KeyboardOptionOutline: &str = "\u{e0a1}";
    pub const KeyboardOutline: &str = "\u{e0a2}";
    pub const KeyboardP: &str = "\u{e0a3}";
    pub const KeyboardPOutline: &str = "\u{e0a4}";
    pub const KeyboardPageDown: &str = "\u{e0a5}";
    pub const KeyboardPageDownOutline: &str = "\u{e0a6}";
    pub const KeyboardPageUp: &str = "\u{e0a7}";
    pub const KeyboardPageUpOutline: &str = "\u{e0a8}";
    pub const KeyboardPeriod: &str = "\u{e0a9}";
    pub const KeyboardPeriodOutline: &str = "\u{e0aa}";
    pub const KeyboardPlus: &str = "\u{e0ab}";
    pub const KeyboardPlusOutline: &str = "\u{e0ac}";
    pub const KeyboardPrintscreen: &str = "\u{e0ad}";
    pub const KeyboardPrintscreenOutline: &str = "\u{e0ae}";
    pub const KeyboardQ: &str = "\u{e0af}";
    pub const KeyboardQOutline: &str = "\u{e0b0}";
    pub const KeyboardQuestion: &str = "\u{e0b1}";
    pub const KeyboardQuestionOutline: &str = "\u{e0b2}";
    pub const KeyboardQuote: &str = "\u{e0b3}";
    pub const KeyboardQuoteOutline: &str = "\u{e0b4}";
    pub const KeyboardR: &str = "\u{e0b5}";
    pub const KeyboardROutline: &str = "\u{e0b6}";
    pub const KeyboardReturn: &str = "\u{e0b7}";
    pub const KeyboardReturnOutline: &str = "\u{e0b8}";
    pub const KeyboardS: &str = "\u{e0b9}";
    pub const KeyboardSOutline: &str = "\u{e0ba}";
    pub const KeyboardSemicolon: &str = "\u{e0bb}";
    pub const KeyboardSemicolonOutline: &str = "\u{e0bc}";
    pub const KeyboardShift: &str = "\u{e0bd}";
    pub const KeyboardShiftIcon: &str = "\u{e0be}";
    pub const KeyboardShiftIconOutline: &str = "\u{e0bf}";
    pub const KeyboardShiftOutline: &str = "\u{e0c0}";
    pub const KeyboardSlashBack: &str = "\u{e0c1}";
    pub const KeyboardSlashBackOutline: &str = "\u{e0c2}";
    pub const KeyboardSlashForward: &str = "\u{e0c3}";
    pub const KeyboardSlashForwardOutline: &str = "\u{e0c4}";
    pub const KeyboardSpace: &str = "\u{e0c5}";
    pub const KeyboardSpaceIcon: &str = "\u{e0c6}";
    pub const KeyboardSpaceIconOutline: &str = "\u{e0c7}";
    pub const KeyboardSpaceOutline: &str = "\u{e0c8}";
    pub const KeyboardT: &str = "\u{e0c9}";
    pub const KeyboardTOutline: &str = "\u{e0ca}";
    pub const KeyboardTab: &str = "\u{e0cb}";
    pub const KeyboardTabIcon: &str = "\u{e0cc}";
    pub const KeyboardTabIconAlternative: &str = "\u{e0cd}";
    pub const KeyboardTabIconAlternativeOutline: &str = "\u{e0ce}";
    pub const KeyboardTabIconOutline: &str = "\u{e0cf}";
    pub const KeyboardTabOutline: &str = "\u{e0d0}";
    pub const KeyboardTilde: &str = "\u{e0d1}";
    pub const KeyboardTildeOutline: &str = "\u{e0d2}";
    pub const KeyboardU: &str = "\u{e0d3}";
    pub const KeyboardUOutline: &str = "\u{e0d4}";
    pub const KeyboardV: &str = "\u{e0d5}";
    pub const KeyboardVOutline: &str = "\u{e0d6}";
    pub const KeyboardW: &str = "\u{e0d7}";
    pub const KeyboardWOutline: &str = "\u{e0d8}";
    pub const KeyboardWin: &str = "\u{e0d9}";
    pub const KeyboardWinOutline: &str = "\u{e0da}";
    pub const KeyboardX: &str = "\u{e0db}";
    pub const KeyboardXOutline: &str = "\u{e0dc}";
    pub const KeyboardY: &str = "\u{e0dd}";
    pub const KeyboardYOutline: &str = "\u{e0de}";
    pub const KeyboardZ: &str = "\u{e0df}";
    pub const KeyboardZOutline: &str = "\u{e0e0}";
    pub const Mouse: &str = "\u{e0e1}";
    pub const MouseHorizontal: &str = "\u{e0e2}";
    pub const MouseLeft: &str = "\u{e0e3}";
    pub const MouseLeftOutline: &str = "\u{e0e4}";
    pub const MouseMove: &str = "\u{e0e5}";
    pub const MouseOutline: &str = "\u{e0e6}";
    pub const MouseRight: &str = "\u{e0e7}";
    pub const MouseRightOutline: &str = "\u{e0e8}";
    pub const MouseScroll: &str = "\u{e0e9}";
    pub const MouseScrollDown: &str = "\u{e0ea}";
    pub const MouseScrollDownOutline: &str = "\u{e0eb}";
    pub const MouseScrollOutline: &str = "\u{e0ec}";
    pub const MouseScrollUp: &str = "\u{e0ed}";
    pub const MouseScrollUpOutline: &str = "\u{e0ee}";
    pub const MouseScrollVertical: &str = "\u{e0ef}";
    pub const MouseScrollVerticalOutline: &str = "\u{e0f0}";
    pub const MouseSmall: &str = "\u{e0f1}";
    pub const MouseVertical: &str = "\u{e0f2}";
}

// ===== NINTENDO SWITCH ICONS =====
pub mod switch {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct Switch;

    impl Switch {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::NintendoSwitch)
        }
    }

    pub const ControllerSwitch: &str = "\u{e000}";
    pub const ControllerSwitchJoyconDown: &str = "\u{e001}";
    pub const ControllerSwitchJoyconUp: &str = "\u{e002}";
    pub const ControllerSwitchPro: &str = "\u{e003}";
    pub const ButtonA: &str = "\u{e004}";
    pub const ButtonAOutline: &str = "\u{e005}";
    pub const ButtonB: &str = "\u{e006}";
    pub const ButtonBOutline: &str = "\u{e007}";
    pub const ButtonHome: &str = "\u{e008}";
    pub const ButtonHomeOutline: &str = "\u{e009}";
    pub const ButtonL: &str = "\u{e00a}";
    pub const ButtonLOutline: &str = "\u{e00b}";
    pub const ButtonMinus: &str = "\u{e00c}";
    pub const ButtonMinusOutline: &str = "\u{e00d}";
    pub const ButtonPlus: &str = "\u{e00e}";
    pub const ButtonPlusOutline: &str = "\u{e00f}";
    pub const ButtonR: &str = "\u{e010}";
    pub const ButtonROutline: &str = "\u{e011}";
    pub const ButtonSl: &str = "\u{e012}";
    pub const ButtonSlOutline: &str = "\u{e013}";
    pub const ButtonSr: &str = "\u{e014}";
    pub const ButtonSrOutline: &str = "\u{e015}";
    pub const ButtonSync: &str = "\u{e016}";
    pub const ButtonSyncOutline: &str = "\u{e017}";
    pub const ButtonX: &str = "\u{e018}";
    pub const ButtonXOutline: &str = "\u{e019}";
    pub const ButtonY: &str = "\u{e01a}";
    pub const ButtonYOutline: &str = "\u{e01b}";
    pub const ButtonZl: &str = "\u{e01c}";
    pub const ButtonZlOutline: &str = "\u{e01d}";
    pub const ButtonZr: &str = "\u{e01e}";
    pub const ButtonZrOutline: &str = "\u{e01f}";
    pub const Buttons: &str = "\u{e020}";
    pub const ButtonsAll: &str = "\u{e021}";
    pub const ButtonsDown: &str = "\u{e022}";
    pub const ButtonsDownOutline: &str = "\u{e023}";
    pub const ButtonsHorizontal: &str = "\u{e024}";
    pub const ButtonsHorizontalOutline: &str = "\u{e025}";
    pub const ButtonsLeft: &str = "\u{e026}";
    pub const ButtonsLeftOutline: &str = "\u{e027}";
    pub const ButtonsNone: &str = "\u{e028}";
    pub const ButtonsRight: &str = "\u{e029}";
    pub const ButtonsRightOutline: &str = "\u{e02a}";
    pub const ButtonsUp: &str = "\u{e02b}";
    pub const ButtonsUpOutline: &str = "\u{e02c}";
    pub const ButtonsVertical: &str = "\u{e02d}";
    pub const ButtonsVerticalOutline: &str = "\u{e02e}";
    pub const Down: &str = "\u{e02f}";
    pub const DownOutline: &str = "\u{e030}";
    pub const Dpad: &str = "\u{e031}";
    pub const DpadAll: &str = "\u{e032}";
    pub const DpadDown: &str = "\u{e033}";
    pub const DpadDownOutline: &str = "\u{e034}";
    pub const DpadHorizontal: &str = "\u{e035}";
    pub const DpadHorizontalOutline: &str = "\u{e036}";
    pub const DpadLeft: &str = "\u{e037}";
    pub const DpadLeftOutline: &str = "\u{e038}";
    pub const DpadNone: &str = "\u{e039}";
    pub const DpadRight: &str = "\u{e03a}";
    pub const DpadRightOutline: &str = "\u{e03b}";
    pub const DpadUp: &str = "\u{e03c}";
    pub const DpadUpOutline: &str = "\u{e03d}";
    pub const DpadVertical: &str = "\u{e03e}";
    pub const DpadVerticalOutline: &str = "\u{e03f}";
    pub const Joycon: &str = "\u{e040}";
    pub const JoyconLeft: &str = "\u{e041}";
    pub const JoyconLeftDiagonal: &str = "\u{e042}";
    pub const JoyconLeftDiagonalOutline: &str = "\u{e043}";
    pub const JoyconLeftHorizontal: &str = "\u{e044}";
    pub const JoyconLeftHorizontalOutline: &str = "\u{e045}";
    pub const JoyconLeftOutline: &str = "\u{e046}";
    pub const JoyconLeftRotate: &str = "\u{e047}";
    pub const JoyconLeftRotateOutline: &str = "\u{e048}";
    pub const JoyconLeftVertical: &str = "\u{e049}";
    pub const JoyconLeftVerticalOutline: &str = "\u{e04a}";
    pub const JoyconOutline: &str = "\u{e04b}";
    pub const JoyconRight: &str = "\u{e04c}";
    pub const JoyconRightDiagonal: &str = "\u{e04d}";
    pub const JoyconRightDiagonalOutline: &str = "\u{e04e}";
    pub const JoyconRightHorizontal: &str = "\u{e04f}";
    pub const JoyconRightHorizontalOutline: &str = "\u{e050}";
    pub const JoyconRightOutline: &str = "\u{e051}";
    pub const JoyconRightRotate: &str = "\u{e052}";
    pub const JoyconRightRotateOutline: &str = "\u{e053}";
    pub const JoyconRightVertical: &str = "\u{e054}";
    pub const JoyconRightVerticalOutline: &str = "\u{e055}";
    pub const Left: &str = "\u{e056}";
    pub const LeftOutline: &str = "\u{e057}";
    pub const Right: &str = "\u{e058}";
    pub const RightOutline: &str = "\u{e059}";
    pub const StickL: &str = "\u{e05a}";
    pub const StickLDown: &str = "\u{e05b}";
    pub const StickLHorizontal: &str = "\u{e05c}";
    pub const StickLLeft: &str = "\u{e05d}";
    pub const StickLPress: &str = "\u{e05e}";
    pub const StickLRight: &str = "\u{e05f}";
    pub const StickLUp: &str = "\u{e060}";
    pub const StickLVertical: &str = "\u{e061}";
    pub const StickR: &str = "\u{e062}";
    pub const StickRDown: &str = "\u{e063}";
    pub const StickRHorizontal: &str = "\u{e064}";
    pub const StickRLeft: &str = "\u{e065}";
    pub const StickRPress: &str = "\u{e066}";
    pub const StickRRight: &str = "\u{e067}";
    pub const StickRUp: &str = "\u{e068}";
    pub const StickRVertical: &str = "\u{e069}";
    pub const StickSideL: &str = "\u{e06a}";
    pub const StickSideR: &str = "\u{e06b}";
    pub const StickTopL: &str = "\u{e06c}";
    pub const StickTopR: &str = "\u{e06d}";
    pub const Up: &str = "\u{e06e}";
    pub const UpOutline: &str = "\u{e06f}";
}

// ===== NINTENDO SWITCH 2 ICONS =====
pub mod switch_2 {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct Switch2;

    impl Switch2 {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::NintendoSwitch2)
        }
    }

    pub const ControllerSwitch2: &str = "\u{e000}";
    pub const ControllerSwitch2JoyconDown: &str = "\u{e001}";
    pub const ControllerSwitch2JoyconUp: &str = "\u{e002}";
    pub const ControllerSwitch2Pro: &str = "\u{e003}";
    pub const ButtonA: &str = "\u{e004}";
    pub const ButtonAOutline: &str = "\u{e005}";
    pub const ButtonB: &str = "\u{e006}";
    pub const ButtonBOutline: &str = "\u{e007}";
    pub const ButtonC: &str = "\u{e008}";
    pub const ButtonCOutline: &str = "\u{e009}";
    pub const ButtonGl: &str = "\u{e00a}";
    pub const ButtonGlOutline: &str = "\u{e00b}";
    pub const ButtonGr: &str = "\u{e00c}";
    pub const ButtonGrOutline: &str = "\u{e00d}";
    pub const ButtonHome: &str = "\u{e00e}";
    pub const ButtonHomeOutline: &str = "\u{e00f}";
    pub const ButtonL: &str = "\u{e010}";
    pub const ButtonLOutline: &str = "\u{e011}";
    pub const ButtonMinus: &str = "\u{e012}";
    pub const ButtonMinusOutline: &str = "\u{e013}";
    pub const ButtonPlus: &str = "\u{e014}";
    pub const ButtonPlusOutline: &str = "\u{e015}";
    pub const ButtonR: &str = "\u{e016}";
    pub const ButtonROutline: &str = "\u{e017}";
    pub const ButtonSl: &str = "\u{e018}";
    pub const ButtonSlOutline: &str = "\u{e019}";
    pub const ButtonSr: &str = "\u{e01a}";
    pub const ButtonSrOutline: &str = "\u{e01b}";
    pub const ButtonSync: &str = "\u{e01c}";
    pub const ButtonSyncOutline: &str = "\u{e01d}";
    pub const ButtonX: &str = "\u{e01e}";
    pub const ButtonXOutline: &str = "\u{e01f}";
    pub const ButtonY: &str = "\u{e020}";
    pub const ButtonYOutline: &str = "\u{e021}";
    pub const ButtonZl: &str = "\u{e022}";
    pub const ButtonZlOutline: &str = "\u{e023}";
    pub const ButtonZr: &str = "\u{e024}";
    pub const ButtonZrOutline: &str = "\u{e025}";
    pub const Buttons: &str = "\u{e026}";
    pub const ButtonsAll: &str = "\u{e027}";
    pub const ButtonsDown: &str = "\u{e028}";
    pub const ButtonsDownOutline: &str = "\u{e029}";
    pub const ButtonsHorizontal: &str = "\u{e02a}";
    pub const ButtonsHorizontalOutline: &str = "\u{e02b}";
    pub const ButtonsLeft: &str = "\u{e02c}";
    pub const ButtonsLeftOutline: &str = "\u{e02d}";
    pub const ButtonsNone: &str = "\u{e02e}";
    pub const ButtonsRight: &str = "\u{e02f}";
    pub const ButtonsRightOutline: &str = "\u{e030}";
    pub const ButtonsUp: &str = "\u{e031}";
    pub const ButtonsUpOutline: &str = "\u{e032}";
    pub const ButtonsVertical: &str = "\u{e033}";
    pub const ButtonsVerticalOutline: &str = "\u{e034}";
    pub const Down: &str = "\u{e035}";
    pub const DownOutline: &str = "\u{e036}";
    pub const Dpad: &str = "\u{e037}";
    pub const DpadAll: &str = "\u{e038}";
    pub const DpadDown: &str = "\u{e039}";
    pub const DpadDownOutline: &str = "\u{e03a}";
    pub const DpadHorizontal: &str = "\u{e03b}";
    pub const DpadHorizontalOutline: &str = "\u{e03c}";
    pub const DpadLeft: &str = "\u{e03d}";
    pub const DpadLeftOutline: &str = "\u{e03e}";
    pub const DpadNone: &str = "\u{e03f}";
    pub const DpadRight: &str = "\u{e040}";
    pub const DpadRightOutline: &str = "\u{e041}";
    pub const DpadUp: &str = "\u{e042}";
    pub const DpadUpOutline: &str = "\u{e043}";
    pub const DpadVertical: &str = "\u{e044}";
    pub const DpadVerticalOutline: &str = "\u{e045}";
    pub const Joycon: &str = "\u{e046}";
    pub const JoyconLeft: &str = "\u{e047}";
    pub const JoyconLeftDiagonal: &str = "\u{e048}";
    pub const JoyconLeftDiagonalOutline: &str = "\u{e049}";
    pub const JoyconLeftHorizontal: &str = "\u{e04a}";
    pub const JoyconLeftHorizontalOutline: &str = "\u{e04b}";
    pub const JoyconLeftMouse: &str = "\u{e04c}";
    pub const JoyconLeftMouseOutline: &str = "\u{e04d}";
    pub const JoyconLeftOutline: &str = "\u{e04e}";
    pub const JoyconLeftRotate: &str = "\u{e04f}";
    pub const JoyconLeftRotateOutline: &str = "\u{e050}";
    pub const JoyconLeftVertical: &str = "\u{e051}";
    pub const JoyconLeftVerticalOutline: &str = "\u{e052}";
    pub const JoyconOutline: &str = "\u{e053}";
    pub const JoyconRight: &str = "\u{e054}";
    pub const JoyconRightDiagonal: &str = "\u{e055}";
    pub const JoyconRightDiagonalOutline: &str = "\u{e056}";
    pub const JoyconRightHorizontal: &str = "\u{e057}";
    pub const JoyconRightHorizontalOutline: &str = "\u{e058}";
    pub const JoyconRightMouse: &str = "\u{e059}";
    pub const JoyconRightMouseOutline: &str = "\u{e05a}";
    pub const JoyconRightOutline: &str = "\u{e05b}";
    pub const JoyconRightRotate: &str = "\u{e05c}";
    pub const JoyconRightRotateOutline: &str = "\u{e05d}";
    pub const JoyconRightVertical: &str = "\u{e05e}";
    pub const JoyconRightVerticalOutline: &str = "\u{e05f}";
    pub const Left: &str = "\u{e060}";
    pub const LeftOutline: &str = "\u{e061}";
    pub const Right: &str = "\u{e062}";
    pub const RightOutline: &str = "\u{e063}";
    pub const StickL: &str = "\u{e064}";
    pub const StickLDown: &str = "\u{e065}";
    pub const StickLHorizontal: &str = "\u{e066}";
    pub const StickLLeft: &str = "\u{e067}";
    pub const StickLPress: &str = "\u{e068}";
    pub const StickLRight: &str = "\u{e069}";
    pub const StickLUp: &str = "\u{e06a}";
    pub const StickLVertical: &str = "\u{e06b}";
    pub const StickR: &str = "\u{e06c}";
    pub const StickRDown: &str = "\u{e06d}";
    pub const StickRHorizontal: &str = "\u{e06e}";
    pub const StickRLeft: &str = "\u{e06f}";
    pub const StickRPress: &str = "\u{e070}";
    pub const StickRRight: &str = "\u{e071}";
    pub const StickRUp: &str = "\u{e072}";
    pub const StickRVertical: &str = "\u{e073}";
    pub const StickSideL: &str = "\u{e074}";
    pub const StickSideR: &str = "\u{e075}";
    pub const StickTopL: &str = "\u{e076}";
    pub const StickTopR: &str = "\u{e077}";
    pub const Up: &str = "\u{e078}";
    pub const UpOutline: &str = "\u{e079}";
}

// ===== PLAYSTATION ICONS =====
pub mod playstation {
    use bevy::color::palettes::css::*;
    use bevy::color::Color;

    use crate::components::{text::Text, TextBuilder};
    use crate::theme::color::TextColor;
    use crate::theme::typography::FontFamily;

    pub struct Playstation;

    impl Playstation {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::PlayStation)
        }
    }

    pub const ControllerPlaystation1: &str = "\u{e000}";
    pub const ControllerPlaystation2: &str = "\u{e001}";
    pub const ControllerPlaystation3: &str = "\u{e002}";
    pub const ControllerPlaystation4: &str = "\u{e003}";
    pub const ControllerPlaystation5: &str = "\u{e004}";
    pub const Playstation3ButtonSelect: &str = "\u{e005}";
    pub const Playstation3ButtonSelectOutline: &str = "\u{e006}";
    pub const Playstation3ButtonStart: &str = "\u{e007}";
    pub const Playstation3ButtonStartOutline: &str = "\u{e008}";
    pub const Playstation4ButtonOptions: &str = "\u{e009}";
    pub const Playstation4ButtonOptionsOutline: &str = "\u{e00a}";
    pub const Playstation4ButtonShare: &str = "\u{e00b}";
    pub const Playstation4ButtonShareOutline: &str = "\u{e00c}";
    pub const Playstation4Touchpad: &str = "\u{e00d}";
    pub const Playstation4TouchpadOutline: &str = "\u{e00e}";
    pub const Playstation4TouchpadPress: &str = "\u{e00f}";
    pub const Playstation4TouchpadPressCenter: &str = "\u{e010}";
    pub const Playstation4TouchpadPressLeft: &str = "\u{e011}";
    pub const Playstation4TouchpadPressRight: &str = "\u{e012}";
    pub const Playstation4TouchpadSelected: &str = "\u{e013}";
    pub const Playstation4TouchpadSwipeDown: &str = "\u{e014}";
    pub const Playstation4TouchpadSwipeHorizontal: &str = "\u{e015}";
    pub const Playstation4TouchpadSwipeLeft: &str = "\u{e016}";
    pub const Playstation4TouchpadSwipeRight: &str = "\u{e017}";
    pub const Playstation4TouchpadSwipeUp: &str = "\u{e018}";
    pub const Playstation4TouchpadSwipeVertical: &str = "\u{e019}";
    pub const Playstation4TouchpadTouch: &str = "\u{e01a}";
    pub const Playstation4TouchpadTouchOutline: &str = "\u{e01b}";
    pub const Playstation5ButtonCreate: &str = "\u{e01c}";
    pub const Playstation5ButtonCreateAlternative: &str = "\u{e01d}";
    pub const Playstation5ButtonCreateAlternativeOutline: &str = "\u{e01e}";
    pub const Playstation5ButtonCreateOutline: &str = "\u{e01f}";
    pub const Playstation5ButtonMute: &str = "\u{e020}";
    pub const Playstation5ButtonMuteOutline: &str = "\u{e021}";
    pub const Playstation5ButtonOptions: &str = "\u{e022}";
    pub const Playstation5ButtonOptionsAlternative: &str = "\u{e023}";
    pub const Playstation5ButtonOptionsAlternativeOutline: &str = "\u{e024}";
    pub const Playstation5ButtonOptionsOutline: &str = "\u{e025}";
    pub const Playstation5EliteFnL: &str = "\u{e026}";
    pub const Playstation5EliteFnLOutline: &str = "\u{e027}";
    pub const Playstation5EliteFnR: &str = "\u{e028}";
    pub const Playstation5EliteFnROutline: &str = "\u{e029}";
    pub const Playstation5EliteLb: &str = "\u{e02a}";
    pub const Playstation5EliteLbOutline: &str = "\u{e02b}";
    pub const Playstation5EliteRb: &str = "\u{e02c}";
    pub const Playstation5EliteRbOutline: &str = "\u{e02d}";
    pub const Playstation5Touchpad: &str = "\u{e02e}";
    pub const Playstation5TouchpadOutline: &str = "\u{e02f}";
    pub const Playstation5TouchpadPress: &str = "\u{e030}";
    pub const Playstation5TouchpadPressCenter: &str = "\u{e031}";
    pub const Playstation5TouchpadPressLeft: &str = "\u{e032}";
    pub const Playstation5TouchpadPressRight: &str = "\u{e033}";
    pub const Playstation5TouchpadSelected: &str = "\u{e034}";
    pub const Playstation5TouchpadSwipeDown: &str = "\u{e035}";
    pub const Playstation5TouchpadSwipeHorizontal: &str = "\u{e036}";
    pub const Playstation5TouchpadSwipeLeft: &str = "\u{e037}";
    pub const Playstation5TouchpadSwipeRight: &str = "\u{e038}";
    pub const Playstation5TouchpadSwipeUp: &str = "\u{e039}";
    pub const Playstation5TouchpadSwipeVertical: &str = "\u{e03a}";
    pub const Playstation5TouchpadTouch: &str = "\u{e03b}";
    pub const Playstation5TouchpadTouchOutline: &str = "\u{e03c}";
    pub const ButtonAnalog: &str = "\u{e03d}";
    pub const ButtonAnalogOutline: &str = "\u{e03e}";
    pub const ButtonCircle: &str = "\u{e03f}";
    pub const ButtonCircleOutline: &str = "\u{e040}";
    pub const ButtonColorCircle: &str = "\u{e041}";
    pub const ButtonColorCircleOutline: &str = "\u{e042}";
    pub const ButtonColorCross: &str = "\u{e043}";
    pub const ButtonColorCrossOutline: &str = "\u{e044}";
    pub const ButtonColorSquare: &str = "\u{e045}";
    pub const ButtonColorSquareOutline: &str = "\u{e046}";
    pub const ButtonColorTriangle: &str = "\u{e047}";
    pub const ButtonColorTriangleOutline: &str = "\u{e048}";
    pub const ButtonCross: &str = "\u{e049}";
    pub const ButtonCrossOutline: &str = "\u{e04a}";
    pub const ButtonL3: &str = "\u{e04b}";
    pub const ButtonL3Outline: &str = "\u{e04c}";
    pub const ButtonR3: &str = "\u{e04d}";
    pub const ButtonR3Outline: &str = "\u{e04e}";
    pub const ButtonSquare: &str = "\u{e04f}";
    pub const ButtonSquareOutline: &str = "\u{e050}";
    pub const ButtonTriangle: &str = "\u{e051}";
    pub const ButtonTriangleOutline: &str = "\u{e052}";
    pub const Dpad: &str = "\u{e053}";
    pub const DpadAll: &str = "\u{e054}";
    pub const DpadDown: &str = "\u{e055}";
    pub const DpadDownOutline: &str = "\u{e056}";
    pub const DpadHorizontal: &str = "\u{e057}";
    pub const DpadHorizontalOutline: &str = "\u{e058}";
    pub const DpadLeft: &str = "\u{e059}";
    pub const DpadLeftOutline: &str = "\u{e05a}";
    pub const DpadNone: &str = "\u{e05b}";
    pub const DpadRight: &str = "\u{e05c}";
    pub const DpadRightOutline: &str = "\u{e05d}";
    pub const DpadUp: &str = "\u{e05e}";
    pub const DpadUpOutline: &str = "\u{e05f}";
    pub const DpadVertical: &str = "\u{e060}";
    pub const DpadVerticalOutline: &str = "\u{e061}";
    pub const StickL: &str = "\u{e062}";
    pub const StickLDown: &str = "\u{e063}";
    pub const StickLHorizontal: &str = "\u{e064}";
    pub const StickLLeft: &str = "\u{e065}";
    pub const StickLPress: &str = "\u{e066}";
    pub const StickLRight: &str = "\u{e067}";
    pub const StickLUp: &str = "\u{e068}";
    pub const StickLVertical: &str = "\u{e069}";
    pub const StickR: &str = "\u{e06a}";
    pub const StickRDown: &str = "\u{e06b}";
    pub const StickRHorizontal: &str = "\u{e06c}";
    pub const StickRLeft: &str = "\u{e06d}";
    pub const StickRPress: &str = "\u{e06e}";
    pub const StickRRight: &str = "\u{e06f}";
    pub const StickRUp: &str = "\u{e070}";
    pub const StickRVertical: &str = "\u{e071}";
    pub const StickSideL: &str = "\u{e072}";
    pub const StickSideR: &str = "\u{e073}";
    pub const StickTopL: &str = "\u{e074}";
    pub const StickTopR: &str = "\u{e075}";
    pub const TriggerL1: &str = "\u{e076}";
    pub const TriggerL1Alternative: &str = "\u{e077}";
    pub const TriggerL1AlternativeOutline: &str = "\u{e078}";
    pub const TriggerL1Outline: &str = "\u{e079}";
    pub const TriggerL2: &str = "\u{e07a}";
    pub const TriggerL2Alternative: &str = "\u{e07b}";
    pub const TriggerL2AlternativeOutline: &str = "\u{e07c}";
    pub const TriggerL2Outline: &str = "\u{e07d}";
    pub const TriggerR1: &str = "\u{e07e}";
    pub const TriggerR1Alternative: &str = "\u{e07f}";
    pub const TriggerR1AlternativeOutline: &str = "\u{e080}";
    pub const TriggerR1Outline: &str = "\u{e081}";
    pub const TriggerR2: &str = "\u{e082}";
    pub const TriggerR2Alternative: &str = "\u{e083}";
    pub const TriggerR2AlternativeOutline: &str = "\u{e084}";
    pub const TriggerR2Outline: &str = "\u{e085}";
}

// ===== XBOX ICONS =====
pub mod xbox {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct Xbox;

    impl Xbox {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::Xbox)
        }
    }

    pub const ControllerXbox360: &str = "\u{e000}";
    pub const ControllerXboxAdaptive: &str = "\u{e001}";
    pub const ControllerXboxone: &str = "\u{e002}";
    pub const ControllerXboxseries: &str = "\u{e003}";
    pub const ButtonA: &str = "\u{e004}";
    pub const ButtonAOutline: &str = "\u{e005}";
    pub const ButtonB: &str = "\u{e006}";
    pub const ButtonBOutline: &str = "\u{e007}";
    pub const ButtonBack: &str = "\u{e008}";
    pub const ButtonBackIcon: &str = "\u{e009}";
    pub const ButtonBackIconOutline: &str = "\u{e00a}";
    pub const ButtonBackOutline: &str = "\u{e00b}";
    pub const ButtonColorA: &str = "\u{e00c}";
    pub const ButtonColorAOutline: &str = "\u{e00d}";
    pub const ButtonColorB: &str = "\u{e00e}";
    pub const ButtonColorBOutline: &str = "\u{e00f}";
    pub const ButtonColorX: &str = "\u{e010}";
    pub const ButtonColorXOutline: &str = "\u{e011}";
    pub const ButtonColorY: &str = "\u{e012}";
    pub const ButtonColorYOutline: &str = "\u{e013}";
    pub const ButtonMenu: &str = "\u{e014}";
    pub const ButtonMenuOutline: &str = "\u{e015}";
    pub const ButtonShare: &str = "\u{e016}";
    pub const ButtonShareOutline: &str = "\u{e017}";
    pub const ButtonStart: &str = "\u{e018}";
    pub const ButtonStartIcon: &str = "\u{e019}";
    pub const ButtonStartIconOutline: &str = "\u{e01a}";
    pub const ButtonStartOutline: &str = "\u{e01b}";
    pub const ButtonView: &str = "\u{e01c}";
    pub const ButtonViewOutline: &str = "\u{e01d}";
    pub const ButtonX: &str = "\u{e01e}";
    pub const ButtonXOutline: &str = "\u{e01f}";
    pub const ButtonY: &str = "\u{e020}";
    pub const ButtonYOutline: &str = "\u{e021}";
    pub const Dpad: &str = "\u{e022}";
    pub const DpadAll: &str = "\u{e023}";
    pub const DpadDown: &str = "\u{e024}";
    pub const DpadDownOutline: &str = "\u{e025}";
    pub const DpadHorizontal: &str = "\u{e026}";
    pub const DpadHorizontalOutline: &str = "\u{e027}";
    pub const DpadLeft: &str = "\u{e028}";
    pub const DpadLeftOutline: &str = "\u{e029}";
    pub const DpadNone: &str = "\u{e02a}";
    pub const DpadRight: &str = "\u{e02b}";
    pub const DpadRightOutline: &str = "\u{e02c}";
    pub const DpadRound: &str = "\u{e02d}";
    pub const DpadRoundAll: &str = "\u{e02e}";
    pub const DpadRoundDown: &str = "\u{e02f}";
    pub const DpadRoundHorizontal: &str = "\u{e030}";
    pub const DpadRoundLeft: &str = "\u{e031}";
    pub const DpadRoundRight: &str = "\u{e032}";
    pub const DpadRoundUp: &str = "\u{e033}";
    pub const DpadRoundVertical: &str = "\u{e034}";
    pub const DpadUp: &str = "\u{e035}";
    pub const DpadUpOutline: &str = "\u{e036}";
    pub const DpadVertical: &str = "\u{e037}";
    pub const DpadVerticalOutline: &str = "\u{e038}";
    pub const ElitePaddleBottomLeft: &str = "\u{e039}";
    pub const ElitePaddleBottomLeftOutline: &str = "\u{e03a}";
    pub const ElitePaddleBottomRight: &str = "\u{e03b}";
    pub const ElitePaddleBottomRightOutline: &str = "\u{e03c}";
    pub const ElitePaddleTopLeft: &str = "\u{e03d}";
    pub const ElitePaddleTopLeftOutline: &str = "\u{e03e}";
    pub const ElitePaddleTopRight: &str = "\u{e03f}";
    pub const ElitePaddleTopRightOutline: &str = "\u{e040}";
    pub const Guide: &str = "\u{e041}";
    pub const GuideOutline: &str = "\u{e042}";
    pub const Lb: &str = "\u{e043}";
    pub const LbOutline: &str = "\u{e044}";
    pub const Ls: &str = "\u{e045}";
    pub const LsOutline: &str = "\u{e046}";
    pub const Lt: &str = "\u{e047}";
    pub const LtOutline: &str = "\u{e048}";
    pub const Rb: &str = "\u{e049}";
    pub const RbOutline: &str = "\u{e04a}";
    pub const Rs: &str = "\u{e04b}";
    pub const RsOutline: &str = "\u{e04c}";
    pub const Rt: &str = "\u{e04d}";
    pub const RtOutline: &str = "\u{e04e}";
    pub const StickL: &str = "\u{e04f}";
    pub const StickLDown: &str = "\u{e050}";
    pub const StickLHorizontal: &str = "\u{e051}";
    pub const StickLLeft: &str = "\u{e052}";
    pub const StickLPress: &str = "\u{e053}";
    pub const StickLRight: &str = "\u{e054}";
    pub const StickLUp: &str = "\u{e055}";
    pub const StickLVertical: &str = "\u{e056}";
    pub const StickR: &str = "\u{e057}";
    pub const StickRDown: &str = "\u{e058}";
    pub const StickRHorizontal: &str = "\u{e059}";
    pub const StickRLeft: &str = "\u{e05a}";
    pub const StickRPress: &str = "\u{e05b}";
    pub const StickRRight: &str = "\u{e05c}";
    pub const StickRUp: &str = "\u{e05d}";
    pub const StickRVertical: &str = "\u{e05e}";
    pub const StickSideL: &str = "\u{e05f}";
    pub const StickSideR: &str = "\u{e060}";
    pub const StickTopL: &str = "\u{e061}";
    pub const StickTopR: &str = "\u{e062}";
}

// ===== STEAM CONTROLLER ICONS =====
pub mod steam_controller {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct SteamController;

    impl SteamController {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::SteamController)
        }
    }
    pub const ControllerSteamController: &str = "\u{e000}";
    pub const ButtonA: &str = "\u{e001}";
    pub const ButtonAOutline: &str = "\u{e002}";
    pub const ButtonB: &str = "\u{e003}";
    pub const ButtonBOutline: &str = "\u{e004}";
    pub const ButtonBackIcon: &str = "\u{e005}";
    pub const ButtonBackIconOutline: &str = "\u{e006}";
    pub const ButtonColorA: &str = "\u{e007}";
    pub const ButtonColorAOutline: &str = "\u{e008}";
    pub const ButtonColorB: &str = "\u{e009}";
    pub const ButtonColorBOutline: &str = "\u{e00a}";
    pub const ButtonColorX: &str = "\u{e00b}";
    pub const ButtonColorXOutline: &str = "\u{e00c}";
    pub const ButtonColorY: &str = "\u{e00d}";
    pub const ButtonColorYOutline: &str = "\u{e00e}";
    pub const ButtonLp: &str = "\u{e00f}";
    pub const ButtonLpOutline: &str = "\u{e010}";
    pub const ButtonRp: &str = "\u{e011}";
    pub const ButtonRpOutline: &str = "\u{e012}";
    pub const ButtonStartIcon: &str = "\u{e013}";
    pub const ButtonStartIconOutline: &str = "\u{e014}";
    pub const ButtonX: &str = "\u{e015}";
    pub const ButtonXOutline: &str = "\u{e016}";
    pub const ButtonY: &str = "\u{e017}";
    pub const ButtonYOutline: &str = "\u{e018}";
    pub const Dpad: &str = "\u{e019}";
    pub const DpadAll: &str = "\u{e01a}";
    pub const DpadDown: &str = "\u{e01b}";
    pub const DpadDownOutline: &str = "\u{e01c}";
    pub const DpadHorizontal: &str = "\u{e01d}";
    pub const DpadHorizontalOutline: &str = "\u{e01e}";
    pub const DpadLeft: &str = "\u{e01f}";
    pub const DpadLeftOutline: &str = "\u{e020}";
    pub const DpadNone: &str = "\u{e021}";
    pub const DpadRight: &str = "\u{e022}";
    pub const DpadRightOutline: &str = "\u{e023}";
    pub const DpadUp: &str = "\u{e024}";
    pub const DpadUpOutline: &str = "\u{e025}";
    pub const DpadVertical: &str = "\u{e026}";
    pub const DpadVerticalOutline: &str = "\u{e027}";
    pub const Lb: &str = "\u{e028}";
    pub const LbOutline: &str = "\u{e029}";
    pub const Lg: &str = "\u{e02a}";
    pub const LgOutline: &str = "\u{e02b}";
    pub const Lt: &str = "\u{e02c}";
    pub const LtOutline: &str = "\u{e02d}";
    pub const Pad: &str = "\u{e02e}";
    pub const PadCenter: &str = "\u{e02f}";
    pub const PadDown: &str = "\u{e030}";
    pub const PadLeft: &str = "\u{e031}";
    pub const PadRight: &str = "\u{e032}";
    pub const PadUp: &str = "\u{e033}";
    pub const Rb: &str = "\u{e034}";
    pub const RbOutline: &str = "\u{e035}";
    pub const Rg: &str = "\u{e036}";
    pub const RgOutline: &str = "\u{e037}";
    pub const Rt: &str = "\u{e038}";
    pub const RtOutline: &str = "\u{e039}";
    pub const Stick: &str = "\u{e03a}";
    pub const StickDown: &str = "\u{e03b}";
    pub const StickHorizontal: &str = "\u{e03c}";
    pub const StickLPress: &str = "\u{e03d}";
    pub const StickLeft: &str = "\u{e03e}";
    pub const StickRight: &str = "\u{e03f}";
    pub const StickSideL: &str = "\u{e040}";
    pub const StickUp: &str = "\u{e041}";
    pub const StickVertical: &str = "\u{e042}";
}

// ===== STEAM DECK ICONS =====
pub mod steam_deck {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct SteamDeck;

    impl SteamDeck {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::SteamDeck)
        }
    }

    pub const ControllerSteamDeck: &str = "\u{e000}";
    pub const ButtonA: &str = "\u{e001}";
    pub const ButtonAOutline: &str = "\u{e002}";
    pub const ButtonB: &str = "\u{e003}";
    pub const ButtonBOutline: &str = "\u{e004}";
    pub const ButtonGuide: &str = "\u{e005}";
    pub const ButtonGuideOutline: &str = "\u{e006}";
    pub const ButtonL1: &str = "\u{e007}";
    pub const ButtonL1Outline: &str = "\u{e008}";
    pub const ButtonL2: &str = "\u{e009}";
    pub const ButtonL2Outline: &str = "\u{e00a}";
    pub const ButtonL4: &str = "\u{e00b}";
    pub const ButtonL4Outline: &str = "\u{e00c}";
    pub const ButtonL5: &str = "\u{e00d}";
    pub const ButtonL5Outline: &str = "\u{e00e}";
    pub const ButtonOptions: &str = "\u{e00f}";
    pub const ButtonOptionsOutline: &str = "\u{e010}";
    pub const ButtonQuickaccess: &str = "\u{e011}";
    pub const ButtonQuickaccessOutline: &str = "\u{e012}";
    pub const ButtonR1: &str = "\u{e013}";
    pub const ButtonR1Outline: &str = "\u{e014}";
    pub const ButtonR2: &str = "\u{e015}";
    pub const ButtonR2Outline: &str = "\u{e016}";
    pub const ButtonR4: &str = "\u{e017}";
    pub const ButtonR4Outline: &str = "\u{e018}";
    pub const ButtonR5: &str = "\u{e019}";
    pub const ButtonR5Outline: &str = "\u{e01a}";
    pub const ButtonView: &str = "\u{e01b}";
    pub const ButtonViewOutline: &str = "\u{e01c}";
    pub const ButtonX: &str = "\u{e01d}";
    pub const ButtonXOutline: &str = "\u{e01e}";
    pub const ButtonY: &str = "\u{e01f}";
    pub const ButtonYOutline: &str = "\u{e020}";
    pub const Dpad: &str = "\u{e021}";
    pub const DpadAll: &str = "\u{e022}";
    pub const DpadDown: &str = "\u{e023}";
    pub const DpadDownOutline: &str = "\u{e024}";
    pub const DpadHorizontal: &str = "\u{e025}";
    pub const DpadHorizontalOutline: &str = "\u{e026}";
    pub const DpadLeft: &str = "\u{e027}";
    pub const DpadLeftOutline: &str = "\u{e028}";
    pub const DpadNone: &str = "\u{e029}";
    pub const DpadRight: &str = "\u{e02a}";
    pub const DpadRightOutline: &str = "\u{e02b}";
    pub const DpadUp: &str = "\u{e02c}";
    pub const DpadUpOutline: &str = "\u{e02d}";
    pub const DpadVertical: &str = "\u{e02e}";
    pub const DpadVerticalOutline: &str = "\u{e02f}";
    pub const StickL: &str = "\u{e030}";
    pub const StickLDown: &str = "\u{e031}";
    pub const StickLHorizontal: &str = "\u{e032}";
    pub const StickLLeft: &str = "\u{e033}";
    pub const StickLPress: &str = "\u{e034}";
    pub const StickLRight: &str = "\u{e035}";
    pub const StickLUp: &str = "\u{e036}";
    pub const StickLVertical: &str = "\u{e037}";
    pub const StickR: &str = "\u{e038}";
    pub const StickRDown: &str = "\u{e039}";
    pub const StickRHorizontal: &str = "\u{e03a}";
    pub const StickRLeft: &str = "\u{e03b}";
    pub const StickRPress: &str = "\u{e03c}";
    pub const StickRRight: &str = "\u{e03d}";
    pub const StickRUp: &str = "\u{e03e}";
    pub const StickRVertical: &str = "\u{e03f}";
    pub const StickSideL: &str = "\u{e040}";
    pub const StickSideR: &str = "\u{e041}";
    pub const StickTopL: &str = "\u{e042}";
    pub const StickTopR: &str = "\u{e043}";
    pub const Trackpad: &str = "\u{e044}";
    pub const TrackpadAll: &str = "\u{e045}";
    pub const TrackpadAllOutline: &str = "\u{e046}";
    pub const TrackpadDown: &str = "\u{e047}";
    pub const TrackpadDownOutline: &str = "\u{e048}";
    pub const TrackpadHorizontal: &str = "\u{e049}";
    pub const TrackpadHorizontalOutline: &str = "\u{e04a}";
    pub const TrackpadL: &str = "\u{e04b}";
    pub const TrackpadLAll: &str = "\u{e04c}";
    pub const TrackpadLAllOutline: &str = "\u{e04d}";
    pub const TrackpadLDown: &str = "\u{e04e}";
    pub const TrackpadLDownOutline: &str = "\u{e04f}";
    pub const TrackpadLHorizontal: &str = "\u{e050}";
    pub const TrackpadLHorizontalOutline: &str = "\u{e051}";
    pub const TrackpadLLeft: &str = "\u{e052}";
    pub const TrackpadLLeftOutline: &str = "\u{e053}";
    pub const TrackpadLOutline: &str = "\u{e054}";
    pub const TrackpadLRight: &str = "\u{e055}";
    pub const TrackpadLRightOutline: &str = "\u{e056}";
    pub const TrackpadLUp: &str = "\u{e057}";
    pub const TrackpadLUpOutline: &str = "\u{e058}";
    pub const TrackpadLVertical: &str = "\u{e059}";
    pub const TrackpadLVerticalOutline: &str = "\u{e05a}";
    pub const TrackpadLeft: &str = "\u{e05b}";
    pub const TrackpadLeftOutline: &str = "\u{e05c}";
    pub const TrackpadOutline: &str = "\u{e05d}";
    pub const TrackpadR: &str = "\u{e05e}";
    pub const TrackpadRAll: &str = "\u{e05f}";
    pub const TrackpadRAllOutline: &str = "\u{e060}";
    pub const TrackpadRDown: &str = "\u{e061}";
    pub const TrackpadRDownOutline: &str = "\u{e062}";
    pub const TrackpadRHorizontal: &str = "\u{e063}";
    pub const TrackpadRHorizontalOutline: &str = "\u{e064}";
    pub const TrackpadRLeft: &str = "\u{e065}";
    pub const TrackpadRLeftOutline: &str = "\u{e066}";
    pub const TrackpadROutline: &str = "\u{e067}";
    pub const TrackpadRRight: &str = "\u{e068}";
    pub const TrackpadRRightOutline: &str = "\u{e069}";
    pub const TrackpadRUp: &str = "\u{e06a}";
    pub const TrackpadRUpOutline: &str = "\u{e06b}";
    pub const TrackpadRVertical: &str = "\u{e06c}";
    pub const TrackpadRVerticalOutline: &str = "\u{e06d}";
    pub const TrackpadRight: &str = "\u{e06e}";
    pub const TrackpadRightOutline: &str = "\u{e06f}";
    pub const TrackpadUp: &str = "\u{e070}";
    pub const TrackpadUpOutline: &str = "\u{e071}";
    pub const TrackpadVertical: &str = "\u{e072}";
    pub const TrackpadVerticalOutline: &str = "\u{e073}";
}

// ===== GAMECUBE ICONS =====
pub mod gamecube {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct Gamecube;

    impl Gamecube {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::Gamecube)
        }
    }

    pub const ButtonA: &str = "\u{e000}";
    pub const ButtonAOutline: &str = "\u{e001}";
    pub const ButtonB: &str = "\u{e002}";
    pub const ButtonBOutline: &str = "\u{e003}";
    pub const ButtonColorA: &str = "\u{e004}";
    pub const ButtonColorAOutline: &str = "\u{e005}";
    pub const ButtonColorB: &str = "\u{e006}";
    pub const ButtonColorBOutline: &str = "\u{e007}";
    pub const ButtonStart: &str = "\u{e008}";
    pub const ButtonStartOutline: &str = "\u{e009}";
    pub const ButtonX: &str = "\u{e00a}";
    pub const ButtonXOutline: &str = "\u{e00b}";
    pub const ButtonXTilted: &str = "\u{e00c}";
    pub const ButtonXTiltedOutline: &str = "\u{e00d}";
    pub const ButtonY: &str = "\u{e00e}";
    pub const ButtonYOutline: &str = "\u{e00f}";
    pub const ButtonYTilted: &str = "\u{e010}";
    pub const ButtonYTiltedOutline: &str = "\u{e011}";
    pub const ButtonZ: &str = "\u{e012}";
    pub const ButtonZOutline: &str = "\u{e013}";
    pub const ControllerGamecube: &str = "\u{e014}";
    pub const ControllerGamecubeWavebird: &str = "\u{e015}";
    pub const Dpad: &str = "\u{e016}";
    pub const DpadAll: &str = "\u{e017}";
    pub const DpadDown: &str = "\u{e018}";
    pub const DpadDownOutline: &str = "\u{e019}";
    pub const DpadHorizontal: &str = "\u{e01a}";
    pub const DpadHorizontalOutline: &str = "\u{e01b}";
    pub const DpadLeft: &str = "\u{e01c}";
    pub const DpadLeftOutline: &str = "\u{e01d}";
    pub const DpadNone: &str = "\u{e01e}";
    pub const DpadRight: &str = "\u{e01f}";
    pub const DpadRightOutline: &str = "\u{e020}";
    pub const DpadUp: &str = "\u{e021}";
    pub const DpadUpOutline: &str = "\u{e022}";
    pub const DpadVertical: &str = "\u{e023}";
    pub const DpadVerticalOutline: &str = "\u{e024}";
    pub const Stick: &str = "\u{e025}";
    pub const StickC: &str = "\u{e026}";
    pub const StickCColorDown: &str = "\u{e027}";
    pub const StickCColorHorizontal: &str = "\u{e028}";
    pub const StickCColorLeft: &str = "\u{e029}";
    pub const StickCColorRight: &str = "\u{e02a}";
    pub const StickCColorTop: &str = "\u{e02b}";
    pub const StickCColorUp: &str = "\u{e02c}";
    pub const StickCColorVertical: &str = "\u{e02d}";
    pub const StickCDown: &str = "\u{e02e}";
    pub const StickCHorizontal: &str = "\u{e02f}";
    pub const StickCLeft: &str = "\u{e030}";
    pub const StickCRight: &str = "\u{e031}";
    pub const StickCTop: &str = "\u{e032}";
    pub const StickCUp: &str = "\u{e033}";
    pub const StickCVertical: &str = "\u{e034}";
    pub const StickColorC: &str = "\u{e035}";
    pub const StickDown: &str = "\u{e036}";
    pub const StickGrip: &str = "\u{e037}";
    pub const StickGripDown: &str = "\u{e038}";
    pub const StickGripHorizontal: &str = "\u{e039}";
    pub const StickGripLeft: &str = "\u{e03a}";
    pub const StickGripRight: &str = "\u{e03b}";
    pub const StickGripTop: &str = "\u{e03c}";
    pub const StickGripUp: &str = "\u{e03d}";
    pub const StickGripVertical: &str = "\u{e03e}";
    pub const StickHorizontal: &str = "\u{e03f}";
    pub const StickLeft: &str = "\u{e040}";
    pub const StickRight: &str = "\u{e041}";
    pub const StickTop: &str = "\u{e042}";
    pub const StickUp: &str = "\u{e043}";
    pub const StickVertical: &str = "\u{e044}";
    pub const TriggerL: &str = "\u{e045}";
    pub const TriggerLOutline: &str = "\u{e046}";
    pub const TriggerR: &str = "\u{e047}";
    pub const TriggerROutline: &str = "\u{e048}";
}

// ===== WII ICONS =====
pub mod wii {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct Wii;

    impl Wii {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::Wii)
        }
    }

    pub const ControllerClassicWII: &str = "\u{e000}";
    pub const ControllerClassicWIIPro: &str = "\u{e001}";
    pub const Button1: &str = "\u{e002}";
    pub const Button1Outline: &str = "\u{e003}";
    pub const Button2: &str = "\u{e004}";
    pub const Button2Outline: &str = "\u{e005}";
    pub const ButtonA: &str = "\u{e006}";
    pub const ButtonAOutline: &str = "\u{e007}";
    pub const ButtonB: &str = "\u{e008}";
    pub const ButtonBOutline: &str = "\u{e009}";
    pub const ButtonC: &str = "\u{e00a}";
    pub const ButtonCOutline: &str = "\u{e00b}";
    pub const ButtonHome: &str = "\u{e00c}";
    pub const ButtonHomeOutline: &str = "\u{e00d}";
    pub const ButtonL: &str = "\u{e00e}";
    pub const ButtonLOutline: &str = "\u{e00f}";
    pub const ButtonMinus: &str = "\u{e010}";
    pub const ButtonMinusOutline: &str = "\u{e011}";
    pub const ButtonPlus: &str = "\u{e012}";
    pub const ButtonPlusOutline: &str = "\u{e013}";
    pub const ButtonPower: &str = "\u{e014}";
    pub const ButtonPowerOutline: &str = "\u{e015}";
    pub const ButtonR: &str = "\u{e016}";
    pub const ButtonROutline: &str = "\u{e017}";
    pub const ButtonX: &str = "\u{e018}";
    pub const ButtonXOutline: &str = "\u{e019}";
    pub const ButtonY: &str = "\u{e01a}";
    pub const ButtonYOutline: &str = "\u{e01b}";
    pub const ButtonZ: &str = "\u{e01c}";
    pub const ButtonZOutline: &str = "\u{e01d}";
    pub const ButtonZl: &str = "\u{e01e}";
    pub const ButtonZlOutline: &str = "\u{e01f}";
    pub const ButtonZr: &str = "\u{e020}";
    pub const ButtonZrOutline: &str = "\u{e021}";
    pub const ControllerWII: &str = "\u{e022}";
    pub const ControllerDiagonal: &str = "\u{e023}";
    pub const ControllerDiagonalOutline: &str = "\u{e024}";
    pub const ControllerHorizontal: &str = "\u{e025}";
    pub const ControllerHorizontalOutline: &str = "\u{e026}";
    pub const ControllerNunchuk: &str = "\u{e027}";
    pub const ControllerNunchukWire: &str = "\u{e028}";
    pub const ControllerOutline: &str = "\u{e029}";
    pub const ControllerRotate: &str = "\u{e02a}";
    pub const ControllerRotateOutline: &str = "\u{e02b}";
    pub const ControllerVertical: &str = "\u{e02c}";
    pub const ControllerVerticalOutline: &str = "\u{e02d}";
    pub const Dpad: &str = "\u{e02e}";
    pub const DpadAll: &str = "\u{e02f}";
    pub const DpadDown: &str = "\u{e030}";
    pub const DpadDownOutline: &str = "\u{e031}";
    pub const DpadHorizontal: &str = "\u{e032}";
    pub const DpadHorizontalOutline: &str = "\u{e033}";
    pub const DpadLeft: &str = "\u{e034}";
    pub const DpadLeftOutline: &str = "\u{e035}";
    pub const DpadNone: &str = "\u{e036}";
    pub const DpadRight: &str = "\u{e037}";
    pub const DpadRightOutline: &str = "\u{e038}";
    pub const DpadUp: &str = "\u{e039}";
    pub const DpadUpOutline: &str = "\u{e03a}";
    pub const DpadVertical: &str = "\u{e03b}";
    pub const DpadVerticalOutline: &str = "\u{e03c}";
    pub const Stick: &str = "\u{e03d}";
    pub const StickDown: &str = "\u{e03e}";
    pub const StickHorizontal: &str = "\u{e03f}";
    pub const StickL: &str = "\u{e040}";
    pub const StickLDown: &str = "\u{e041}";
    pub const StickLHorizontal: &str = "\u{e042}";
    pub const StickLLeft: &str = "\u{e043}";
    pub const StickLRight: &str = "\u{e044}";
    pub const StickLUp: &str = "\u{e045}";
    pub const StickLVertical: &str = "\u{e046}";
    pub const StickLeft: &str = "\u{e047}";
    pub const StickR: &str = "\u{e048}";
    pub const StickRDown: &str = "\u{e049}";
    pub const StickRHorizontal: &str = "\u{e04a}";
    pub const StickRLeft: &str = "\u{e04b}";
    pub const StickRRight: &str = "\u{e04c}";
    pub const StickRUp: &str = "\u{e04d}";
    pub const StickRVertical: &str = "\u{e04e}";
    pub const StickRight: &str = "\u{e04f}";
    pub const StickTopL: &str = "\u{e050}";
    pub const StickTopR: &str = "\u{e051}";
    pub const StickUp: &str = "\u{e052}";
    pub const StickVertical: &str = "\u{e053}";
}

// ===== WIIU ICONS =====
pub mod wiiu {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct WiiU;

    impl WiiU {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::WiiU)
        }
    }

    pub const ControllerWIIUPro: &str = "\u{e000}";
    pub const Button1: &str = "\u{e001}";
    pub const Button1Outline: &str = "\u{e002}";
    pub const Button2: &str = "\u{e003}";
    pub const Button2Outline: &str = "\u{e004}";
    pub const ButtonA: &str = "\u{e005}";
    pub const ButtonAOutline: &str = "\u{e006}";
    pub const ButtonB: &str = "\u{e007}";
    pub const ButtonBOutline: &str = "\u{e008}";
    pub const ButtonHome: &str = "\u{e009}";
    pub const ButtonHomeOutline: &str = "\u{e00a}";
    pub const ButtonL: &str = "\u{e00b}";
    pub const ButtonLOutline: &str = "\u{e00c}";
    pub const ButtonMinus: &str = "\u{e00d}";
    pub const ButtonMinusOutline: &str = "\u{e00e}";
    pub const ButtonPlus: &str = "\u{e00f}";
    pub const ButtonPlusOutline: &str = "\u{e010}";
    pub const ButtonPower: &str = "\u{e011}";
    pub const ButtonPowerOutline: &str = "\u{e012}";
    pub const ButtonR: &str = "\u{e013}";
    pub const ButtonROutline: &str = "\u{e014}";
    pub const ButtonTv: &str = "\u{e015}";
    pub const ButtonTvOutline: &str = "\u{e016}";
    pub const ButtonX: &str = "\u{e017}";
    pub const ButtonXOutline: &str = "\u{e018}";
    pub const ButtonY: &str = "\u{e019}";
    pub const ButtonYOutline: &str = "\u{e01a}";
    pub const ButtonZl: &str = "\u{e01b}";
    pub const ButtonZlOutline: &str = "\u{e01c}";
    pub const ButtonZr: &str = "\u{e01d}";
    pub const ButtonZrOutline: &str = "\u{e01e}";
    pub const ControllerWIIU: &str = "\u{e01f}";
    pub const ControllerDown: &str = "\u{e020}";
    pub const ControllerDownOutline: &str = "\u{e021}";
    pub const ControllerOutline: &str = "\u{e022}";
    pub const ControllerUp: &str = "\u{e023}";
    pub const ControllerUpOutlnie: &str = "\u{e024}";
    pub const Dpad: &str = "\u{e025}";
    pub const DpadAll: &str = "\u{e026}";
    pub const DpadDown: &str = "\u{e027}";
    pub const DpadDownOutline: &str = "\u{e028}";
    pub const DpadHorizontal: &str = "\u{e029}";
    pub const DpadHorizontalOutline: &str = "\u{e02a}";
    pub const DpadLeft: &str = "\u{e02b}";
    pub const DpadLeftOutline: &str = "\u{e02c}";
    pub const DpadNone: &str = "\u{e02d}";
    pub const DpadRight: &str = "\u{e02e}";
    pub const DpadRightOutline: &str = "\u{e02f}";
    pub const DpadUp: &str = "\u{e030}";
    pub const DpadUpOutline: &str = "\u{e031}";
    pub const DpadVertical: &str = "\u{e032}";
    pub const DpadVerticalOutline: &str = "\u{e033}";
    pub const StickL: &str = "\u{e034}";
    pub const StickLDown: &str = "\u{e035}";
    pub const StickLHorizontal: &str = "\u{e036}";
    pub const StickLLeft: &str = "\u{e037}";
    pub const StickLRight: &str = "\u{e038}";
    pub const StickLUp: &str = "\u{e039}";
    pub const StickLVertical: &str = "\u{e03a}";
    pub const StickR: &str = "\u{e03b}";
    pub const StickRDown: &str = "\u{e03c}";
    pub const StickRHorizontal: &str = "\u{e03d}";
    pub const StickRLeft: &str = "\u{e03e}";
    pub const StickRRight: &str = "\u{e03f}";
    pub const StickRUp: &str = "\u{e040}";
    pub const StickRVertical: &str = "\u{e041}";
    pub const StickTopL: &str = "\u{e042}";
    pub const StickTopR: &str = "\u{e043}";
}

// ===== TOUCH ICONS =====
pub mod touch {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct Touch;

    impl Touch {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content).family(FontFamily::Touch)
        }
    }

    pub const FingerOne: &str = "\u{e000}";
    pub const FingerTwo: &str = "\u{e001}";
    pub const HandClosed: &str = "\u{e002}";
    pub const HandOpen: &str = "\u{e003}";
    pub const RotateLeft: &str = "\u{e004}";
    pub const RotateRight: &str = "\u{e005}";
    pub const SwipeDown: &str = "\u{e006}";
    pub const SwipeHorizontal: &str = "\u{e007}";
    pub const SwipeLeft: &str = "\u{e008}";
    pub const SwipeMove: &str = "\u{e009}";
    pub const SwipeRight: &str = "\u{e00a}";
    pub const SwipeTwoDown: &str = "\u{e00b}";
    pub const SwipeTwoHorizontal: &str = "\u{e00c}";
    pub const SwipeTwoLeft: &str = "\u{e00d}";
    pub const SwipeTwoMove: &str = "\u{e00e}";
    pub const SwipeTwoRight: &str = "\u{e00f}";
    pub const SwipeTwoUp: &str = "\u{e010}";
    pub const SwipeTwoVertical: &str = "\u{e011}";
    pub const SwipeUp: &str = "\u{e012}";
    pub const SwipeVertical: &str = "\u{e013}";
    pub const Tap: &str = "\u{e014}";
    pub const TapDouble: &str = "\u{e015}";
    pub const TapHold: &str = "\u{e016}";
    pub const Two: &str = "\u{e017}";
    pub const TwoDouble: &str = "\u{e018}";
    pub const TwoHold: &str = "\u{e019}";
    pub const ZoomIn: &str = "\u{e01a}";
    pub const ZoomOut: &str = "\u{e01b}";
}

// ===== INDICATORS/FLAIRS =====
pub mod indicators {
    use crate::components::{text::Text, TextBuilder};
    use crate::theme::typography::FontFamily;

    pub struct Indicators;

    impl Indicators {
        pub fn new(content: impl Into<String>) -> TextBuilder {
            Text::new(content)
                .family(FontFamily::Indicators)
                .size(crate::theme::typography::TextSize::X2l)
        }
    }

    pub const ControllerBatteryEmpty: &str = "\u{e000}";
    pub const ControllerBatteryFull: &str = "\u{e001}";
    pub const ControllerBatteryHalf: &str = "\u{e002}";
    pub const ControllerConnectingA: &str = "\u{e003}";
    pub const ControllerConnectingB: &str = "\u{e004}";
    pub const ControllerDisconnected: &str = "\u{e005}";
    pub const ControllerGeneric: &str = "\u{e006}";
    pub const ControllerIconBatteryEmpty: &str = "\u{e007}";
    pub const ControllerIconBatteryFull: &str = "\u{e008}";
    pub const ControllerIconBatteryHalf: &str = "\u{e009}";
    pub const ControllerIconConnectingA: &str = "\u{e00a}";
    pub const ControllerIconConnectingB: &str = "\u{e00b}";
    pub const ControllerIconDisconnected: &str = "\u{e00c}";
    pub const FlairArrow0: &str = "\u{e00d}";
    pub const FlairArrow1: &str = "\u{e00e}";
    pub const FlairArrow2: &str = "\u{e00f}";
    pub const FlairArrow3: &str = "\u{e010}";
    pub const FlairArrowBackforth: &str = "\u{e011}";
    pub const FlairArrowCenter0: &str = "\u{e012}";
    pub const FlairArrowCenter1: &str = "\u{e013}";
    pub const FlairArrowCenter2: &str = "\u{e014}";
    pub const FlairArrowCenter3: &str = "\u{e015}";
    pub const FlairArrowLong: &str = "\u{e016}";
    pub const FlairArrowShort: &str = "\u{e017}";
    pub const FlairArrowZ: &str = "\u{e018}";
    pub const FlairArrowsAll: &str = "\u{e019}";
    pub const FlairArrowsDiagonalAll: &str = "\u{e01a}";
    pub const FlairArrowsDiagonalLeft: &str = "\u{e01b}";
    pub const FlairArrowsDiagonalRight: &str = "\u{e01c}";
    pub const FlairArrowsDown: &str = "\u{e01d}";
    pub const FlairArrowsHorizontal: &str = "\u{e01e}";
    pub const FlairArrowsLeft: &str = "\u{e01f}";
    pub const FlairArrowsRight: &str = "\u{e020}";
    pub const FlairArrowsUp: &str = "\u{e021}";
    pub const FlairArrowsVertical: &str = "\u{e022}";
    pub const FlairCircle0: &str = "\u{e023}";
    pub const FlairCircle1: &str = "\u{e024}";
    pub const FlairCircle2: &str = "\u{e025}";
    pub const FlairCircle3: &str = "\u{e026}";
    pub const FlairCircle4: &str = "\u{e027}";
    pub const FlairCircle5: &str = "\u{e028}";
    pub const FlairCircle6: &str = "\u{e029}";
    pub const FlairCircle7: &str = "\u{e02a}";
    pub const FlairCircle8: &str = "\u{e02b}";
    pub const FlairCircleRed0: &str = "\u{e02c}";
    pub const FlairCircleRed1: &str = "\u{e02d}";
    pub const FlairCircleRed2: &str = "\u{e02e}";
    pub const FlairCircleRed3: &str = "\u{e02f}";
    pub const FlairCircleRed4: &str = "\u{e030}";
    pub const FlairCircleRed5: &str = "\u{e031}";
    pub const FlairCircleRed6: &str = "\u{e032}";
    pub const FlairCircleRed7: &str = "\u{e033}";
    pub const FlairCircleRed8: &str = "\u{e034}";
    pub const FlairCircleTargetA: &str = "\u{e035}";
    pub const FlairCircleTargetB: &str = "\u{e036}";
    pub const FlairCross: &str = "\u{e037}";
    pub const FlairDisabled: &str = "\u{e038}";
    pub const FlairDisabledCross: &str = "\u{e039}";
    pub const FlairDisabledCrossOutline: &str = "\u{e03a}";
    pub const FlairDisabledLine: &str = "\u{e03b}";
    pub const FlairDisabledLineOutline: &str = "\u{e03c}";
    pub const FlairDisabledOutline: &str = "\u{e03d}";
    pub const FlairNumber0: &str = "\u{e03e}";
    pub const FlairNumber0Outline: &str = "\u{e03f}";
    pub const FlairNumber1: &str = "\u{e040}";
    pub const FlairNumber1Outline: &str = "\u{e041}";
    pub const FlairNumber2: &str = "\u{e042}";
    pub const FlairNumber2Outline: &str = "\u{e043}";
    pub const FlairNumber3: &str = "\u{e044}";
    pub const FlairNumber3Outline: &str = "\u{e045}";
    pub const FlairNumber4: &str = "\u{e046}";
    pub const FlairNumber4Outline: &str = "\u{e047}";
    pub const FlairNumber5: &str = "\u{e048}";
    pub const FlairNumber5Outline: &str = "\u{e049}";
    pub const FlairNumber6: &str = "\u{e04a}";
    pub const FlairNumber6Outline: &str = "\u{e04b}";
    pub const FlairNumber7: &str = "\u{e04c}";
    pub const FlairNumber7Outline: &str = "\u{e04d}";
    pub const FlairNumber8: &str = "\u{e04e}";
    pub const FlairNumber8Outline: &str = "\u{e04f}";
    pub const FlairNumber9: &str = "\u{e050}";
    pub const FlairNumber9Outline: &str = "\u{e051}";
    pub const FlairPlus: &str = "\u{e052}";
    pub const FlairSmallCheck: &str = "\u{e053}";
    pub const FlairSmallCheckOutline: &str = "\u{e054}";
    pub const FlairSmallCross: &str = "\u{e055}";
    pub const FlairSmallCrossOutline: &str = "\u{e056}";
    pub const FlairSmallDisabled: &str = "\u{e057}";
    pub const FlairSmallDisabledOutline: &str = "\u{e058}";
    pub const FlairSmallInfo: &str = "\u{e059}";
    pub const FlairSmallInfoOutline: &str = "\u{e05a}";
    pub const FlairSmallRotate: &str = "\u{e05b}";
    pub const FlairSmallRotateOutline: &str = "\u{e05c}";
}
