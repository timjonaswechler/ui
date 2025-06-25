#![allow(non_snake_case)]
use super::macros::IconSize;
use crate::define_icon_category;
use bevy::prelude::*;

// ===== GENERIC CONTROLLER ICONS =====
define_icon_category! {
    Generic {
        path: "ui/icons/controllers/generic",
        atlas: (20, 2),
        icons: [

            GenericButton = 0,
            GenericButtonCircle = 1,
            GenericButtonCircleFill = 2,
            GenericButtonCircleOutline = 3,
            GenericButtonFinger = 4,
            GenericButtonFingerPressed = 5,
            GenericButtonPressed = 6,
            GenericButtonSquare = 7,
            GenericButtonSquareFill = 8,
            GenericButtonSquareOutline = 9,
            GenericButtonTriggerA = 10,
            GenericButtonTriggerAFill = 11,
            GenericButtonTriggerAOutline = 12,
            GenericButtonTriggerB = 13,
            GenericButtonTriggerBFill = 14,
            GenericButtonTriggerBOutline = 15,
            GenericButtonTriggerC = 16,
            GenericButtonTriggerCFill = 17,
            GenericButtonTriggerCOutline = 18,
            GenericJoystick = 19,
            GenericJoystickHorizontal = 20,
            GenericJoystickLeft = 21,
            GenericJoystickRed = 22,
            GenericJoystickRedHorizontal = 23,
            GenericJoystickRedLeft = 24,
            GenericJoystickRedRight = 25,
            GenericJoystickRight = 26,
            GenericStick = 27,
            GenericStickDown = 28,
            GenericStickHorizontal = 29,
            GenericStickLeft = 30,
            GenericStickPress = 31,
            GenericStickRight = 32,
            GenericStickSide = 33,
            GenericStickUp = 34,
            GenericStickVertical = 35,
        ]
    }
}

// ===== KEYBOARD & MOUSE ICONS =====
define_icon_category! {
    KeyboardMouse {
        path: "ui/icons/controllers/keyboard_mouse",
        atlas: (20, 13),
        icons: [
            Keyboard = 0,
            KeyboardA = 1,
            KeyboardB = 2,
            KeyboardC = 3,
            KeyboardD = 4,
            KeyboardE = 5,
            KeyboardF = 6,
            KeyboardG = 7,
            KeyboardH = 8,
            KeyboardI = 9,
            KeyboardJ = 10,
            KeyboardK = 11,
            KeyboardL = 12,
            KeyboardM = 13,
            KeyboardN = 14,
            KeyboardO = 15,
            KeyboardP = 16,
            KeyboardQ = 17,
            KeyboardR = 18,
            KeyboardS = 19,
            KeyboardT = 20,
            KeyboardU = 21,
            KeyboardV = 22,
            KeyboardW = 23,
            KeyboardX = 24,
            KeyboardY = 25,
            KeyboardZ = 26,
            Keyboard0 = 27,
            Keyboard1 = 28,
            Keyboard2 = 29,
            Keyboard3 = 30,
            Keyboard4 = 31,
            Keyboard5 = 32,
            Keyboard6 = 33,
            Keyboard7 = 34,
            Keyboard8 = 35,
            Keyboard9 = 36,
            KeyboardF1 = 37,
            KeyboardF2 = 38,
            KeyboardF3 = 39,
            KeyboardF4 = 40,
            KeyboardF5 = 41,
            KeyboardF6 = 42,
            KeyboardF7 = 43,
            KeyboardF8 = 44,
            KeyboardF9 = 45,
            KeyboardF10 = 46,
            KeyboardF11 = 47,
            KeyboardF12 = 48,
            KeyboardSpace = 49,
            KeyboardEnter = 50,
            KeyboardShift = 51,
            KeyboardCtrl = 52,
            KeyboardAlt = 53,
            KeyboardTab = 54,
            KeyboardBackspace = 55,
            KeyboardEscape = 56,
            KeyboardArrowUp = 57,
            KeyboardArrowDown = 58,
            KeyboardArrowLeft = 59,
            KeyboardArrowRight = 60,
            Mouse = 70,
            MouseLeft = 71,
            MouseRight = 72,
            MouseScroll = 73,
            MouseScrollUp = 74,
            MouseScrollDown = 75,
            MouseMove = 76,
        ]
    }
}

