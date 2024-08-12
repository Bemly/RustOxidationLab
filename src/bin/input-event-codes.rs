/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Input event codes
 *
 *    *** IMPORTANT ***
 * This file is not only included from C-code but also from devicetree source
 * files. As such this file MUST only contain comments and defines.
 *
 * Copyright (c) 1999-2002 Vojtech Pavlik
 * Copyright (c) 2015 Hans de Goede <hdegoede@redhat.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 as published by
 * the Free Software Foundation.
 */
#ifndef _UAPI_INPUT_EVENT_CODES_H
#define _UAPI_INPUT_EVENT_CODES_H

/*
 * Device properties and quirks
 */

const INPUT_PROP_POINTER: u8 = 0x00;	/* needs a pointer */
const INPUT_PROP_DIRECT: u8 = 0x01;	/* direct input devices */
const INPUT_PROP_BUTTONPAD: u8 = 0x02;	/* has button(s) under pad */
const INPUT_PROP_SEMI_MT: u8 = 0x03;	/* touch rectangle only */
const INPUT_PROP_TOPBUTTONPAD: u8 = 0x04;	/* softbuttons at top of pad */
const INPUT_PROP_POINTING_STICK: u8 = 0x05;	/* is a pointing stick */
const INPUT_PROP_ACCELEROMETER: u8 = 0x06;	/* has accelerometer */

const INPUT_PROP_MAX: u8 = 0x1f;
const INPUT_PROP_CNT: u8 = (INPUT_PROP_MAX + 1);

/*
 * Event types
 */

const EV_SYN: u8 = 0x00;
const EV_KEY: u8 = 0x01;
const EV_REL: u8 = 0x02;
const EV_ABS: u8 = 0x03;
const EV_MSC: u8 = 0x04;
const EV_SW: u8 = 0x05;
const EV_LED: u8 = 0x11;
const EV_SND: u8 = 0x12;
const EV_REP: u8 = 0x14;
const EV_FF: u8 = 0x15;
const EV_PWR: u8 = 0x16;
const EV_FF_STATUS: u8 = 0x17;
const EV_MAX: u8 = 0x1f;
const EV_CNT: u8 = (EV_MAX+1);

/*
 * Synchronization events.
 */

const SYN_REPORT: u8 = 0;
const SYN_CONFIG: u8 = 1;
const SYN_MT_REPORT: u8 = 2;
const SYN_DROPPED: u8 = 3;
const SYN_MAX: u8 = 0xf;
const SYN_CNT: u8 = (SYN_MAX+1);

/*
 * Keys and buttons
 *
 * Most of the keys/buttons are modeled after USB HUT 1.12
 * (see http://www.usb.org/developers/hidpage).
 * Abbreviations in the comments:
 * AC - Application Control
 * AL - Application Launch Button
 * SC - System Control
 */

const KEY_RESERVED: u8 = 0;
const KEY_ESC: u8 = 1;
const KEY_1: u8 = 2;
const KEY_2: u8 = 3;
const KEY_3: u8 = 4;
const KEY_4: u8 = 5;
const KEY_5: u8 = 6;
const KEY_6: u8 = 7;
const KEY_7: u8 = 8;
const KEY_8: u8 = 9;
const KEY_9: u8 = 10;
const KEY_0: u8 = 11;
const KEY_MINUS: u8 = 12;
const KEY_EQUAL: u8 = 13;
const KEY_BACKSPACE: u8 = 14;
const KEY_TAB: u8 = 15;
const KEY_Q: u8 = 16;
const KEY_W: u8 = 17;
const KEY_E: u8 = 18;
const KEY_R: u8 = 19;
const KEY_T: u8 = 20;
const KEY_Y: u8 = 21;
const KEY_U: u8 = 22;
const KEY_I: u8 = 23;
const KEY_O: u8 = 24;
const KEY_P: u8 = 25;
const KEY_LEFTBRACE: u8 = 26;
const KEY_RIGHTBRACE: u8 = 27;
const KEY_ENTER: u8 = 28;
const KEY_LEFTCTRL: u8 = 29;
const KEY_A: u8 = 30;
const KEY_S: u8 = 31;
const KEY_D: u8 = 32;
const KEY_F: u8 = 33;
const KEY_G: u8 = 34;
const KEY_H: u8 = 35;
const KEY_J: u8 = 36;
const KEY_K: u8 = 37;
const KEY_L: u8 = 38;
const KEY_SEMICOLON: u8 = 39;
const KEY_APOSTROPHE: u8 = 40;
const KEY_GRAVE: u8 = 41;
const KEY_LEFTSHIFT: u8 = 42;
const KEY_BACKSLASH: u8 = 43;
const KEY_Z: u8 = 44;
const KEY_X: u8 = 45;
const KEY_C: u8 = 46;
const KEY_V: u8 = 47;
const KEY_B: u8 = 48;
const KEY_N: u8 = 49;
const KEY_M: u8 = 50;
const KEY_COMMA: u8 = 51;
const KEY_DOT: u8 = 52;
const KEY_SLASH: u8 = 53;
const KEY_RIGHTSHIFT: u8 = 54;
const KEY_KPASTERISK: u8 = 55;
const KEY_LEFTALT: u8 = 56;
const KEY_SPACE: u8 = 57;
const KEY_CAPSLOCK: u8 = 58;
const KEY_F1: u8 = 59;
const KEY_F2: u8 = 60;
const KEY_F3: u8 = 61;
const KEY_F4: u8 = 62;
const KEY_F5: u8 = 63;
const KEY_F6: u8 = 64;
const KEY_F7: u8 = 65;
const KEY_F8: u8 = 66;
const KEY_F9: u8 = 67;
const KEY_F10: u8 = 68;
const KEY_NUMLOCK: u8 = 69;
const KEY_SCROLLLOCK: u8 = 70;
const KEY_KP7: u8 = 71;
const KEY_KP8: u8 = 72;
const KEY_KP9: u8 = 73;
const KEY_KPMINUS: u8 = 74;
const KEY_KP4: u8 = 75;
const KEY_KP5: u8 = 76;
const KEY_KP6: u8 = 77;
const KEY_KPPLUS: u8 = 78;
const KEY_KP1: u8 = 79;
const KEY_KP2: u8 = 80;
const KEY_KP3: u8 = 81;
const KEY_KP0: u8 = 82;
const KEY_KPDOT: u8 = 83;

