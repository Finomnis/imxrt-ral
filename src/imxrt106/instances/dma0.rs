#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::dma0::Instance;
pub use crate::imxrt106::peripherals::dma0::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::dma0::{
    CDNE, CEEI, CERQ, CERR, CINT, CR, DCHPRI0, DCHPRI1, DCHPRI10, DCHPRI11, DCHPRI12, DCHPRI13,
    DCHPRI14, DCHPRI15, DCHPRI16, DCHPRI17, DCHPRI18, DCHPRI19, DCHPRI2, DCHPRI20, DCHPRI21,
    DCHPRI22, DCHPRI23, DCHPRI24, DCHPRI25, DCHPRI26, DCHPRI27, DCHPRI28, DCHPRI29, DCHPRI3,
    DCHPRI30, DCHPRI31, DCHPRI4, DCHPRI5, DCHPRI6, DCHPRI7, DCHPRI8, DCHPRI9, EARS, EEI, ERQ, ERR,
    ES, HRS, INT, SEEI, SERQ, SSRT, TCD_ATTR0, TCD_ATTR1, TCD_ATTR10, TCD_ATTR11, TCD_ATTR12,
    TCD_ATTR13, TCD_ATTR14, TCD_ATTR15, TCD_ATTR16, TCD_ATTR17, TCD_ATTR18, TCD_ATTR19, TCD_ATTR2,
    TCD_ATTR20, TCD_ATTR21, TCD_ATTR22, TCD_ATTR23, TCD_ATTR24, TCD_ATTR25, TCD_ATTR26, TCD_ATTR27,
    TCD_ATTR28, TCD_ATTR29, TCD_ATTR3, TCD_ATTR30, TCD_ATTR31, TCD_ATTR4, TCD_ATTR5, TCD_ATTR6,
    TCD_ATTR7, TCD_ATTR8, TCD_ATTR9, TCD_BITER_ELINK, TCD_BITER_ELINKNO1, TCD_BITER_ELINKNO10,
    TCD_BITER_ELINKNO11, TCD_BITER_ELINKNO12, TCD_BITER_ELINKNO13, TCD_BITER_ELINKNO14,
    TCD_BITER_ELINKNO15, TCD_BITER_ELINKNO16, TCD_BITER_ELINKNO17, TCD_BITER_ELINKNO18,
    TCD_BITER_ELINKNO19, TCD_BITER_ELINKNO2, TCD_BITER_ELINKNO20, TCD_BITER_ELINKNO21,
    TCD_BITER_ELINKNO22, TCD_BITER_ELINKNO23, TCD_BITER_ELINKNO24, TCD_BITER_ELINKNO25,
    TCD_BITER_ELINKNO26, TCD_BITER_ELINKNO27, TCD_BITER_ELINKNO28, TCD_BITER_ELINKNO29,
    TCD_BITER_ELINKNO3, TCD_BITER_ELINKNO30, TCD_BITER_ELINKNO31, TCD_BITER_ELINKNO4,
    TCD_BITER_ELINKNO5, TCD_BITER_ELINKNO6, TCD_BITER_ELINKNO7, TCD_BITER_ELINKNO8,
    TCD_BITER_ELINKNO9, TCD_CITER_ELINK, TCD_CITER_ELINKNO1, TCD_CITER_ELINKNO10,
    TCD_CITER_ELINKNO11, TCD_CITER_ELINKNO12, TCD_CITER_ELINKNO13, TCD_CITER_ELINKNO14,
    TCD_CITER_ELINKNO15, TCD_CITER_ELINKNO16, TCD_CITER_ELINKNO17, TCD_CITER_ELINKNO18,
    TCD_CITER_ELINKNO19, TCD_CITER_ELINKNO2, TCD_CITER_ELINKNO20, TCD_CITER_ELINKNO21,
    TCD_CITER_ELINKNO22, TCD_CITER_ELINKNO23, TCD_CITER_ELINKNO24, TCD_CITER_ELINKNO25,
    TCD_CITER_ELINKNO26, TCD_CITER_ELINKNO27, TCD_CITER_ELINKNO28, TCD_CITER_ELINKNO29,
    TCD_CITER_ELINKNO3, TCD_CITER_ELINKNO30, TCD_CITER_ELINKNO31, TCD_CITER_ELINKNO4,
    TCD_CITER_ELINKNO5, TCD_CITER_ELINKNO6, TCD_CITER_ELINKNO7, TCD_CITER_ELINKNO8,
    TCD_CITER_ELINKNO9, TCD_CSR0, TCD_CSR1, TCD_CSR10, TCD_CSR11, TCD_CSR12, TCD_CSR13, TCD_CSR14,
    TCD_CSR15, TCD_CSR16, TCD_CSR17, TCD_CSR18, TCD_CSR19, TCD_CSR2, TCD_CSR20, TCD_CSR21,
    TCD_CSR22, TCD_CSR23, TCD_CSR24, TCD_CSR25, TCD_CSR26, TCD_CSR27, TCD_CSR28, TCD_CSR29,
    TCD_CSR3, TCD_CSR30, TCD_CSR31, TCD_CSR4, TCD_CSR5, TCD_CSR6, TCD_CSR7, TCD_CSR8, TCD_CSR9,
    TCD_DADDR0, TCD_DADDR1, TCD_DADDR10, TCD_DADDR11, TCD_DADDR12, TCD_DADDR13, TCD_DADDR14,
    TCD_DADDR15, TCD_DADDR16, TCD_DADDR17, TCD_DADDR18, TCD_DADDR19, TCD_DADDR2, TCD_DADDR20,
    TCD_DADDR21, TCD_DADDR22, TCD_DADDR23, TCD_DADDR24, TCD_DADDR25, TCD_DADDR26, TCD_DADDR27,
    TCD_DADDR28, TCD_DADDR29, TCD_DADDR3, TCD_DADDR30, TCD_DADDR31, TCD_DADDR4, TCD_DADDR5,
    TCD_DADDR6, TCD_DADDR7, TCD_DADDR8, TCD_DADDR9, TCD_DLASTSGA0, TCD_DLASTSGA1, TCD_DLASTSGA10,
    TCD_DLASTSGA11, TCD_DLASTSGA12, TCD_DLASTSGA13, TCD_DLASTSGA14, TCD_DLASTSGA15, TCD_DLASTSGA16,
    TCD_DLASTSGA17, TCD_DLASTSGA18, TCD_DLASTSGA19, TCD_DLASTSGA2, TCD_DLASTSGA20, TCD_DLASTSGA21,
    TCD_DLASTSGA22, TCD_DLASTSGA23, TCD_DLASTSGA24, TCD_DLASTSGA25, TCD_DLASTSGA26, TCD_DLASTSGA27,
    TCD_DLASTSGA28, TCD_DLASTSGA29, TCD_DLASTSGA3, TCD_DLASTSGA30, TCD_DLASTSGA31, TCD_DLASTSGA4,
    TCD_DLASTSGA5, TCD_DLASTSGA6, TCD_DLASTSGA7, TCD_DLASTSGA8, TCD_DLASTSGA9, TCD_DOFF0,
    TCD_DOFF1, TCD_DOFF10, TCD_DOFF11, TCD_DOFF12, TCD_DOFF13, TCD_DOFF14, TCD_DOFF15, TCD_DOFF16,
    TCD_DOFF17, TCD_DOFF18, TCD_DOFF19, TCD_DOFF2, TCD_DOFF20, TCD_DOFF21, TCD_DOFF22, TCD_DOFF23,
    TCD_DOFF24, TCD_DOFF25, TCD_DOFF26, TCD_DOFF27, TCD_DOFF28, TCD_DOFF29, TCD_DOFF3, TCD_DOFF30,
    TCD_DOFF31, TCD_DOFF4, TCD_DOFF5, TCD_DOFF6, TCD_DOFF7, TCD_DOFF8, TCD_DOFF9, TCD_NBYTES_ML,
    TCD_NBYTES_MLNO1, TCD_NBYTES_MLNO10, TCD_NBYTES_MLNO11, TCD_NBYTES_MLNO12, TCD_NBYTES_MLNO13,
    TCD_NBYTES_MLNO14, TCD_NBYTES_MLNO15, TCD_NBYTES_MLNO16, TCD_NBYTES_MLNO17, TCD_NBYTES_MLNO18,
    TCD_NBYTES_MLNO19, TCD_NBYTES_MLNO2, TCD_NBYTES_MLNO20, TCD_NBYTES_MLNO21, TCD_NBYTES_MLNO22,
    TCD_NBYTES_MLNO23, TCD_NBYTES_MLNO24, TCD_NBYTES_MLNO25, TCD_NBYTES_MLNO26, TCD_NBYTES_MLNO27,
    TCD_NBYTES_MLNO28, TCD_NBYTES_MLNO29, TCD_NBYTES_MLNO3, TCD_NBYTES_MLNO30, TCD_NBYTES_MLNO31,
    TCD_NBYTES_MLNO4, TCD_NBYTES_MLNO5, TCD_NBYTES_MLNO6, TCD_NBYTES_MLNO7, TCD_NBYTES_MLNO8,
    TCD_NBYTES_MLNO9, TCD_SADDR0, TCD_SADDR1, TCD_SADDR10, TCD_SADDR11, TCD_SADDR12, TCD_SADDR13,
    TCD_SADDR14, TCD_SADDR15, TCD_SADDR16, TCD_SADDR17, TCD_SADDR18, TCD_SADDR19, TCD_SADDR2,
    TCD_SADDR20, TCD_SADDR21, TCD_SADDR22, TCD_SADDR23, TCD_SADDR24, TCD_SADDR25, TCD_SADDR26,
    TCD_SADDR27, TCD_SADDR28, TCD_SADDR29, TCD_SADDR3, TCD_SADDR30, TCD_SADDR31, TCD_SADDR4,
    TCD_SADDR5, TCD_SADDR6, TCD_SADDR7, TCD_SADDR8, TCD_SADDR9, TCD_SLAST0, TCD_SLAST1,
    TCD_SLAST10, TCD_SLAST11, TCD_SLAST12, TCD_SLAST13, TCD_SLAST14, TCD_SLAST15, TCD_SLAST16,
    TCD_SLAST17, TCD_SLAST18, TCD_SLAST19, TCD_SLAST2, TCD_SLAST20, TCD_SLAST21, TCD_SLAST22,
    TCD_SLAST23, TCD_SLAST24, TCD_SLAST25, TCD_SLAST26, TCD_SLAST27, TCD_SLAST28, TCD_SLAST29,
    TCD_SLAST3, TCD_SLAST30, TCD_SLAST31, TCD_SLAST4, TCD_SLAST5, TCD_SLAST6, TCD_SLAST7,
    TCD_SLAST8, TCD_SLAST9, TCD_SOFF0, TCD_SOFF1, TCD_SOFF10, TCD_SOFF11, TCD_SOFF12, TCD_SOFF13,
    TCD_SOFF14, TCD_SOFF15, TCD_SOFF16, TCD_SOFF17, TCD_SOFF18, TCD_SOFF19, TCD_SOFF2, TCD_SOFF20,
    TCD_SOFF21, TCD_SOFF22, TCD_SOFF23, TCD_SOFF24, TCD_SOFF25, TCD_SOFF26, TCD_SOFF27, TCD_SOFF28,
    TCD_SOFF29, TCD_SOFF3, TCD_SOFF30, TCD_SOFF31, TCD_SOFF4, TCD_SOFF5, TCD_SOFF6, TCD_SOFF7,
    TCD_SOFF8, TCD_SOFF9,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The DMA0 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type DMA0 = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DMA0_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DMA0 peripheral instance
#[cfg(not(feature = "nosync"))]
impl DMA0 {
    const INSTANCE: Self = Self {
        addr: 0x400e8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::DMA0_DMA16,
            crate::interrupt::DMA1_DMA17,
            crate::interrupt::DMA2_DMA18,
            crate::interrupt::DMA3_DMA19,
            crate::interrupt::DMA4_DMA20,
            crate::interrupt::DMA5_DMA21,
            crate::interrupt::DMA6_DMA22,
            crate::interrupt::DMA7_DMA23,
            crate::interrupt::DMA8_DMA24,
            crate::interrupt::DMA9_DMA25,
            crate::interrupt::DMA10_DMA26,
            crate::interrupt::DMA11_DMA27,
            crate::interrupt::DMA12_DMA28,
            crate::interrupt::DMA13_DMA29,
            crate::interrupt::DMA14_DMA30,
            crate::interrupt::DMA15_DMA31,
            crate::interrupt::DMA_ERROR,
        ],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in DMA0
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000400,
        ES: 0x00000000,
        ERQ: 0x00000000,
        EEI: 0x00000000,
        CEEI: 0x00000000,
        SEEI: 0x00000000,
        CERQ: 0x00000000,
        SERQ: 0x00000000,
        CDNE: 0x00000000,
        SSRT: 0x00000000,
        CERR: 0x00000000,
        CINT: 0x00000000,
        INT: 0x00000000,
        ERR: 0x00000000,
        HRS: 0x00000000,
        EARS: 0x00000000,
        DCHPRI3: 0x00000003,
        DCHPRI2: 0x00000002,
        DCHPRI1: 0x00000001,
        DCHPRI0: 0x00000000,
        DCHPRI7: 0x00000007,
        DCHPRI6: 0x00000006,
        DCHPRI5: 0x00000005,
        DCHPRI4: 0x00000004,
        DCHPRI11: 0x0000000B,
        DCHPRI10: 0x0000000A,
        DCHPRI9: 0x00000009,
        DCHPRI8: 0x00000008,
        DCHPRI15: 0x0000000F,
        DCHPRI14: 0x0000000E,
        DCHPRI13: 0x0000000D,
        DCHPRI12: 0x0000000C,
        DCHPRI19: 0x00000013,
        DCHPRI18: 0x00000012,
        DCHPRI17: 0x00000011,
        DCHPRI16: 0x00000010,
        DCHPRI23: 0x00000017,
        DCHPRI22: 0x00000016,
        DCHPRI21: 0x00000015,
        DCHPRI20: 0x00000014,
        DCHPRI27: 0x0000001B,
        DCHPRI26: 0x0000001A,
        DCHPRI25: 0x00000019,
        DCHPRI24: 0x00000018,
        DCHPRI31: 0x0000001F,
        DCHPRI30: 0x0000001E,
        DCHPRI29: 0x0000001D,
        DCHPRI28: 0x0000001C,
        TCD_SADDR0: 0x00000000,
        TCD_SOFF0: 0x00000000,
        TCD_ATTR0: 0x00000000,
        TCD_NBYTES_ML: 0x00000000,
        TCD_SLAST0: 0x00000000,
        TCD_DADDR0: 0x00000000,
        TCD_DOFF0: 0x00000000,
        TCD_CITER_ELINK: 0x00000000,
        TCD_DLASTSGA0: 0x00000000,
        TCD_CSR0: 0x00000000,
        TCD_BITER_ELINK: 0x00000000,
        TCD_SADDR1: 0x00000000,
        TCD_SOFF1: 0x00000000,
        TCD_ATTR1: 0x00000000,
        TCD_NBYTES_MLNO1: 0x00000000,
        TCD_SLAST1: 0x00000000,
        TCD_DADDR1: 0x00000000,
        TCD_DOFF1: 0x00000000,
        TCD_CITER_ELINKNO1: 0x00000000,
        TCD_DLASTSGA1: 0x00000000,
        TCD_CSR1: 0x00000000,
        TCD_BITER_ELINKNO1: 0x00000000,
        TCD_SADDR2: 0x00000000,
        TCD_SOFF2: 0x00000000,
        TCD_ATTR2: 0x00000000,
        TCD_NBYTES_MLNO2: 0x00000000,
        TCD_SLAST2: 0x00000000,
        TCD_DADDR2: 0x00000000,
        TCD_DOFF2: 0x00000000,
        TCD_CITER_ELINKNO2: 0x00000000,
        TCD_DLASTSGA2: 0x00000000,
        TCD_CSR2: 0x00000000,
        TCD_BITER_ELINKNO2: 0x00000000,
        TCD_SADDR3: 0x00000000,
        TCD_SOFF3: 0x00000000,
        TCD_ATTR3: 0x00000000,
        TCD_NBYTES_MLNO3: 0x00000000,
        TCD_SLAST3: 0x00000000,
        TCD_DADDR3: 0x00000000,
        TCD_DOFF3: 0x00000000,
        TCD_CITER_ELINKNO3: 0x00000000,
        TCD_DLASTSGA3: 0x00000000,
        TCD_CSR3: 0x00000000,
        TCD_BITER_ELINKNO3: 0x00000000,
        TCD_SADDR4: 0x00000000,
        TCD_SOFF4: 0x00000000,
        TCD_ATTR4: 0x00000000,
        TCD_NBYTES_MLNO4: 0x00000000,
        TCD_SLAST4: 0x00000000,
        TCD_DADDR4: 0x00000000,
        TCD_DOFF4: 0x00000000,
        TCD_CITER_ELINKNO4: 0x00000000,
        TCD_DLASTSGA4: 0x00000000,
        TCD_CSR4: 0x00000000,
        TCD_BITER_ELINKNO4: 0x00000000,
        TCD_SADDR5: 0x00000000,
        TCD_SOFF5: 0x00000000,
        TCD_ATTR5: 0x00000000,
        TCD_NBYTES_MLNO5: 0x00000000,
        TCD_SLAST5: 0x00000000,
        TCD_DADDR5: 0x00000000,
        TCD_DOFF5: 0x00000000,
        TCD_CITER_ELINKNO5: 0x00000000,
        TCD_DLASTSGA5: 0x00000000,
        TCD_CSR5: 0x00000000,
        TCD_BITER_ELINKNO5: 0x00000000,
        TCD_SADDR6: 0x00000000,
        TCD_SOFF6: 0x00000000,
        TCD_ATTR6: 0x00000000,
        TCD_NBYTES_MLNO6: 0x00000000,
        TCD_SLAST6: 0x00000000,
        TCD_DADDR6: 0x00000000,
        TCD_DOFF6: 0x00000000,
        TCD_CITER_ELINKNO6: 0x00000000,
        TCD_DLASTSGA6: 0x00000000,
        TCD_CSR6: 0x00000000,
        TCD_BITER_ELINKNO6: 0x00000000,
        TCD_SADDR7: 0x00000000,
        TCD_SOFF7: 0x00000000,
        TCD_ATTR7: 0x00000000,
        TCD_NBYTES_MLNO7: 0x00000000,
        TCD_SLAST7: 0x00000000,
        TCD_DADDR7: 0x00000000,
        TCD_DOFF7: 0x00000000,
        TCD_CITER_ELINKNO7: 0x00000000,
        TCD_DLASTSGA7: 0x00000000,
        TCD_CSR7: 0x00000000,
        TCD_BITER_ELINKNO7: 0x00000000,
        TCD_SADDR8: 0x00000000,
        TCD_SOFF8: 0x00000000,
        TCD_ATTR8: 0x00000000,
        TCD_NBYTES_MLNO8: 0x00000000,
        TCD_SLAST8: 0x00000000,
        TCD_DADDR8: 0x00000000,
        TCD_DOFF8: 0x00000000,
        TCD_CITER_ELINKNO8: 0x00000000,
        TCD_DLASTSGA8: 0x00000000,
        TCD_CSR8: 0x00000000,
        TCD_BITER_ELINKNO8: 0x00000000,
        TCD_SADDR9: 0x00000000,
        TCD_SOFF9: 0x00000000,
        TCD_ATTR9: 0x00000000,
        TCD_NBYTES_MLNO9: 0x00000000,
        TCD_SLAST9: 0x00000000,
        TCD_DADDR9: 0x00000000,
        TCD_DOFF9: 0x00000000,
        TCD_CITER_ELINKNO9: 0x00000000,
        TCD_DLASTSGA9: 0x00000000,
        TCD_CSR9: 0x00000000,
        TCD_BITER_ELINKNO9: 0x00000000,
        TCD_SADDR10: 0x00000000,
        TCD_SOFF10: 0x00000000,
        TCD_ATTR10: 0x00000000,
        TCD_NBYTES_MLNO10: 0x00000000,
        TCD_SLAST10: 0x00000000,
        TCD_DADDR10: 0x00000000,
        TCD_DOFF10: 0x00000000,
        TCD_CITER_ELINKNO10: 0x00000000,
        TCD_DLASTSGA10: 0x00000000,
        TCD_CSR10: 0x00000000,
        TCD_BITER_ELINKNO10: 0x00000000,
        TCD_SADDR11: 0x00000000,
        TCD_SOFF11: 0x00000000,
        TCD_ATTR11: 0x00000000,
        TCD_NBYTES_MLNO11: 0x00000000,
        TCD_SLAST11: 0x00000000,
        TCD_DADDR11: 0x00000000,
        TCD_DOFF11: 0x00000000,
        TCD_CITER_ELINKNO11: 0x00000000,
        TCD_DLASTSGA11: 0x00000000,
        TCD_CSR11: 0x00000000,
        TCD_BITER_ELINKNO11: 0x00000000,
        TCD_SADDR12: 0x00000000,
        TCD_SOFF12: 0x00000000,
        TCD_ATTR12: 0x00000000,
        TCD_NBYTES_MLNO12: 0x00000000,
        TCD_SLAST12: 0x00000000,
        TCD_DADDR12: 0x00000000,
        TCD_DOFF12: 0x00000000,
        TCD_CITER_ELINKNO12: 0x00000000,
        TCD_DLASTSGA12: 0x00000000,
        TCD_CSR12: 0x00000000,
        TCD_BITER_ELINKNO12: 0x00000000,
        TCD_SADDR13: 0x00000000,
        TCD_SOFF13: 0x00000000,
        TCD_ATTR13: 0x00000000,
        TCD_NBYTES_MLNO13: 0x00000000,
        TCD_SLAST13: 0x00000000,
        TCD_DADDR13: 0x00000000,
        TCD_DOFF13: 0x00000000,
        TCD_CITER_ELINKNO13: 0x00000000,
        TCD_DLASTSGA13: 0x00000000,
        TCD_CSR13: 0x00000000,
        TCD_BITER_ELINKNO13: 0x00000000,
        TCD_SADDR14: 0x00000000,
        TCD_SOFF14: 0x00000000,
        TCD_ATTR14: 0x00000000,
        TCD_NBYTES_MLNO14: 0x00000000,
        TCD_SLAST14: 0x00000000,
        TCD_DADDR14: 0x00000000,
        TCD_DOFF14: 0x00000000,
        TCD_CITER_ELINKNO14: 0x00000000,
        TCD_DLASTSGA14: 0x00000000,
        TCD_CSR14: 0x00000000,
        TCD_BITER_ELINKNO14: 0x00000000,
        TCD_SADDR15: 0x00000000,
        TCD_SOFF15: 0x00000000,
        TCD_ATTR15: 0x00000000,
        TCD_NBYTES_MLNO15: 0x00000000,
        TCD_SLAST15: 0x00000000,
        TCD_DADDR15: 0x00000000,
        TCD_DOFF15: 0x00000000,
        TCD_CITER_ELINKNO15: 0x00000000,
        TCD_DLASTSGA15: 0x00000000,
        TCD_CSR15: 0x00000000,
        TCD_BITER_ELINKNO15: 0x00000000,
        TCD_SADDR16: 0x00000000,
        TCD_SOFF16: 0x00000000,
        TCD_ATTR16: 0x00000000,
        TCD_NBYTES_MLNO16: 0x00000000,
        TCD_SLAST16: 0x00000000,
        TCD_DADDR16: 0x00000000,
        TCD_DOFF16: 0x00000000,
        TCD_CITER_ELINKNO16: 0x00000000,
        TCD_DLASTSGA16: 0x00000000,
        TCD_CSR16: 0x00000000,
        TCD_BITER_ELINKNO16: 0x00000000,
        TCD_SADDR17: 0x00000000,
        TCD_SOFF17: 0x00000000,
        TCD_ATTR17: 0x00000000,
        TCD_NBYTES_MLNO17: 0x00000000,
        TCD_SLAST17: 0x00000000,
        TCD_DADDR17: 0x00000000,
        TCD_DOFF17: 0x00000000,
        TCD_CITER_ELINKNO17: 0x00000000,
        TCD_DLASTSGA17: 0x00000000,
        TCD_CSR17: 0x00000000,
        TCD_BITER_ELINKNO17: 0x00000000,
        TCD_SADDR18: 0x00000000,
        TCD_SOFF18: 0x00000000,
        TCD_ATTR18: 0x00000000,
        TCD_NBYTES_MLNO18: 0x00000000,
        TCD_SLAST18: 0x00000000,
        TCD_DADDR18: 0x00000000,
        TCD_DOFF18: 0x00000000,
        TCD_CITER_ELINKNO18: 0x00000000,
        TCD_DLASTSGA18: 0x00000000,
        TCD_CSR18: 0x00000000,
        TCD_BITER_ELINKNO18: 0x00000000,
        TCD_SADDR19: 0x00000000,
        TCD_SOFF19: 0x00000000,
        TCD_ATTR19: 0x00000000,
        TCD_NBYTES_MLNO19: 0x00000000,
        TCD_SLAST19: 0x00000000,
        TCD_DADDR19: 0x00000000,
        TCD_DOFF19: 0x00000000,
        TCD_CITER_ELINKNO19: 0x00000000,
        TCD_DLASTSGA19: 0x00000000,
        TCD_CSR19: 0x00000000,
        TCD_BITER_ELINKNO19: 0x00000000,
        TCD_SADDR20: 0x00000000,
        TCD_SOFF20: 0x00000000,
        TCD_ATTR20: 0x00000000,
        TCD_NBYTES_MLNO20: 0x00000000,
        TCD_SLAST20: 0x00000000,
        TCD_DADDR20: 0x00000000,
        TCD_DOFF20: 0x00000000,
        TCD_CITER_ELINKNO20: 0x00000000,
        TCD_DLASTSGA20: 0x00000000,
        TCD_CSR20: 0x00000000,
        TCD_BITER_ELINKNO20: 0x00000000,
        TCD_SADDR21: 0x00000000,
        TCD_SOFF21: 0x00000000,
        TCD_ATTR21: 0x00000000,
        TCD_NBYTES_MLNO21: 0x00000000,
        TCD_SLAST21: 0x00000000,
        TCD_DADDR21: 0x00000000,
        TCD_DOFF21: 0x00000000,
        TCD_CITER_ELINKNO21: 0x00000000,
        TCD_DLASTSGA21: 0x00000000,
        TCD_CSR21: 0x00000000,
        TCD_BITER_ELINKNO21: 0x00000000,
        TCD_SADDR22: 0x00000000,
        TCD_SOFF22: 0x00000000,
        TCD_ATTR22: 0x00000000,
        TCD_NBYTES_MLNO22: 0x00000000,
        TCD_SLAST22: 0x00000000,
        TCD_DADDR22: 0x00000000,
        TCD_DOFF22: 0x00000000,
        TCD_CITER_ELINKNO22: 0x00000000,
        TCD_DLASTSGA22: 0x00000000,
        TCD_CSR22: 0x00000000,
        TCD_BITER_ELINKNO22: 0x00000000,
        TCD_SADDR23: 0x00000000,
        TCD_SOFF23: 0x00000000,
        TCD_ATTR23: 0x00000000,
        TCD_NBYTES_MLNO23: 0x00000000,
        TCD_SLAST23: 0x00000000,
        TCD_DADDR23: 0x00000000,
        TCD_DOFF23: 0x00000000,
        TCD_CITER_ELINKNO23: 0x00000000,
        TCD_DLASTSGA23: 0x00000000,
        TCD_CSR23: 0x00000000,
        TCD_BITER_ELINKNO23: 0x00000000,
        TCD_SADDR24: 0x00000000,
        TCD_SOFF24: 0x00000000,
        TCD_ATTR24: 0x00000000,
        TCD_NBYTES_MLNO24: 0x00000000,
        TCD_SLAST24: 0x00000000,
        TCD_DADDR24: 0x00000000,
        TCD_DOFF24: 0x00000000,
        TCD_CITER_ELINKNO24: 0x00000000,
        TCD_DLASTSGA24: 0x00000000,
        TCD_CSR24: 0x00000000,
        TCD_BITER_ELINKNO24: 0x00000000,
        TCD_SADDR25: 0x00000000,
        TCD_SOFF25: 0x00000000,
        TCD_ATTR25: 0x00000000,
        TCD_NBYTES_MLNO25: 0x00000000,
        TCD_SLAST25: 0x00000000,
        TCD_DADDR25: 0x00000000,
        TCD_DOFF25: 0x00000000,
        TCD_CITER_ELINKNO25: 0x00000000,
        TCD_DLASTSGA25: 0x00000000,
        TCD_CSR25: 0x00000000,
        TCD_BITER_ELINKNO25: 0x00000000,
        TCD_SADDR26: 0x00000000,
        TCD_SOFF26: 0x00000000,
        TCD_ATTR26: 0x00000000,
        TCD_NBYTES_MLNO26: 0x00000000,
        TCD_SLAST26: 0x00000000,
        TCD_DADDR26: 0x00000000,
        TCD_DOFF26: 0x00000000,
        TCD_CITER_ELINKNO26: 0x00000000,
        TCD_DLASTSGA26: 0x00000000,
        TCD_CSR26: 0x00000000,
        TCD_BITER_ELINKNO26: 0x00000000,
        TCD_SADDR27: 0x00000000,
        TCD_SOFF27: 0x00000000,
        TCD_ATTR27: 0x00000000,
        TCD_NBYTES_MLNO27: 0x00000000,
        TCD_SLAST27: 0x00000000,
        TCD_DADDR27: 0x00000000,
        TCD_DOFF27: 0x00000000,
        TCD_CITER_ELINKNO27: 0x00000000,
        TCD_DLASTSGA27: 0x00000000,
        TCD_CSR27: 0x00000000,
        TCD_BITER_ELINKNO27: 0x00000000,
        TCD_SADDR28: 0x00000000,
        TCD_SOFF28: 0x00000000,
        TCD_ATTR28: 0x00000000,
        TCD_NBYTES_MLNO28: 0x00000000,
        TCD_SLAST28: 0x00000000,
        TCD_DADDR28: 0x00000000,
        TCD_DOFF28: 0x00000000,
        TCD_CITER_ELINKNO28: 0x00000000,
        TCD_DLASTSGA28: 0x00000000,
        TCD_CSR28: 0x00000000,
        TCD_BITER_ELINKNO28: 0x00000000,
        TCD_SADDR29: 0x00000000,
        TCD_SOFF29: 0x00000000,
        TCD_ATTR29: 0x00000000,
        TCD_NBYTES_MLNO29: 0x00000000,
        TCD_SLAST29: 0x00000000,
        TCD_DADDR29: 0x00000000,
        TCD_DOFF29: 0x00000000,
        TCD_CITER_ELINKNO29: 0x00000000,
        TCD_DLASTSGA29: 0x00000000,
        TCD_CSR29: 0x00000000,
        TCD_BITER_ELINKNO29: 0x00000000,
        TCD_SADDR30: 0x00000000,
        TCD_SOFF30: 0x00000000,
        TCD_ATTR30: 0x00000000,
        TCD_NBYTES_MLNO30: 0x00000000,
        TCD_SLAST30: 0x00000000,
        TCD_DADDR30: 0x00000000,
        TCD_DOFF30: 0x00000000,
        TCD_CITER_ELINKNO30: 0x00000000,
        TCD_DLASTSGA30: 0x00000000,
        TCD_CSR30: 0x00000000,
        TCD_BITER_ELINKNO30: 0x00000000,
        TCD_SADDR31: 0x00000000,
        TCD_SOFF31: 0x00000000,
        TCD_ATTR31: 0x00000000,
        TCD_NBYTES_MLNO31: 0x00000000,
        TCD_SLAST31: 0x00000000,
        TCD_DADDR31: 0x00000000,
        TCD_DOFF31: 0x00000000,
        TCD_CITER_ELINKNO31: 0x00000000,
        TCD_DLASTSGA31: 0x00000000,
        TCD_CSR31: 0x00000000,
        TCD_BITER_ELINKNO31: 0x00000000,
    };

    /// Safe access to DMA0
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Self> {
        let taken = DMA0_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DMA0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Self) {
        assert!(
            inst.addr == Self::INSTANCE.addr,
            "Released the wrong instance"
        );

        let taken = DMA0_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DMA0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DMA0_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with DMA0
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 17] = [
        crate::interrupt::DMA0_DMA16,
        crate::interrupt::DMA1_DMA17,
        crate::interrupt::DMA2_DMA18,
        crate::interrupt::DMA3_DMA19,
        crate::interrupt::DMA4_DMA20,
        crate::interrupt::DMA5_DMA21,
        crate::interrupt::DMA6_DMA22,
        crate::interrupt::DMA7_DMA23,
        crate::interrupt::DMA8_DMA24,
        crate::interrupt::DMA9_DMA25,
        crate::interrupt::DMA10_DMA26,
        crate::interrupt::DMA11_DMA27,
        crate::interrupt::DMA12_DMA28,
        crate::interrupt::DMA13_DMA29,
        crate::interrupt::DMA14_DMA30,
        crate::interrupt::DMA15_DMA31,
        crate::interrupt::DMA_ERROR,
    ];

    /// The interrupts associated with DMA0
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DMA0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA0: *const RegisterBlock = 0x400e8000 as *const _;
