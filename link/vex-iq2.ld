/*
SPDX-FileCopyrightText: 2026 Gabe Schrock
SPDX-License-Identifier: MIT
*/

MEMORY { USER_RAM (rwx) : ORIGIN = 0x20010000, LENGTH = 0x18000 }
PROVIDE(__vcodesig_magic = 0x32515658);
PROVIDE(__heap_length  = 0x1000);
PROVIDE(__stack_length = 0x800);
INCLUDE vex-common.ld