const KEY_ZENKAKUHANKAKU: u8 = 85;
const KEY_102ND: u8 = 86;
const KEY_F11: u8 = 87;
const KEY_F12: u8 = 88;
const KEY_RO: u8 = 89;
const KEY_KATAKANA: u8 = 90;
const KEY_HIRAGANA: u8 = 91;
const KEY_HENKAN: u8 = 92;
const KEY_KATAKANAHIRAGANA: u8 = 93;
const KEY_MUHENKAN: u8 = 94;
const KEY_KPJPCOMMA: u8 = 95;
const KEY_KPENTER: u8 = 96;
const KEY_RIGHTCTRL: u8 = 97;
const KEY_KPSLASH: u8 = 98;
const KEY_SYSRQ: u8 = 99;
const KEY_RIGHTALT: u8 = 100;
const KEY_LINEFEED: u8 = 101;
const KEY_HOME: u8 = 102;
const KEY_UP: u8 = 103;
const KEY_PAGEUP: u8 = 104;
const KEY_LEFT: u8 = 105;
const KEY_RIGHT: u8 = 106;
const KEY_END: u8 = 107;
const KEY_DOWN: u8 = 108;
const KEY_PAGEDOWN: u8 = 109;
const KEY_INSERT: u8 = 110;
const KEY_DELETE: u8 = 111;
const KEY_MACRO: u8 = 112;
const KEY_MUTE: u8 = 113;
const KEY_VOLUMEDOWN: u8 = 114;
const KEY_VOLUMEUP: u8 = 115;
const KEY_POWER: u8 = 116;	/* SC System Power Down */
const KEY_KPEQUAL: u8 = 117;
const KEY_KPPLUSMINUS: u8 = 118;
const KEY_PAUSE: u8 = 119;
const KEY_SCALE: u8 = 120;	/* AL Compiz Scale (Expose) */

const KEY_KPCOMMA: u8 = 121;
const KEY_HANGEUL: u8 = 122;
#define KEY_HANGUEL		KEY_HANGEUL
const KEY_HANJA: u8 = 123;
const KEY_YEN: u8 = 124;
const KEY_LEFTMETA: u8 = 125;
const KEY_RIGHTMETA: u8 = 126;
const KEY_COMPOSE: u8 = 127;

const KEY_STOP: u8 = 128;	/* AC Stop */
const KEY_AGAIN: u8 = 129;
const KEY_PROPS: u8 = 130;	/* AC Properties */
const KEY_UNDO: u8 = 131;	/* AC Undo */
const KEY_FRONT: u8 = 132;
const KEY_COPY: u8 = 133;	/* AC Copy */
const KEY_OPEN: u8 = 134;	/* AC Open */
const KEY_PASTE: u8 = 135;	/* AC Paste */
const KEY_FIND: u8 = 136;	/* AC Search */
const KEY_CUT: u8 = 137;	/* AC Cut */
const KEY_HELP: u8 = 138;	/* AL Integrated Help Center */
const KEY_MENU: u8 = 139;	/* Menu (show menu) */
const KEY_CALC: u8 = 140;	/* AL Calculator */
const KEY_SETUP: u8 = 141;
const KEY_SLEEP: u8 = 142;	/* SC System Sleep */
const KEY_WAKEUP: u8 = 143;	/* System Wake Up */
const KEY_FILE: u8 = 144;	/* AL Local Machine Browser */
const KEY_SENDFILE: u8 = 145;
const KEY_DELETEFILE: u8 = 146;
const KEY_XFER: u8 = 147;
const KEY_PROG1: u8 = 148;
const KEY_PROG2: u8 = 149;
const KEY_WWW: u8 = 150;	/* AL Internet Browser */
const KEY_MSDOS: u8 = 151;
const KEY_COFFEE: u8 = 152;	/* AL Terminal Lock/Screensaver */
#define KEY_SCREENLOCK		KEY_COFFEE
const KEY_ROTATE_DISPLAY: u8 = 153;	/* Display orientation for e.g. tablets */
#define KEY_DIRECTION		KEY_ROTATE_DISPLAY
const KEY_CYCLEWINDOWS: u8 = 154;
const KEY_MAIL: u8 = 155;
const KEY_BOOKMARKS: u8 = 156;	/* AC Bookmarks */
const KEY_COMPUTER: u8 = 157;
const KEY_BACK: u8 = 158;	/* AC Back */
const KEY_FORWARD: u8 = 159;	/* AC Forward */
const KEY_CLOSECD: u8 = 160;
const KEY_EJECTCD: u8 = 161;
const KEY_EJECTCLOSECD: u8 = 162;
const KEY_NEXTSONG: u8 = 163;
const KEY_PLAYPAUSE: u8 = 164;
const KEY_PREVIOUSSONG: u8 = 165;
const KEY_STOPCD: u8 = 166;
const KEY_RECORD: u8 = 167;
const KEY_REWIND: u8 = 168;
const KEY_PHONE: u8 = 169;	/* Media Select Telephone */
const KEY_ISO: u8 = 170;
const KEY_CONFIG: u8 = 171;	/* AL Consumer Control Configuration */
const KEY_HOMEPAGE: u8 = 172;	/* AC Home */
const KEY_REFRESH: u8 = 173;	/* AC Refresh */
const KEY_EXIT: u8 = 174;	/* AC Exit */
const KEY_MOVE: u8 = 175;
const KEY_EDIT: u8 = 176;
const KEY_SCROLLUP: u8 = 177;
const KEY_SCROLLDOWN: u8 = 178;
const KEY_KPLEFTPAREN: u8 = 179;
const KEY_KPRIGHTPAREN: u8 = 180;
const KEY_NEW: u8 = 181;	/* AC New */
const KEY_REDO: u8 = 182;	/* AC Redo/Repeat */

const KEY_F13: u8 = 183;
const KEY_F14: u8 = 184;
const KEY_F15: u8 = 185;
const KEY_F16: u8 = 186;
const KEY_F17: u8 = 187;
const KEY_F18: u8 = 188;
const KEY_F19: u8 = 189;
const KEY_F20: u8 = 190;
const KEY_F21: u8 = 191;
const KEY_F22: u8 = 192;
const KEY_F23: u8 = 193;
const KEY_F24: u8 = 194;

