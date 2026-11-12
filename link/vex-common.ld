ENTRY(_boot)

PROVIDE(__vcodesig_type    = 0);
PROVIDE(__vcodesig_owner   = 2);
PROVIDE(__vcodesig_options = 0);

SECTIONS {
  .code_signature : {
    LONG(__vcodesig_magic) LONG(__vcodesig_type)
    LONG(__vcodesig_owner) LONG(__vcodesig_options)
    FILL(0)
    . = ORIGIN(USER_RAM) + 0x20;
  } > USER_RAM

  .text : { KEEP(*(.boot)) *(.text .text.*) } > USER_RAM
  .rodata : { *(.rodata .rodata1 .rodata.*) } > USER_RAM
  .ARM.extab : { *(.ARM.extab* .gcc_except_table*) } > USER_RAM
  .ARM.exidx : {
    __exidx_start = .; *(.ARM.exidx*); __exidx_end = .;
  } > USER_RAM
  .data : { *(.data .data1 .data.*) } > USER_RAM

  .bss (NOLOAD) : ALIGN(4) {
    __bss_start = .; *(.bss .bss.* COMMON); . = ALIGN(4); __bss_end = .;
  } > USER_RAM
  .heap (NOLOAD) : ALIGN(8) {
    __heap_start = .; . += __heap_length; __heap_end = .;
  } > USER_RAM
  .stack (NOLOAD) : ALIGN(8) {
    __stack_bottom = .; . += __stack_length; __stack_top = .;
  } > USER_RAM

  /DISCARD/ : { *(.ARM.attributes*) }
}
