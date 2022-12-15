PROVIDE(_stext = ORIGIN(REGION_TEXT));
PROVIDE(_stack_start = ORIGIN(REGION_STACK) + LENGTH(REGION_STACK));
PROVIDE(_max_hart_id = 0);
PROVIDE(_hart_stack_size = 2K);
PROVIDE(_heap_size = 0);

PROVIDE(UserSoft = DefaultHandler);
PROVIDE(SupervisorSoft = DefaultHandler);
PROVIDE(MachineSoft = DefaultHandler);
PROVIDE(UserTimer = DefaultHandler);
PROVIDE(SupervisorTimer = DefaultHandler);
PROVIDE(MachineTimer = DefaultHandler);
PROVIDE(UserExternal = DefaultHandler);
PROVIDE(SupervisorExternal = DefaultHandler);
PROVIDE(MachineExternal = DefaultHandler);

PROVIDE(DefaultHandler = DefaultInterruptHandler);
PROVIDE(ExceptionHandler = DefaultExceptionHandler);

/* # Pre-initialization function */
/* If the user overrides this using the `#[pre_init]` attribute or by creating a `__pre_init` function,
   then the function this points to will be called before the RAM is initialized. */
PROVIDE(__pre_init = default_pre_init);

/* A PAC/HAL defined routine that should initialize custom interrupt controller if needed. */
PROVIDE(_setup_interrupts = default_setup_interrupts);

/* # Multi-processing hook function
   fn _mp_hook() -> bool;

   This function is called from all the harts and must return true only for one hart,
   which will perform memory initialization. For other harts it must return false
   and implement wake-up in platform-dependent way (e.g. after waiting for a user interrupt).
*/
PROVIDE(_mp_hook = default_mp_hook);

/* # Start trap function override
  By default uses the riscv crates default trap handler
  but by providing the `_start_trap` symbol external crates can override.
*/
PROVIDE(_start_trap = default_start_trap);