const KEY_PLAYCD: u8 = 200;
const KEY_PAUSECD: u8 = 201;
const KEY_PROG3: u8 = 202;
const KEY_PROG4: u8 = 203;
const KEY_ALL_APPLICATIONS: u8 = 204;	/* AC Desktop Show All Applications */
#define KEY_DASHBOARD		KEY_ALL_APPLICATIONS
const KEY_SUSPEND: u8 = 205;
const KEY_CLOSE: u8 = 206;	/* AC Close */
const KEY_PLAY: u8 = 207;
const KEY_FASTFORWARD: u8 = 208;
const KEY_BASSBOOST: u8 = 209;
const KEY_PRINT: u8 = 210;	/* AC Print */
const KEY_HP: u8 = 211;
const KEY_CAMERA: u8 = 212;
const KEY_SOUND: u8 = 213;
const KEY_QUESTION: u8 = 214;
const KEY_EMAIL: u8 = 215;
const KEY_CHAT: u8 = 216;
const KEY_SEARCH: u8 = 217;
const KEY_CONNECT: u8 = 218;
const KEY_FINANCE: u8 = 219;	/* AL Checkbook/Finance */
const KEY_SPORT: u8 = 220;
const KEY_SHOP: u8 = 221;
const KEY_ALTERASE: u8 = 222;
const KEY_CANCEL: u8 = 223;	/* AC Cancel */
const KEY_BRIGHTNESSDOWN: u8 = 224;
const KEY_BRIGHTNESSUP: u8 = 225;
const KEY_MEDIA: u8 = 226;

const KEY_SWITCHVIDEOMODE: u8 = 227;	/* Cycle between available video
					   outputs (Monitor/LCD/TV-out/etc) */
const KEY_KBDILLUMTOGGLE: u8 = 228;
const KEY_KBDILLUMDOWN: u8 = 229;
const KEY_KBDILLUMUP: u8 = 230;

const KEY_SEND: u8 = 231;	/* AC Send */
const KEY_REPLY: u8 = 232;	/* AC Reply */
const KEY_FORWARDMAIL: u8 = 233;	/* AC Forward Msg */
const KEY_SAVE: u8 = 234;	/* AC Save */
const KEY_DOCUMENTS: u8 = 235;

const KEY_BATTERY: u8 = 236;

const KEY_BLUETOOTH: u8 = 237;
const KEY_WLAN: u8 = 238;
const KEY_UWB: u8 = 239;

const KEY_UNKNOWN: u8 = 240;

const KEY_VIDEO_NEXT: u8 = 241;	/* drive next video source */
const KEY_VIDEO_PREV: u8 = 242;	/* drive previous video source */
const KEY_BRIGHTNESS_CYCLE: u8 = 243;	/* brightness up, after max is min */
const KEY_BRIGHTNESS_AUTO: u8 = 244;	/* Set Auto Brightness: manual
					  brightness control is off,
					  rely on ambient */
#define KEY_BRIGHTNESS_ZERO	KEY_BRIGHTNESS_AUTO
const KEY_DISPLAY_OFF: u8 = 245;	/* display device to off state */

const KEY_WWAN: u8 = 246;	/* Wireless WAN (LTE, UMTS, GSM, etc.) */
#define KEY_WIMAX		KEY_WWAN
const KEY_RFKILL: u8 = 247;	/* Key that controls all radios */

const KEY_MICMUTE: u8 = 248;	/* Mute / unmute the microphone */

/* Code 255 is reserved for special needs of AT keyboard driver */

const BTN_MISC: u8 = 0x100;
const BTN_0: u8 = 0x100;
const BTN_1: u8 = 0x101;
const BTN_2: u8 = 0x102;
const BTN_3: u8 = 0x103;
const BTN_4: u8 = 0x104;
const BTN_5: u8 = 0x105;
const BTN_6: u8 = 0x106;
const BTN_7: u8 = 0x107;
const BTN_8: u8 = 0x108;
const BTN_9: u8 = 0x109;

const BTN_MOUSE: u8 = 0x110;
const BTN_LEFT: u8 = 0x110;
const BTN_RIGHT: u8 = 0x111;
const BTN_MIDDLE: u8 = 0x112;
const BTN_SIDE: u8 = 0x113;
const BTN_EXTRA: u8 = 0x114;
const BTN_FORWARD: u8 = 0x115;
const BTN_BACK: u8 = 0x116;
const BTN_TASK: u8 = 0x117;

const BTN_JOYSTICK: u8 = 0x120;
const BTN_TRIGGER: u8 = 0x120;
const BTN_THUMB: u8 = 0x121;
const BTN_THUMB2: u8 = 0x122;
const BTN_TOP: u8 = 0x123;
const BTN_TOP2: u8 = 0x124;
const BTN_PINKIE: u8 = 0x125;
const BTN_BASE: u8 = 0x126;
const BTN_BASE2: u8 = 0x127;
const BTN_BASE3: u8 = 0x128;
const BTN_BASE4: u8 = 0x129;
const BTN_BASE5: u8 = 0x12a;
const BTN_BASE6: u8 = 0x12b;
const BTN_DEAD: u8 = 0x12f;

const BTN_GAMEPAD: u8 = 0x130;
const BTN_SOUTH: u8 = 0x130;
const BTN_A: u8 = BTN_SOUTH
const BTN_EAST: u8 = 0x131;
const BTN_B: u8 = BTN_EAST
const BTN_C: u8 = 0x132;
const BTN_NORTH: u8 = 0x133;
const BTN_X: u8 = BTN_NORTH
const BTN_WEST: u8 = 0x134;
const BTN_Y: u8 = BTN_WEST
const BTN_Z: u8 = 0x135;
const BTN_TL: u8 = 0x136;
const BTN_TR: u8 = 0x137;
const BTN_TL2: u8 = 0x138;
const BTN_TR2: u8 = 0x139;
const BTN_SELECT: u8 = 0x13a;
const BTN_START: u8 = 0x13b;
const BTN_MODE: u8 = 0x13c;
const BTN_THUMBL: u8 = 0x13d;
const BTN_THUMBR: u8 = 0x13e;

const BTN_DIGI: u8 = 0x140;
const BTN_TOOL_PEN: u8 = 0x140;
const BTN_TOOL_RUBBER: u8 = 0x141;
const BTN_TOOL_BRUSH: u8 = 0x142;
const BTN_TOOL_PENCIL: u8 = 0x143;
const BTN_TOOL_AIRBRUSH: u8 = 0x144;
const BTN_TOOL_FINGER: u8 = 0x145;
const BTN_TOOL_MOUSE: u8 = 0x146;
const BTN_TOOL_LENS: u8 = 0x147;
const BTN_TOOL_QUINTTAP: u8 = 0x148;	/* Five fingers on trackpad */
const BTN_STYLUS3: u8 = 0x149;
const BTN_TOUCH: u8 = 0x14a;
const BTN_STYLUS: u8 = 0x14b;
const BTN_STYLUS2: u8 = 0x14c;
const BTN_TOOL_DOUBLETAP: u8 = 0x14d;
const BTN_TOOL_TRIPLETAP: u8 = 0x14e;
const BTN_TOOL_QUADTAP: u8 = 0x14f;	/* Four fingers on trackpad */

