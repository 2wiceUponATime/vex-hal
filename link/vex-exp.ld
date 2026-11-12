MEMORY { USER_RAM (rwx) : ORIGIN = 0x202C0000, LENGTH = 0x80000 }
PROVIDE(__vcodesig_magic = 0x45585658);
PROVIDE(__heap_length  = 0x8000);
PROVIDE(__stack_length = 0x1000);
INCLUDE vex-common.ld