// ===== NINTENDO SWITCH ICONS =====
define_icon_category! {
    NintendoSwitch {
        path: "ui/icons/controllers/nintendo_switch",
        atlas: (20, 6),
        icons: [
            SwitchButtonA = 0,
            SwitchButtonB = 1,
            SwitchButtonX = 2,
            SwitchButtonY = 3,
            SwitchButtonL = 4,
            SwitchButtonR = 5,
            SwitchButtonZL = 6,
            SwitchButtonZR = 7,
            SwitchButtonPlus = 8,
            SwitchButtonMinus = 9,
            SwitchButtonHome = 10,
            SwitchButtonCapture = 11,
            SwitchDpadUp = 12,
            SwitchDpadDown = 13,
            SwitchDpadLeft = 14,
            SwitchDpadRight = 15,
            SwitchDpad = 16,
            SwitchStickL = 17,
            SwitchStickR = 18,
            SwitchJoycon = 19,
        ]
    }
}

// ===== PLAYSTATION ICONS =====
define_icon_category! {
    PlayStation {
        path: "ui/icons/controllers/playstation",
        atlas: (20, 7),
        icons: [
            PlayStationCross = 0,
            PlayStationCircle = 1,
            PlayStationSquare = 2,
            PlayStationTriangle = 3,
            PlayStationL1 = 4,
            PlayStationL2 = 5,
            PlayStationR1 = 6,
            PlayStationR2 = 7,
            PlayStationL3 = 8,
            PlayStationR3 = 9,
            PlayStationSelect = 10,
            PlayStationStart = 11,
            PlayStationPS = 12,
            PlayStationShare = 13,
            PlayStationOptions = 14,
            PlayStationTouchpad = 15,
            PlayStationDpadUp = 16,
            PlayStationDpadDown = 17,
            PlayStationDpadLeft = 18,
            PlayStationDpadRight = 19,
        ]
    }
}

// ===== XBOX ICONS =====
define_icon_category! {
    Xbox {
        path: "ui/icons/controllers/xbox",
        atlas: (20, 5),
        icons: [
            XboxA = 0,
            XboxB = 1,
            XboxX = 2,
            XboxY = 3,
            XboxLB = 4,
            XboxLT = 5,
            XboxRB = 6,
            XboxRT = 7,
            XboxLS = 8,
            XboxRS = 9,
            XboxView = 10,
            XboxMenu = 11,
            XboxXbox = 12,
            XboxDpadUp = 13,
            XboxDpadDown = 14,
            XboxDpadLeft = 15,
            XboxDpadRight = 16,
        ]
    }
}

// ===== STEAM DECK ICONS =====
define_icon_category! {
    SteamDeck {
        path: "ui/icons/controllers/steam_deck",
        atlas: (20, 6),
        icons: [
            SteamDeckA = 0,
            SteamDeckB = 1,
            SteamDeckX = 2,
            SteamDeckY = 3,
            SteamDeckL1 = 4,
            SteamDeckL2 = 5,
            SteamDeckR1 = 6,
            SteamDeckR2 = 7,
            SteamDeckL4 = 8,
            SteamDeckL5 = 9,
            SteamDeckR4 = 10,
            SteamDeckR5 = 11,
            SteamDeckMenu = 12,
            SteamDeckView = 13,
            SteamDeckSteam = 14,
            SteamDeckQuickAccess = 15,
            SteamDeckTrackpadLeft = 16,
            SteamDeckTrackpadRight = 17,
            SteamDeckStickLeft = 18,
            SteamDeckStickRight = 19,
        ]
    }
}

// ===== INDICATORS/FLAIRS =====
define_icon_category! {
    Indicators {
        path: "ui/icons/controllers/indicators",
        atlas: (20, 5),
        icons: [
            ControllerBatteryEmpty = 0,
            ControllerBatteryHalf = 1,
            ControllerBatteryFull = 2,
            ControllerDisconnected = 3,
            ControllerConnectingA = 4,
            ControllerConnectingB = 5,
            ControllerGeneric = 6,
            FlairArrow0 = 7,
            FlairArrow1 = 8,
            FlairArrow2 = 9,
            FlairArrow3 = 10,
            FlairArrowBackforth = 11,
            FlairArrowLong = 12,
            FlairArrowShort = 13,
            FlairArrowZ = 14,
            FlairArrowsAll = 15,
            FlairArrowsHorizontal = 16,
            FlairArrowsVertical = 17,
            FlairNumber0 = 18,
            FlairNumber1 = 19,
            FlairNumber2 = 20,
            FlairNumber3 = 21,
            FlairNumber4 = 22,
            FlairNumber5 = 23,
            FlairNumber6 = 24,
            FlairNumber7 = 25,
            FlairNumber8 = 26,
            FlairNumber9 = 27,
            FlairDisabled = 28,
            FlairCross = 29,
            FlairPlus = 30,
            FlairCheck = 31,
            FlairInfo = 32,
            FlairRotate = 33,
        ]
    }
}