const BTN_WHEEL: u8 = 0x150;
const BTN_GEAR_DOWN: u8 = 0x150;
const BTN_GEAR_UP: u8 = 0x151;

const KEY_OK: u8 = 0x160;
const KEY_SELECT: u8 = 0x161;
const KEY_GOTO: u8 = 0x162;
const KEY_CLEAR: u8 = 0x163;
const KEY_POWER2: u8 = 0x164;
const KEY_OPTION: u8 = 0x165;
const KEY_INFO: u8 = 0x166;	/* AL OEM Features/Tips/Tutorial */
const KEY_TIME: u8 = 0x167;
const KEY_VENDOR: u8 = 0x168;
const KEY_ARCHIVE: u8 = 0x169;
const KEY_PROGRAM: u8 = 0x16a;	/* Media Select Program Guide */
const KEY_CHANNEL: u8 = 0x16b;
const KEY_FAVORITES: u8 = 0x16c;
const KEY_EPG: u8 = 0x16d;
const KEY_PVR: u8 = 0x16e;	/* Media Select Home */
const KEY_MHP: u8 = 0x16f;
const KEY_LANGUAGE: u8 = 0x170;
const KEY_TITLE: u8 = 0x171;
const KEY_SUBTITLE: u8 = 0x172;
const KEY_ANGLE: u8 = 0x173;
const KEY_FULL_SCREEN: u8 = 0x174;	/* AC View Toggle */
#define KEY_ZOOM		KEY_FULL_SCREEN
const KEY_MODE: u8 = 0x175;
const KEY_KEYBOARD: u8 = 0x176;
const KEY_ASPECT_RATIO: u8 = 0x177;	/* HUTRR37: Aspect */
#define KEY_SCREEN		KEY_ASPECT_RATIO
const KEY_PC: u8 = 0x178;	/* Media Select Computer */
const KEY_TV: u8 = 0x179;	/* Media Select TV */
const KEY_TV2: u8 = 0x17a;	/* Media Select Cable */
const KEY_VCR: u8 = 0x17b;	/* Media Select VCR */
const KEY_VCR2: u8 = 0x17c;	/* VCR Plus */
const KEY_SAT: u8 = 0x17d;	/* Media Select Satellite */
const KEY_SAT2: u8 = 0x17e;
const KEY_CD: u8 = 0x17f;	/* Media Select CD */
const KEY_TAPE: u8 = 0x180;	/* Media Select Tape */
const KEY_RADIO: u8 = 0x181;
const KEY_TUNER: u8 = 0x182;	/* Media Select Tuner */
const KEY_PLAYER: u8 = 0x183;
const KEY_TEXT: u8 = 0x184;
const KEY_DVD: u8 = 0x185;	/* Media Select DVD */
const KEY_AUX: u8 = 0x186;
const KEY_MP3: u8 = 0x187;
const KEY_AUDIO: u8 = 0x188;	/* AL Audio Browser */
const KEY_VIDEO: u8 = 0x189;	/* AL Movie Browser */
const KEY_DIRECTORY: u8 = 0x18a;
const KEY_LIST: u8 = 0x18b;
const KEY_MEMO: u8 = 0x18c;	/* Media Select Messages */
const KEY_CALENDAR: u8 = 0x18d;
const KEY_RED: u8 = 0x18e;
const KEY_GREEN: u8 = 0x18f;
const KEY_YELLOW: u8 = 0x190;
const KEY_BLUE: u8 = 0x191;
const KEY_CHANNELUP: u8 = 0x192;	/* Channel Increment */
const KEY_CHANNELDOWN: u8 = 0x193;	/* Channel Decrement */
const KEY_FIRST: u8 = 0x194;
const KEY_LAST: u8 = 0x195;	/* Recall Last */
const KEY_AB: u8 = 0x196;
const KEY_NEXT: u8 = 0x197;
const KEY_RESTART: u8 = 0x198;
const KEY_SLOW: u8 = 0x199;
const KEY_SHUFFLE: u8 = 0x19a;
const KEY_BREAK: u8 = 0x19b;
const KEY_PREVIOUS: u8 = 0x19c;
const KEY_DIGITS: u8 = 0x19d;
const KEY_TEEN: u8 = 0x19e;
const KEY_TWEN: u8 = 0x19f;
const KEY_VIDEOPHONE: u8 = 0x1a0;	/* Media Select Video Phone */
const KEY_GAMES: u8 = 0x1a1;	/* Media Select Games */
const KEY_ZOOMIN: u8 = 0x1a2;	/* AC Zoom In */
const KEY_ZOOMOUT: u8 = 0x1a3;	/* AC Zoom Out */
const KEY_ZOOMRESET: u8 = 0x1a4;	/* AC Zoom */
const KEY_WORDPROCESSOR: u8 = 0x1a5;	/* AL Word Processor */
const KEY_EDITOR: u8 = 0x1a6;	/* AL Text Editor */
const KEY_SPREADSHEET: u8 = 0x1a7;	/* AL Spreadsheet */
const KEY_GRAPHICSEDITOR: u8 = 0x1a8;	/* AL Graphics Editor */
const KEY_PRESENTATION: u8 = 0x1a9;	/* AL Presentation App */
const KEY_DATABASE: u8 = 0x1aa;	/* AL Database App */
const KEY_NEWS: u8 = 0x1ab;	/* AL Newsreader */
const KEY_VOICEMAIL: u8 = 0x1ac;	/* AL Voicemail */
const KEY_ADDRESSBOOK: u8 = 0x1ad;	/* AL Contacts/Address Book */
const KEY_MESSENGER: u8 = 0x1ae;	/* AL Instant Messaging */
const KEY_DISPLAYTOGGLE: u8 = 0x1af;	/* Turn display (LCD) on and off */
#define KEY_BRIGHTNESS_TOGGLE	KEY_DISPLAYTOGGLE
const KEY_SPELLCHECK: u8 = 0x1b0;   /* AL Spell Check */
const KEY_LOGOFF: u8 = 0x1b1;   /* AL Logoff */

const KEY_DOLLAR: u8 = 0x1b2;
const KEY_EURO: u8 = 0x1b3;

const KEY_FRAMEBACK: u8 = 0x1b4;	/* Consumer - transport controls */
const KEY_FRAMEFORWARD: u8 = 0x1b5;
const KEY_CONTEXT_MENU: u8 = 0x1b6;	/* GenDesc - system context menu */
const KEY_MEDIA_REPEAT: u8 = 0x1b7;	/* Consumer - transport control */
const KEY_10CHANNELSUP: u8 = 0x1b8;	/* 10 channels up (10+) */
const KEY_10CHANNELSDOWN: u8 = 0x1b9;	/* 10 channels down (10-) */
const KEY_IMAGES: u8 = 0x1ba;	/* AL Image Browser */
const KEY_NOTIFICATION_CENTER: u8 = 0x1bc;	/* Show/hide the notification center */
const KEY_PICKUP_PHONE: u8 = 0x1bd;	/* Answer incoming call */
const KEY_HANGUP_PHONE: u8 = 0x1be;	/* Decline incoming call */

