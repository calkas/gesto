MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K

  /* .bss, .data and the heap go in this region */
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K

  /* Core coupled (faster) RAM dedicated to hold the stack */
  CCRAM : ORIGIN = 0x10000000, LENGTH = 64K
}


/* Linker script for the STM32F407 */
/* See https://docs.rs/cortex-m-rt/latest/cortex_m_rt/ */

/*
    ---------------------------
    |      MEMORY LAYOUT      |
    ---------------------------
    | SRAM                    |
    | 0x20000000 - 0x2001FFFF |
    ---------------------------
    | CCM data RAM - STACK    |
    | 0x10000000 - 0x1000FFFF |
    ---------------------------
    | FLASH                   |
    | 0x08000000 - 0x080FFFFF |
    ---------------------------
*/
