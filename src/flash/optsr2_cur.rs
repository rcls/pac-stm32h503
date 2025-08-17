#[doc = "Register `OPTSR2_CUR` reader"]
pub type R = crate::R<OPTSR2_CUR_SPEC>;
#[doc = "SRAM2 erase when system reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_RST_A {
    #[doc = "0: SRAM2 erased when a system reset occurs"]
    B_0x0 = 0,
    #[doc = "1: SRAM2 not erased when a system reset occurs."]
    B_0x1 = 1,
}
impl From<SRAM2_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2_RST` reader - SRAM2 erase when system reset"]
pub type SRAM2_RST_R = crate::BitReader<SRAM2_RST_A>;
impl SRAM2_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2_RST_A {
        match self.bits {
            false => SRAM2_RST_A::B_0x0,
            true => SRAM2_RST_A::B_0x1,
        }
    }
    #[doc = "SRAM2 erased when a system reset occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM2_RST_A::B_0x0
    }
    #[doc = "SRAM2 not erased when a system reset occurs."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM2_RST_A::B_0x1
    }
}
#[doc = "Backup RAM ECC detection and correction disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKPRAM_ECC_A {
    #[doc = "0: BKPRAM ECC check enabled"]
    B_0x0 = 0,
    #[doc = "1: BKPRAM ECC check disabled"]
    B_0x1 = 1,
}
impl From<BKPRAM_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: BKPRAM_ECC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction disable"]
pub type BKPRAM_ECC_R = crate::BitReader<BKPRAM_ECC_A>;
impl BKPRAM_ECC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKPRAM_ECC_A {
        match self.bits {
            false => BKPRAM_ECC_A::B_0x0,
            true => BKPRAM_ECC_A::B_0x1,
        }
    }
    #[doc = "BKPRAM ECC check enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKPRAM_ECC_A::B_0x0
    }
    #[doc = "BKPRAM ECC check disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKPRAM_ECC_A::B_0x1
    }
}
#[doc = "SRAM2 ECC detection and correction disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_ECC_A {
    #[doc = "0: SRAM2 ECC check enabled"]
    B_0x0 = 0,
    #[doc = "1: SRAM2 ECC check disabled"]
    B_0x1 = 1,
}
impl From<SRAM2_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_ECC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction disable"]
pub type SRAM2_ECC_R = crate::BitReader<SRAM2_ECC_A>;
impl SRAM2_ECC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2_ECC_A {
        match self.bits {
            false => SRAM2_ECC_A::B_0x0,
            true => SRAM2_ECC_A::B_0x1,
        }
    }
    #[doc = "SRAM2 ECC check enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM2_ECC_A::B_0x0
    }
    #[doc = "SRAM2 ECC check disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM2_ECC_A::B_0x1
    }
}
#[doc = "SRAM1 erase upon system reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1_RST_A {
    #[doc = "0: SRAM1 erased when a system reset occurs"]
    B_0x0 = 0,
    #[doc = "1: SRAM1 not erased when a system reset occurs"]
    B_0x1 = 1,
}
impl From<SRAM1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1_RST` reader - SRAM1 erase upon system reset"]
pub type SRAM1_RST_R = crate::BitReader<SRAM1_RST_A>;
impl SRAM1_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1_RST_A {
        match self.bits {
            false => SRAM1_RST_A::B_0x0,
            true => SRAM1_RST_A::B_0x1,
        }
    }
    #[doc = "SRAM1 erased when a system reset occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM1_RST_A::B_0x0
    }
    #[doc = "SRAM1 not erased when a system reset occurs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM1_RST_A::B_0x1
    }
}
#[doc = "SRAM1 ECC detection and correction disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1_ECC_A {
    #[doc = "0: SRAM1 ECC check enabled"]
    B_0x0 = 0,
    #[doc = "1: SRAM1 ECC check disabled"]
    B_0x1 = 1,
}
impl From<SRAM1_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1_ECC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1_ECC` reader - SRAM1 ECC detection and correction disable"]
pub type SRAM1_ECC_R = crate::BitReader<SRAM1_ECC_A>;
impl SRAM1_ECC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1_ECC_A {
        match self.bits {
            false => SRAM1_ECC_A::B_0x0,
            true => SRAM1_ECC_A::B_0x1,
        }
    }
    #[doc = "SRAM1 ECC check enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM1_ECC_A::B_0x0
    }
    #[doc = "SRAM1 ECC check disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM1_ECC_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 3 - SRAM2 erase when system reset"]
    #[inline(always)]
    pub fn SRAM2_RST(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Backup RAM ECC detection and correction disable"]
    #[inline(always)]
    pub fn BKPRAM_ECC(&self) -> BKPRAM_ECC_R {
        BKPRAM_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM2 ECC detection and correction disable"]
    #[inline(always)]
    pub fn SRAM2_ECC(&self) -> SRAM2_ECC_R {
        SRAM2_ECC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 erase upon system reset"]
    #[inline(always)]
    pub fn SRAM1_RST(&self) -> SRAM1_RST_R {
        SRAM1_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM1 ECC detection and correction disable"]
    #[inline(always)]
    pub fn SRAM1_ECC(&self) -> SRAM1_ECC_R {
        SRAM1_ECC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "FLASH option status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr2_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTSR2_CUR_SPEC;
impl crate::RegisterSpec for OPTSR2_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optsr2_cur::R`](R) reader structure"]
impl crate::Readable for OPTSR2_CUR_SPEC {}
#[doc = "`reset()` method sets OPTSR2_CUR to value 0"]
impl crate::Resettable for OPTSR2_CUR_SPEC {}