const KEY_DEL_EOL: u8 = 0x1c0;
const KEY_DEL_EOS: u8 = 0x1c1;
const KEY_INS_LINE: u8 = 0x1c2;
const KEY_DEL_LINE: u8 = 0x1c3;

const KEY_FN: u8 = 0x1d0;
const KEY_FN_ESC: u8 = 0x1d1;
const KEY_FN_F1: u8 = 0x1d2;
const KEY_FN_F2: u8 = 0x1d3;
const KEY_FN_F3: u8 = 0x1d4;
const KEY_FN_F4: u8 = 0x1d5;
const KEY_FN_F5: u8 = 0x1d6;
const KEY_FN_F6: u8 = 0x1d7;
const KEY_FN_F7: u8 = 0x1d8;
const KEY_FN_F8: u8 = 0x1d9;
const KEY_FN_F9: u8 = 0x1da;
const KEY_FN_F10: u8 = 0x1db;
const KEY_FN_F11: u8 = 0x1dc;
const KEY_FN_F12: u8 = 0x1dd;
const KEY_FN_1: u8 = 0x1de;
const KEY_FN_2: u8 = 0x1df;
const KEY_FN_D: u8 = 0x1e0;
const KEY_FN_E: u8 = 0x1e1;
const KEY_FN_F: u8 = 0x1e2;
const KEY_FN_S: u8 = 0x1e3;
const KEY_FN_B: u8 = 0x1e4;
const KEY_FN_RIGHT_SHIFT: u8 = 0x1e5;

const KEY_BRL_DOT1: u8 = 0x1f1;
const KEY_BRL_DOT2: u8 = 0x1f2;
const KEY_BRL_DOT3: u8 = 0x1f3;
const KEY_BRL_DOT4: u8 = 0x1f4;
const KEY_BRL_DOT5: u8 = 0x1f5;
const KEY_BRL_DOT6: u8 = 0x1f6;
const KEY_BRL_DOT7: u8 = 0x1f7;
const KEY_BRL_DOT8: u8 = 0x1f8;
const KEY_BRL_DOT9: u8 = 0x1f9;
const KEY_BRL_DOT10: u8 = 0x1fa;

const KEY_NUMERIC_0: u8 = 0x200;	/* used by phones, remote controls, */
const KEY_NUMERIC_1: u8 = 0x201;	/* and other keypads */
const KEY_NUMERIC_2: u8 = 0x202;
const KEY_NUMERIC_3: u8 = 0x203;
const KEY_NUMERIC_4: u8 = 0x204;
const KEY_NUMERIC_5: u8 = 0x205;
const KEY_NUMERIC_6: u8 = 0x206;
const KEY_NUMERIC_7: u8 = 0x207;
const KEY_NUMERIC_8: u8 = 0x208;
const KEY_NUMERIC_9: u8 = 0x209;
const KEY_NUMERIC_STAR: u8 = 0x20a;
const KEY_NUMERIC_POUND: u8 = 0x20b;
const KEY_NUMERIC_A: u8 = 0x20c;	/* Phone key A - HUT Telephony 0xb9 */
const KEY_NUMERIC_B: u8 = 0x20d;
const KEY_NUMERIC_C: u8 = 0x20e;
const KEY_NUMERIC_D: u8 = 0x20f;

const KEY_CAMERA_FOCUS: u8 = 0x210;
const KEY_WPS_BUTTON: u8 = 0x211;	/* WiFi Protected Setup key */

const KEY_TOUCHPAD_TOGGLE: u8 = 0x212;	/* Request switch touchpad on or off */
const KEY_TOUCHPAD_ON: u8 = 0x213;
const KEY_TOUCHPAD_OFF: u8 = 0x214;

const KEY_CAMERA_ZOOMIN: u8 = 0x215;
const KEY_CAMERA_ZOOMOUT: u8 = 0x216;
const KEY_CAMERA_UP: u8 = 0x217;
const KEY_CAMERA_DOWN: u8 = 0x218;
const KEY_CAMERA_LEFT: u8 = 0x219;
const KEY_CAMERA_RIGHT: u8 = 0x21a;

const KEY_ATTENDANT_ON: u8 = 0x21b;
const KEY_ATTENDANT_OFF: u8 = 0x21c;
const KEY_ATTENDANT_TOGGLE: u8 = 0x21d;	/* Attendant call on or off */
const KEY_LIGHTS_TOGGLE: u8 = 0x21e;	/* Reading light on or off */

const BTN_DPAD_UP: u8 = 0x220;
const BTN_DPAD_DOWN: u8 = 0x221;
const BTN_DPAD_LEFT: u8 = 0x222;
const BTN_DPAD_RIGHT: u8 = 0x223;

const KEY_ALS_TOGGLE: u8 = 0x230;	/* Ambient light sensor */
const KEY_ROTATE_LOCK_TOGGLE: u8 = 0x231;	/* Display rotation lock */
const KEY_REFRESH_RATE_TOGGLE: u8 = 0x232;	/* Display refresh rate toggle */

const KEY_BUTTONCONFIG: u8 = 0x240;	/* AL Button Configuration */
const KEY_TASKMANAGER: u8 = 0x241;	/* AL Task/Project Manager */
const KEY_JOURNAL: u8 = 0x242;	/* AL Log/Journal/Timecard */
const KEY_CONTROLPANEL: u8 = 0x243;	/* AL Control Panel */
const KEY_APPSELECT: u8 = 0x244;	/* AL Select Task/Application */
const KEY_SCREENSAVER: u8 = 0x245;	/* AL Screen Saver */
const KEY_VOICECOMMAND: u8 = 0x246;	/* Listening Voice Command */
const KEY_ASSISTANT: u8 = 0x247;	/* AL Context-aware desktop assistant */
const KEY_KBD_LAYOUT_NEXT: u8 = 0x248;	/* AC Next Keyboard Layout Select */
const KEY_EMOJI_PICKER: u8 = 0x249;	/* Show/hide emoji picker (HUTRR101) */
const KEY_DICTATE: u8 = 0x24a;	/* Start or Stop Voice Dictation Session (HUTRR99) */
const KEY_CAMERA_ACCESS_ENABLE: u8 = 0x24b;	/* Enables programmatic access to camera devices. (HUTRR72) */
const KEY_CAMERA_ACCESS_DISABLE: u8 = 0x24c;	/* Disables programmatic access to camera devices. (HUTRR72) */
const KEY_CAMERA_ACCESS_TOGGLE: u8 = 0x24d;	/* Toggles the current state of the camera access control. (HUTRR72) */
const KEY_ACCESSIBILITY: u8 = 0x24e;	/* Toggles the system bound accessibility UI/command (HUTRR116) */
const KEY_DO_NOT_DISTURB: u8 = 0x24f;	/* Toggles the system-wide "Do Not Disturb" control (HUTRR94)*/