SECTIONS
{
  .text.dummy (NOLOAD) :
  {
    /* This section is intended to make _stext address work */
    . = ABSOLUTE(_stext);
  } > REGION_TEXT

  .text _stext :
  {
    /* Put reset handler first in .text section so it ends up as the entry */
    /* point of the program. */
    KEEP(*(.init));
    KEEP(*(.init.rust));
    . = ALIGN(4);
    *(.trap);
    *(.trap.rust);

    #if defined(CLIC)
    . = ALIGN(64);
    KEEP(*(.text.interrupt_vector));
    #endif

    *(.text .text.*);


  } > REGION_TEXT

  .rodata : ALIGN(4)
  {
    *(.srodata .srodata.*);
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
  } > REGION_RODATA

  .data : ALIGN(4)
  {
    _sidata = LOADADDR(.data);
    _sdata = .;
    /* Must be called __global_pointer$ for linker relaxations to work. */
    PROVIDE(__global_pointer$ = . + 0x800);
    *(.sdata .sdata.* .sdata2 .sdata2.*);
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > REGION_DATA AT > REGION_RODATA

  .bss (NOLOAD) :
  {
    _sbss = .;
    *(.sbss .sbss.* .bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > REGION_BSS

  /* fictitious region that represents the memory available for the heap */
  .heap (NOLOAD) :
  {
    _sheap = .;
    . += _heap_size;
    . = ALIGN(4);
    _eheap = .;
  } > REGION_HEAP

  /* fictitious region that represents the memory available for the stack */
  .stack (NOLOAD) :
  {
    _estack = .;
    . = ABSOLUTE(_stack_start);
    _sstack = .;
  } > REGION_STACK

  /* fake output .got section */
  /* Dynamic relocations are unsupported. This section is only used to detect
     relocatable code in the input files and raise an error if relocatable code
     is found */
  .got (INFO) :
  {
    KEEP(*(.got .got.*));
  }

  .eh_frame (INFO) : { KEEP(*(.eh_frame)) }
  .eh_frame_hdr (INFO) : { *(.eh_frame_hdr) }
}

/* Do not exceed this mark in the error messages above                                    | */
ASSERT(ORIGIN(REGION_TEXT) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_TEXT must be 4-byte aligned");

ASSERT(ORIGIN(REGION_RODATA) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_RODATA must be 4-byte aligned");

ASSERT(ORIGIN(REGION_DATA) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_DATA must be 4-byte aligned");

ASSERT(ORIGIN(REGION_HEAP) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_HEAP must be 4-byte aligned");

ASSERT(ORIGIN(REGION_TEXT) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_TEXT must be 4-byte aligned");

ASSERT(ORIGIN(REGION_STACK) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_STACK must be 4-byte aligned");

ASSERT(_stext % 4 == 0, "
ERROR(riscv-rt): `_stext` must be 4-byte aligned");

ASSERT(_sdata % 4 == 0 && _edata % 4 == 0, "
BUG(riscv-rt): .data is not 4-byte aligned");

ASSERT(_sidata % 4 == 0, "
BUG(riscv-rt): the LMA of .data is not 4-byte aligned");

ASSERT(_sbss % 4 == 0 && _ebss % 4 == 0, "
BUG(riscv-rt): .bss is not 4-byte aligned");

ASSERT(_sheap % 4 == 0, "
BUG(riscv-rt): start of .heap is not 4-byte aligned");

ASSERT(_stext + SIZEOF(.text) < ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT), "
ERROR(riscv-rt): The .text section must be placed inside the REGION_TEXT region.
Set _stext to an address smaller than 'ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT)'");

ASSERT(SIZEOF(.stack) > (_max_hart_id + 1) * _hart_stack_size, "
ERROR(riscv-rt): .stack section is too small for allocating stacks for all the harts.
Consider changing `_max_hart_id` or `_hart_stack_size`.");

ASSERT(SIZEOF(.got) == 0, "
.got section detected in the input files. Dynamic relocations are not
supported. If you are linking to C code compiled using the `gcc` crate
then modify your build script to compile the C code _without_ the
-fPIC flag. See the documentation of the `gcc::Config.fpic` method for
details.");

/* Do not exceed this mark in the error messages above                                    | */

/* default interrupt handlers */
PROVIDE(int_0 = DefaultHandler);
PROVIDE(int_1 = DefaultHandler);
PROVIDE(int_2 = DefaultHandler);
PROVIDE(int_3 = DefaultHandler);
PROVIDE(int_4 = DefaultHandler);
PROVIDE(int_5 = DefaultHandler);
PROVIDE(int_6 = DefaultHandler);
PROVIDE(int_7 = DefaultHandler);
PROVIDE(int_8 = DefaultHandler);
PROVIDE(int_9 = DefaultHandler);
PROVIDE(int_10 = DefaultHandler);
PROVIDE(int_11 = DefaultHandler);
PROVIDE(int_12 = DefaultHandler);
PROVIDE(int_13 = DefaultHandler);
PROVIDE(int_14 = DefaultHandler);
PROVIDE(int_15 = DefaultHandler);
PROVIDE(int_16 = DefaultHandler);
PROVIDE(int_17 = DefaultHandler);
PROVIDE(int_18 = DefaultHandler);
PROVIDE(int_19 = DefaultHandler);
PROVIDE(int_20 = DefaultHandler);
PROVIDE(int_21 = DefaultHandler);
PROVIDE(int_22 = DefaultHandler);
PROVIDE(int_23 = DefaultHandler);
PROVIDE(int_24 = DefaultHandler);
PROVIDE(int_25 = DefaultHandler);
PROVIDE(int_26 = DefaultHandler);
PROVIDE(int_27 = DefaultHandler);
PROVIDE(int_28 = DefaultHandler);
PROVIDE(int_29 = DefaultHandler);
PROVIDE(int_30 = DefaultHandler);
PROVIDE(int_31 = DefaultHandler);
PROVIDE(int_32 = DefaultHandler);
PROVIDE(int_33 = DefaultHandler);
PROVIDE(int_34 = DefaultHandler);
PROVIDE(int_35 = DefaultHandler);
PROVIDE(int_36 = DefaultHandler);
PROVIDE(int_37 = DefaultHandler);
PROVIDE(int_38 = DefaultHandler);
PROVIDE(int_39 = DefaultHandler);
PROVIDE(int_40 = DefaultHandler);
PROVIDE(int_41 = DefaultHandler);
PROVIDE(int_42 = DefaultHandler);
PROVIDE(int_43 = DefaultHandler);
PROVIDE(int_44 = DefaultHandler);
PROVIDE(int_45 = DefaultHandler);
PROVIDE(int_46 = DefaultHandler);
PROVIDE(int_47 = DefaultHandler);
PROVIDE(int_48 = DefaultHandler);
PROVIDE(int_49 = DefaultHandler);
PROVIDE(int_50 = DefaultHandler);
PROVIDE(int_51 = DefaultHandler);
PROVIDE(int_52 = DefaultHandler);
PROVIDE(int_53 = DefaultHandler);
PROVIDE(int_54 = DefaultHandler);
PROVIDE(int_55 = DefaultHandler);
PROVIDE(int_56 = DefaultHandler);
PROVIDE(int_57 = DefaultHandler);
PROVIDE(int_58 = DefaultHandler);
PROVIDE(int_59 = DefaultHandler);
PROVIDE(int_60 = DefaultHandler);
PROVIDE(int_61 = DefaultHandler);
PROVIDE(int_62 = DefaultHandler);
PROVIDE(int_63 = DefaultHandler);
PROVIDE(int_64 = DefaultHandler);
PROVIDE(int_65 = DefaultHandler);
PROVIDE(int_66 = DefaultHandler);
PROVIDE(int_67 = DefaultHandler);
PROVIDE(int_68 = DefaultHandler);
PROVIDE(int_69 = DefaultHandler);
PROVIDE(int_70 = DefaultHandler);
PROVIDE(int_71 = DefaultHandler);
PROVIDE(int_72 = DefaultHandler);
PROVIDE(int_73 = DefaultHandler);
PROVIDE(int_74 = DefaultHandler);
PROVIDE(int_75 = DefaultHandler);
PROVIDE(int_76 = DefaultHandler);
PROVIDE(int_77 = DefaultHandler);
PROVIDE(int_78 = DefaultHandler);
PROVIDE(int_79 = DefaultHandler);
PROVIDE(int_80 = DefaultHandler);
PROVIDE(int_81 = DefaultHandler);
PROVIDE(int_82 = DefaultHandler);
PROVIDE(int_83 = DefaultHandler);
PROVIDE(int_84 = DefaultHandler);
PROVIDE(int_85 = DefaultHandler);
PROVIDE(int_86 = DefaultHandler);
PROVIDE(int_87 = DefaultHandler);
PROVIDE(int_88 = DefaultHandler);
PROVIDE(int_89 = DefaultHandler);
PROVIDE(int_90 = DefaultHandler);
PROVIDE(int_91 = DefaultHandler);
PROVIDE(int_92 = DefaultHandler);
PROVIDE(int_93 = DefaultHandler);
PROVIDE(int_94 = DefaultHandler);
PROVIDE(int_95 = DefaultHandler);
PROVIDE(int_96 = DefaultHandler);
PROVIDE(int_97 = DefaultHandler);
PROVIDE(int_98 = DefaultHandler);
PROVIDE(int_99 = DefaultHandler);
PROVIDE(int_100 = DefaultHandler);
PROVIDE(int_101 = DefaultHandler);
PROVIDE(int_102 = DefaultHandler);
PROVIDE(int_103 = DefaultHandler);
PROVIDE(int_104 = DefaultHandler);
PROVIDE(int_105 = DefaultHandler);
PROVIDE(int_106 = DefaultHandler);
PROVIDE(int_107 = DefaultHandler);
PROVIDE(int_108 = DefaultHandler);
PROVIDE(int_109 = DefaultHandler);
PROVIDE(int_110 = DefaultHandler);
PROVIDE(int_111 = DefaultHandler);
PROVIDE(int_112 = DefaultHandler);
PROVIDE(int_113 = DefaultHandler);
PROVIDE(int_114 = DefaultHandler);
PROVIDE(int_115 = DefaultHandler);
PROVIDE(int_116 = DefaultHandler);
PROVIDE(int_117 = DefaultHandler);
PROVIDE(int_118 = DefaultHandler);
PROVIDE(int_119 = DefaultHandler);
PROVIDE(int_120 = DefaultHandler);
PROVIDE(int_121 = DefaultHandler);
PROVIDE(int_122 = DefaultHandler);
PROVIDE(int_123 = DefaultHandler);
PROVIDE(int_124 = DefaultHandler);
PROVIDE(int_125 = DefaultHandler);
PROVIDE(int_126 = DefaultHandler);
PROVIDE(int_127 = DefaultHandler);
PROVIDE(int_128 = DefaultHandler);
PROVIDE(int_129 = DefaultHandler);
PROVIDE(int_130 = DefaultHandler);
PROVIDE(int_131 = DefaultHandler);
PROVIDE(int_132 = DefaultHandler);
PROVIDE(int_133 = DefaultHandler);
PROVIDE(int_134 = DefaultHandler);
PROVIDE(int_135 = DefaultHandler);
PROVIDE(int_136 = DefaultHandler);
PROVIDE(int_137 = DefaultHandler);
PROVIDE(int_138 = DefaultHandler);
PROVIDE(int_139 = DefaultHandler);
PROVIDE(int_140 = DefaultHandler);
PROVIDE(int_141 = DefaultHandler);
PROVIDE(int_142 = DefaultHandler);
PROVIDE(int_143 = DefaultHandler);
PROVIDE(int_144 = DefaultHandler);
PROVIDE(int_145 = DefaultHandler);
PROVIDE(int_146 = DefaultHandler);
PROVIDE(int_147 = DefaultHandler);
PROVIDE(int_148 = DefaultHandler);
PROVIDE(int_149 = DefaultHandler);
PROVIDE(int_150 = DefaultHandler);
PROVIDE(int_151 = DefaultHandler);
PROVIDE(int_152 = DefaultHandler);
PROVIDE(int_153 = DefaultHandler);
PROVIDE(int_154 = DefaultHandler);
PROVIDE(int_155 = DefaultHandler);
PROVIDE(int_156 = DefaultHandler);
PROVIDE(int_157 = DefaultHandler);
PROVIDE(int_158 = DefaultHandler);
PROVIDE(int_159 = DefaultHandler);
PROVIDE(int_160 = DefaultHandler);
PROVIDE(int_161 = DefaultHandler);
PROVIDE(int_162 = DefaultHandler);
PROVIDE(int_163 = DefaultHandler);
PROVIDE(int_164 = DefaultHandler);
PROVIDE(int_165 = DefaultHandler);
PROVIDE(int_166 = DefaultHandler);
PROVIDE(int_167 = DefaultHandler);
PROVIDE(int_168 = DefaultHandler);
PROVIDE(int_169 = DefaultHandler);
PROVIDE(int_170 = DefaultHandler);
PROVIDE(int_171 = DefaultHandler);
PROVIDE(int_172 = DefaultHandler);
PROVIDE(int_173 = DefaultHandler);
PROVIDE(int_174 = DefaultHandler);
PROVIDE(int_175 = DefaultHandler);
PROVIDE(int_176 = DefaultHandler);
PROVIDE(int_177 = DefaultHandler);
PROVIDE(int_178 = DefaultHandler);
PROVIDE(int_179 = DefaultHandler);
PROVIDE(int_180 = DefaultHandler);
PROVIDE(int_181 = DefaultHandler);
PROVIDE(int_182 = DefaultHandler);
PROVIDE(int_183 = DefaultHandler);
PROVIDE(int_184 = DefaultHandler);
PROVIDE(int_185 = DefaultHandler);
PROVIDE(int_186 = DefaultHandler);
PROVIDE(int_187 = DefaultHandler);
PROVIDE(int_188 = DefaultHandler);
PROVIDE(int_189 = DefaultHandler);
PROVIDE(int_190 = DefaultHandler);
PROVIDE(int_191 = DefaultHandler);
PROVIDE(int_192 = DefaultHandler);
PROVIDE(int_193 = DefaultHandler);
PROVIDE(int_194 = DefaultHandler);
PROVIDE(int_195 = DefaultHandler);
PROVIDE(int_196 = DefaultHandler);
PROVIDE(int_197 = DefaultHandler);
PROVIDE(int_198 = DefaultHandler);
PROVIDE(int_199 = DefaultHandler);
PROVIDE(int_200 = DefaultHandler);
PROVIDE(int_201 = DefaultHandler);
PROVIDE(int_202 = DefaultHandler);
PROVIDE(int_203 = DefaultHandler);
PROVIDE(int_204 = DefaultHandler);
PROVIDE(int_205 = DefaultHandler);
PROVIDE(int_206 = DefaultHandler);
PROVIDE(int_207 = DefaultHandler);
PROVIDE(int_208 = DefaultHandler);
PROVIDE(int_209 = DefaultHandler);
PROVIDE(int_210 = DefaultHandler);
PROVIDE(int_211 = DefaultHandler);
PROVIDE(int_212 = DefaultHandler);
PROVIDE(int_213 = DefaultHandler);
PROVIDE(int_214 = DefaultHandler);
PROVIDE(int_215 = DefaultHandler);
PROVIDE(int_216 = DefaultHandler);
PROVIDE(int_217 = DefaultHandler);
PROVIDE(int_218 = DefaultHandler);
PROVIDE(int_219 = DefaultHandler);
PROVIDE(int_220 = DefaultHandler);
PROVIDE(int_221 = DefaultHandler);
PROVIDE(int_222 = DefaultHandler);
PROVIDE(int_223 = DefaultHandler);
PROVIDE(int_224 = DefaultHandler);
PROVIDE(int_225 = DefaultHandler);
PROVIDE(int_226 = DefaultHandler);
PROVIDE(int_227 = DefaultHandler);
PROVIDE(int_228 = DefaultHandler);
PROVIDE(int_229 = DefaultHandler);
PROVIDE(int_230 = DefaultHandler);
PROVIDE(int_231 = DefaultHandler);
PROVIDE(int_232 = DefaultHandler);
PROVIDE(int_233 = DefaultHandler);
PROVIDE(int_234 = DefaultHandler);
PROVIDE(int_235 = DefaultHandler);
PROVIDE(int_236 = DefaultHandler);
PROVIDE(int_237 = DefaultHandler);
PROVIDE(int_238 = DefaultHandler);
PROVIDE(int_239 = DefaultHandler);
PROVIDE(int_240 = DefaultHandler);
PROVIDE(int_241 = DefaultHandler);
PROVIDE(int_242 = DefaultHandler);
PROVIDE(int_243 = DefaultHandler);
PROVIDE(int_244 = DefaultHandler);
PROVIDE(int_245 = DefaultHandler);
PROVIDE(int_246 = DefaultHandler);
PROVIDE(int_247 = DefaultHandler);
PROVIDE(int_248 = DefaultHandler);
PROVIDE(int_249 = DefaultHandler);
PROVIDE(int_250 = DefaultHandler);
PROVIDE(int_251 = DefaultHandler);
PROVIDE(int_252 = DefaultHandler);
PROVIDE(int_253 = DefaultHandler);
PROVIDE(int_254 = DefaultHandler);
PROVIDE(int_255 = DefaultHandler);
PROVIDE(int_256 = DefaultHandler);
PROVIDE(int_257 = DefaultHandler);
PROVIDE(int_258 = DefaultHandler);
PROVIDE(int_259 = DefaultHandler);
PROVIDE(int_260 = DefaultHandler);
PROVIDE(int_261 = DefaultHandler);
PROVIDE(int_262 = DefaultHandler);
PROVIDE(int_263 = DefaultHandler);
PROVIDE(int_264 = DefaultHandler);