const KEY_BRIGHTNESS_MIN: u8 = 0x250;	/* Set Brightness to Minimum */
const KEY_BRIGHTNESS_MAX: u8 = 0x251;	/* Set Brightness to Maximum */

const KEY_KBDINPUTASSIST_PREV: u8 = 0x260;
const KEY_KBDINPUTASSIST_NEXT: u8 = 0x261;
const KEY_KBDINPUTASSIST_PREVGROUP: u8 = 0x262;
const KEY_KBDINPUTASSIST_NEXTGROUP: u8 = 0x263;
const KEY_KBDINPUTASSIST_ACCEPT: u8 = 0x264;
const KEY_KBDINPUTASSIST_CANCEL: u8 = 0x265;

/* Diagonal movement keys */
const KEY_RIGHT_UP: u8 = 0x266;
const KEY_RIGHT_DOWN: u8 = 0x267;
const KEY_LEFT_UP: u8 = 0x268;
const KEY_LEFT_DOWN: u8 = 0x269;

const KEY_ROOT_MENU: u8 = 0x26a; /* Show Device's Root Menu */
/* Show Top Menu of the Media (e.g. DVD) */
const KEY_MEDIA_TOP_MENU: u8 = 0x26b;
const KEY_NUMERIC_11: u8 = 0x26c;
const KEY_NUMERIC_12: u8 = 0x26d;
/*
 * Toggle Audio Description: refers to an audio service that helps blind and
 * visually impaired consumers understand the action in a program. Note: in
 * some countries this is referred to as "Video Description".
 */
const KEY_AUDIO_DESC: u8 = 0x26e;
const KEY_3D_MODE: u8 = 0x26f;
const KEY_NEXT_FAVORITE: u8 = 0x270;
const KEY_STOP_RECORD: u8 = 0x271;
const KEY_PAUSE_RECORD: u8 = 0x272;
const KEY_VOD: u8 = 0x273; /* Video on Demand */
const KEY_UNMUTE: u8 = 0x274;
const KEY_FASTREVERSE: u8 = 0x275;
const KEY_SLOWREVERSE: u8 = 0x276;
/*
 * Control a data application associated with the currently viewed channel,
 * e.g. teletext or data broadcast application (MHEG, MHP, HbbTV, etc.)
 */
const KEY_DATA: u8 = 0x277;
const KEY_ONSCREEN_KEYBOARD: u8 = 0x278;
/* Electronic privacy screen control */
const KEY_PRIVACY_SCREEN_TOGGLE: u8 = 0x279;

/* Select an area of screen to be copied */
const KEY_SELECTIVE_SCREENSHOT: u8 = 0x27a;

/* Move the focus to the next or previous user controllable element within a UI container */
const KEY_NEXT_ELEMENT: u8 = 0x27b;
const KEY_PREVIOUS_ELEMENT: u8 = 0x27c;

/* Toggle Autopilot engagement */
const KEY_AUTOPILOT_ENGAGE_TOGGLE: u8 = 0x27d;

/* Shortcut Keys */
const KEY_MARK_WAYPOINT: u8 = 0x27e;
const KEY_SOS: u8 = 0x27f;
const KEY_NAV_CHART: u8 = 0x280;
const KEY_FISHING_CHART: u8 = 0x281;
const KEY_SINGLE_RANGE_RADAR: u8 = 0x282;
const KEY_DUAL_RANGE_RADAR: u8 = 0x283;
const KEY_RADAR_OVERLAY: u8 = 0x284;
const KEY_TRADITIONAL_SONAR: u8 = 0x285;
const KEY_CLEARVU_SONAR: u8 = 0x286;
const KEY_SIDEVU_SONAR: u8 = 0x287;
const KEY_NAV_INFO: u8 = 0x288;
const KEY_BRIGHTNESS_MENU: u8 = 0x289;

/*
 * Some keyboards have keys which do not have a defined meaning, these keys
 * are intended to be programmed / bound to macros by the user. For most
 * keyboards with these macro-keys the key-sequence to inject, or action to
 * take, is all handled by software on the host side. So from the kernel's
 * point of view these are just normal keys.
 *
 * The KEY_MACRO# codes below are intended for such keys, which may be labeled
 * e.g. G1-G18, or S1 - S30. The KEY_MACRO# codes MUST NOT be used for keys
 * where the marking on the key does indicate a defined meaning / purpose.
 *
 * The KEY_MACRO# codes MUST also NOT be used as fallback for when no existing
 * KEY_FOO define matches the marking / purpose. In this case a new KEY_FOO
 * define MUST be added.
 */
const KEY_MACRO1: u8 = 0x290;
const KEY_MACRO2: u8 = 0x291;
const KEY_MACRO3: u8 = 0x292;
const KEY_MACRO4: u8 = 0x293;
const KEY_MACRO5: u8 = 0x294;
const KEY_MACRO6: u8 = 0x295;
const KEY_MACRO7: u8 = 0x296;
const KEY_MACRO8: u8 = 0x297;
const KEY_MACRO9: u8 = 0x298;
const KEY_MACRO10: u8 = 0x299;
const KEY_MACRO11: u8 = 0x29a;
const KEY_MACRO12: u8 = 0x29b;
const KEY_MACRO13: u8 = 0x29c;
const KEY_MACRO14: u8 = 0x29d;
const KEY_MACRO15: u8 = 0x29e;
const KEY_MACRO16: u8 = 0x29f;
const KEY_MACRO17: u8 = 0x2a0;
const KEY_MACRO18: u8 = 0x2a1;
const KEY_MACRO19: u8 = 0x2a2;
const KEY_MACRO20: u8 = 0x2a3;
const KEY_MACRO21: u8 = 0x2a4;
const KEY_MACRO22: u8 = 0x2a5;
const KEY_MACRO23: u8 = 0x2a6;
const KEY_MACRO24: u8 = 0x2a7;
const KEY_MACRO25: u8 = 0x2a8;
const KEY_MACRO26: u8 = 0x2a9;
const KEY_MACRO27: u8 = 0x2aa;
const KEY_MACRO28: u8 = 0x2ab;
const KEY_MACRO29: u8 = 0x2ac;
const KEY_MACRO30: u8 = 0x2ad;

/*
 * Some keyboards with the macro-keys described above have some extra keys
 * for controlling the host-side software responsible for the macro handling:
 * -A macro recording start/stop key. Note that not all keyboards which emit
 *  KEY_MACRO_RECORD_START will also emit KEY_MACRO_RECORD_STOP if
 *  KEY_MACRO_RECORD_STOP is not advertised, then KEY_MACRO_RECORD_START
 *  should be interpreted as a recording start/stop toggle;
 * -Keys for switching between different macro (pre)sets, either a key for
 *  cycling through the configured presets or keys to directly select a preset.
 */
const KEY_MACRO_RECORD_START: u8 = 0x2b0;
const KEY_MACRO_RECORD_STOP: u8 = 0x2b1;
const KEY_MACRO_PRESET_CYCLE: u8 = 0x2b2;
const KEY_MACRO_PRESET1: u8 = 0x2b3;
const KEY_MACRO_PRESET2: u8 = 0x2b4;
const KEY_MACRO_PRESET3: u8 = 0x2b5;

/*
 * Some keyboards have a buildin LCD panel where the contents are controlled
 * by the host. Often these have a number of keys directly below the LCD
 * intended for controlling a menu shown on the LCD. These keys often don't
 * have any labeling so we just name them KEY_KBD_LCD_MENU#
 */
const KEY_KBD_LCD_MENU1: u8 = 0x2b8;
const KEY_KBD_LCD_MENU2: u8 = 0x2b9;
const KEY_KBD_LCD_MENU3: u8 = 0x2ba;
const KEY_KBD_LCD_MENU4: u8 = 0x2bb;
const KEY_KBD_LCD_MENU5: u8 = 0x2bc;

const BTN_TRIGGER_HAPPY: u8 = 0x2c0;
const BTN_TRIGGER_HAPPY1: u8 = 0x2c0;
const BTN_TRIGGER_HAPPY2: u8 = 0x2c1;
const BTN_TRIGGER_HAPPY3: u8 = 0x2c2;
const BTN_TRIGGER_HAPPY4: u8 = 0x2c3;
const BTN_TRIGGER_HAPPY5: u8 = 0x2c4;
const BTN_TRIGGER_HAPPY6: u8 = 0x2c5;
const BTN_TRIGGER_HAPPY7: u8 = 0x2c6;
const BTN_TRIGGER_HAPPY8: u8 = 0x2c7;
const BTN_TRIGGER_HAPPY9: u8 = 0x2c8;
const BTN_TRIGGER_HAPPY10: u8 = 0x2c9;
const BTN_TRIGGER_HAPPY11: u8 = 0x2ca;
const BTN_TRIGGER_HAPPY12: u8 = 0x2cb;
const BTN_TRIGGER_HAPPY13: u8 = 0x2cc;
const BTN_TRIGGER_HAPPY14: u8 = 0x2cd;
const BTN_TRIGGER_HAPPY15: u8 = 0x2ce;
const BTN_TRIGGER_HAPPY16: u8 = 0x2cf;
const BTN_TRIGGER_HAPPY17: u8 = 0x2d0;
const BTN_TRIGGER_HAPPY18: u8 = 0x2d1;
const BTN_TRIGGER_HAPPY19: u8 = 0x2d2;
const BTN_TRIGGER_HAPPY20: u8 = 0x2d3;
const BTN_TRIGGER_HAPPY21: u8 = 0x2d4;
const BTN_TRIGGER_HAPPY22: u8 = 0x2d5;
const BTN_TRIGGER_HAPPY23: u8 = 0x2d6;
const BTN_TRIGGER_HAPPY24: u8 = 0x2d7;
const BTN_TRIGGER_HAPPY25: u8 = 0x2d8;
const BTN_TRIGGER_HAPPY26: u8 = 0x2d9;
const BTN_TRIGGER_HAPPY27: u8 = 0x2da;
const BTN_TRIGGER_HAPPY28: u8 = 0x2db;
const BTN_TRIGGER_HAPPY29: u8 = 0x2dc;
const BTN_TRIGGER_HAPPY30: u8 = 0x2dd;
const BTN_TRIGGER_HAPPY31: u8 = 0x2de;
const BTN_TRIGGER_HAPPY32: u8 = 0x2df;
const BTN_TRIGGER_HAPPY33: u8 = 0x2e0;
const BTN_TRIGGER_HAPPY34: u8 = 0x2e1;
const BTN_TRIGGER_HAPPY35: u8 = 0x2e2;
const BTN_TRIGGER_HAPPY36: u8 = 0x2e3;
const BTN_TRIGGER_HAPPY37: u8 = 0x2e4;
const BTN_TRIGGER_HAPPY38: u8 = 0x2e5;
const BTN_TRIGGER_HAPPY39: u8 = 0x2e6;
const BTN_TRIGGER_HAPPY40: u8 = 0x2e7;

/* We avoid low common keys in module aliases so they don't get huge. */
#define KEY_MIN_INTERESTING	KEY_MUTE
const KEY_MAX: u8 = 0x2ff;
const KEY_CNT: u8 = (KEY_MAX+1);

/*
 * Relative axes
 */

const REL_X: u8 = 0x00;
const REL_Y: u8 = 0x01;
const REL_Z: u8 = 0x02;
const REL_RX: u8 = 0x03;
const REL_RY: u8 = 0x04;
const REL_RZ: u8 = 0x05;
const REL_HWHEEL: u8 = 0x06;
const REL_DIAL: u8 = 0x07;
const REL_WHEEL: u8 = 0x08;
const REL_MISC: u8 = 0x09;
/*
 * 0x0a is reserved and should not be used in input drivers.
 * It was used by HID as REL_MISC+1 and userspace needs to detect if
 * the next REL_* event is correct or is just REL_MISC + n.
 * We define here REL_RESERVED so userspace can rely on it and detect
 * the situation described above.
 */
const REL_RESERVED: u8 = 0x0a;
const REL_WHEEL_HI_RES: u8 = 0x0b;
const REL_HWHEEL_HI_RES: u8 = 0x0c;
const REL_MAX: u8 = 0x0f;
const REL_CNT: u8 = (REL_MAX+1);

/*
 * Absolute axes
 */

const ABS_X: u8 = 0x00;
const ABS_Y: u8 = 0x01;
const ABS_Z: u8 = 0x02;
const ABS_RX: u8 = 0x03;
const ABS_RY: u8 = 0x04;
const ABS_RZ: u8 = 0x05;
const ABS_THROTTLE: u8 = 0x06;
const ABS_RUDDER: u8 = 0x07;
const ABS_WHEEL: u8 = 0x08;
const ABS_GAS: u8 = 0x09;
const ABS_BRAKE: u8 = 0x0a;
const ABS_HAT0X: u8 = 0x10;
const ABS_HAT0Y: u8 = 0x11;
const ABS_HAT1X: u8 = 0x12;
const ABS_HAT1Y: u8 = 0x13;
const ABS_HAT2X: u8 = 0x14;
const ABS_HAT2Y: u8 = 0x15;
const ABS_HAT3X: u8 = 0x16;
const ABS_HAT3Y: u8 = 0x17;
const ABS_PRESSURE: u8 = 0x18;
const ABS_DISTANCE: u8 = 0x19;
const ABS_TILT_X: u8 = 0x1a;
const ABS_TILT_Y: u8 = 0x1b;
const ABS_TOOL_WIDTH: u8 = 0x1c;

const ABS_VOLUME: u8 = 0x20;
const ABS_PROFILE: u8 = 0x21;

const ABS_MISC: u8 = 0x28;

/*
 * 0x2e is reserved and should not be used in input drivers.
 * It was used by HID as ABS_MISC+6 and userspace needs to detect if
 * the next ABS_* event is correct or is just ABS_MISC + n.
 * We define here ABS_RESERVED so userspace can rely on it and detect
 * the situation described above.
 */
const ABS_RESERVED: u8 = 0x2e;

const ABS_MT_SLOT: u8 = 0x2f;	/* MT slot being modified */
const ABS_MT_TOUCH_MAJOR: u8 = 0x30;	/* Major axis of touching ellipse */
const ABS_MT_TOUCH_MINOR: u8 = 0x31;	/* Minor axis (omit if circular) */
const ABS_MT_WIDTH_MAJOR: u8 = 0x32;	/* Major axis of approaching ellipse */
const ABS_MT_WIDTH_MINOR: u8 = 0x33;	/* Minor axis (omit if circular) */
const ABS_MT_ORIENTATION: u8 = 0x34;	/* Ellipse orientation */
const ABS_MT_POSITION_X: u8 = 0x35;	/* Center X touch position */
const ABS_MT_POSITION_Y: u8 = 0x36;	/* Center Y touch position */
const ABS_MT_TOOL_TYPE: u8 = 0x37;	/* Type of touching device */
const ABS_MT_BLOB_ID: u8 = 0x38;	/* Group a set of packets as a blob */
const ABS_MT_TRACKING_ID: u8 = 0x39;	/* Unique ID of initiated contact */
const ABS_MT_PRESSURE: u8 = 0x3a;	/* Pressure on contact area */
const ABS_MT_DISTANCE: u8 = 0x3b;	/* Contact hover distance */
const ABS_MT_TOOL_X: u8 = 0x3c;	/* Center X tool position */
const ABS_MT_TOOL_Y: u8 = 0x3d;	/* Center Y tool position */


const ABS_MAX: u8 = 0x3f;
const ABS_CNT: u8 = (ABS_MAX+1);

/*
 * Switch events
 */

const SW_LID: u8 = 0x00;  /* set = lid shut */
const SW_TABLET_MODE: u8 = 0x01;  /* set = tablet mode */
const SW_HEADPHONE_INSERT: u8 = 0x02;  /* set = inserted */
const SW_RFKILL_ALL: u8 = 0x03;  /* rfkill master switch, type "any"
					 set = radio enabled */
#define SW_RADIO		SW_RFKILL_ALL	/* deprecated */
const SW_MICROPHONE_INSERT: u8 = 0x04;  /* set = inserted */
const SW_DOCK: u8 = 0x05;  /* set = plugged into dock */
const SW_LINEOUT_INSERT: u8 = 0x06;  /* set = inserted */
const SW_JACK_PHYSICAL_INSERT: u8 = 0x07;  /* set = mechanical switch set */
const SW_VIDEOOUT_INSERT: u8 = 0x08;  /* set = inserted */
const SW_CAMERA_LENS_COVER: u8 = 0x09;  /* set = lens covered */
const SW_KEYPAD_SLIDE: u8 = 0x0a;  /* set = keypad slide out */
const SW_FRONT_PROXIMITY: u8 = 0x0b;  /* set = front proximity sensor active */
const SW_ROTATE_LOCK: u8 = 0x0c;  /* set = rotate locked/disabled */
const SW_LINEIN_INSERT: u8 = 0x0d;  /* set = inserted */
const SW_MUTE_DEVICE: u8 = 0x0e;  /* set = device disabled */
const SW_PEN_INSERTED: u8 = 0x0f;  /* set = pen inserted */
const SW_MACHINE_COVER: u8 = 0x10;  /* set = cover closed */
const SW_MAX: u8 = 0x10;
const SW_CNT: u8 = (SW_MAX+1);

/*
 * Misc events
 */

const MSC_SERIAL: u8 = 0x00;
const MSC_PULSELED: u8 = 0x01;
const MSC_GESTURE: u8 = 0x02;
const MSC_RAW: u8 = 0x03;
const MSC_SCAN: u8 = 0x04;
const MSC_TIMESTAMP: u8 = 0x05;
const MSC_MAX: u8 = 0x07;
const MSC_CNT: u8 = (MSC_MAX+1);

/*
 * LEDs
 */

const LED_NUML: u8 = 0x00;
const LED_CAPSL: u8 = 0x01;
const LED_SCROLLL: u8 = 0x02;
const LED_COMPOSE: u8 = 0x03;
const LED_KANA: u8 = 0x04;
const LED_SLEEP: u8 = 0x05;
const LED_SUSPEND: u8 = 0x06;
const LED_MUTE: u8 = 0x07;
const LED_MISC: u8 = 0x08;
const LED_MAIL: u8 = 0x09;
const LED_CHARGING: u8 = 0x0a;
const LED_MAX: u8 = 0x0f;
const LED_CNT: u8 = (LED_MAX+1);

/*
 * Autorepeat values
 */

const REP_DELAY: u8 = 0x00;
const REP_PERIOD: u8 = 0x01;
const REP_MAX: u8 = 0x01;
const REP_CNT: u8 = (REP_MAX+1);

/*
 * Sounds
 */

const SND_CLICK: u8 = 0x00;
const SND_BELL: u8 = 0x01;
const SND_TONE: u8 = 0x02;
const SND_MAX: u8 = 0x07;
const SND_CNT: u8 = (SND_MAX+1);

#endif
