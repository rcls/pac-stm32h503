#[doc = "Register `OPTSR_CUR` reader"]
pub type R = crate::R<OPTSR_CUR_SPEC>;
#[doc = "Brownout level option status bit These bits reflects the power level that generates a system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOR_LEV_A {
    #[doc = "0: BOR OFF, POR/PDR reset threshold level is applied"]
    B_0x0 = 0,
    #[doc = "1: BOR Level 1, the threshold level is low (around 2.1 V)"]
    B_0x1 = 1,
    #[doc = "2: BOR Level 2, the threshold level is medium (around 2.4 V)"]
    B_0x2 = 2,
    #[doc = "3: BOR Level 3, the threshold level is high (around 2.7 V)"]
    B_0x3 = 3,
}
impl From<BOR_LEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BOR_LEV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOR_LEV_A {
    type Ux = u8;
}
impl crate::IsEnum for BOR_LEV_A {}
#[doc = "Field `BOR_LEV` reader - Brownout level option status bit These bits reflects the power level that generates a system reset."]
pub type BOR_LEV_R = crate::FieldReader<BOR_LEV_A>;
impl BOR_LEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOR_LEV_A {
        match self.bits {
            0 => BOR_LEV_A::B_0x0,
            1 => BOR_LEV_A::B_0x1,
            2 => BOR_LEV_A::B_0x2,
            3 => BOR_LEV_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "BOR OFF, POR/PDR reset threshold level is applied"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BOR_LEV_A::B_0x0
    }
    #[doc = "BOR Level 1, the threshold level is low (around 2.1 V)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BOR_LEV_A::B_0x1
    }
    #[doc = "BOR Level 2, the threshold level is medium (around 2.4 V)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == BOR_LEV_A::B_0x2
    }
    #[doc = "BOR Level 3, the threshold level is high (around 2.7 V)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == BOR_LEV_A::B_0x3
    }
}
#[doc = "Brownout high enable status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORH_EN_A {
    #[doc = "0: disabled"]
    B_0x0 = 0,
    #[doc = "1: enabled"]
    B_0x1 = 1,
}
impl From<BORH_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BORH_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORH_EN` reader - Brownout high enable status bit"]
pub type BORH_EN_R = crate::BitReader<BORH_EN_A>;
impl BORH_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BORH_EN_A {
        match self.bits {
            false => BORH_EN_A::B_0x0,
            true => BORH_EN_A::B_0x1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BORH_EN_A::B_0x0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BORH_EN_A::B_0x1
    }
}
#[doc = "IWDG control mode option status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_SW_A {
    #[doc = "0: IWDG watchdog is controlled by hardware"]
    B_0x0 = 0,
    #[doc = "1: IWDG watchdog is controlled by software"]
    B_0x1 = 1,
}
impl From<IWDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDG_SW` reader - IWDG control mode option status bit"]
pub type IWDG_SW_R = crate::BitReader<IWDG_SW_A>;
impl IWDG_SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_SW_A {
        match self.bits {
            false => IWDG_SW_A::B_0x0,
            true => IWDG_SW_A::B_0x1,
        }
    }
    #[doc = "IWDG watchdog is controlled by hardware"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IWDG_SW_A::B_0x0
    }
    #[doc = "IWDG watchdog is controlled by software"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IWDG_SW_A::B_0x1
    }
}
#[doc = "WWDG control mode option status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDG_SW_A {
    #[doc = "0: WWDG watchdog is controlled by hardware"]
    B_0x0 = 0,
    #[doc = "1: WWDG watchdog is controlled by software"]
    B_0x1 = 1,
}
impl From<WWDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: WWDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDG_SW` reader - WWDG control mode option status bit"]
pub type WWDG_SW_R = crate::BitReader<WWDG_SW_A>;
impl WWDG_SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDG_SW_A {
        match self.bits {
            false => WWDG_SW_A::B_0x0,
            true => WWDG_SW_A::B_0x1,
        }
    }
    #[doc = "WWDG watchdog is controlled by hardware"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WWDG_SW_A::B_0x0
    }
    #[doc = "WWDG watchdog is controlled by software"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WWDG_SW_A::B_0x1
    }
}
#[doc = "Core domain Shutdown entry reset option status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRST_SHDW_A {
    #[doc = "0: a reset is generated when entering Shutdown mode on core domain"]
    B_0x0 = 0,
    #[doc = "1: no reset generated when entering Shutdown mode on core domain."]
    B_0x1 = 1,
}
impl From<NRST_SHDW_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_SHDW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRST_SHDW` reader - Core domain Shutdown entry reset option status bit"]
pub type NRST_SHDW_R = crate::BitReader<NRST_SHDW_A>;
impl NRST_SHDW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NRST_SHDW_A {
        match self.bits {
            false => NRST_SHDW_A::B_0x0,
            true => NRST_SHDW_A::B_0x1,
        }
    }
    #[doc = "a reset is generated when entering Shutdown mode on core domain"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NRST_SHDW_A::B_0x0
    }
    #[doc = "no reset generated when entering Shutdown mode on core domain."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NRST_SHDW_A::B_0x1
    }
}
#[doc = "Core domain Stop entry reset option status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRST_STOP_A {
    #[doc = "0: a reset is generated when entering Stop mode on core domain"]
    B_0x0 = 0,
    #[doc = "1: no reset generated when entering Stop mode on core domain."]
    B_0x1 = 1,
}
impl From<NRST_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRST_STOP` reader - Core domain Stop entry reset option status bit"]
pub type NRST_STOP_R = crate::BitReader<NRST_STOP_A>;
impl NRST_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NRST_STOP_A {
        match self.bits {
            false => NRST_STOP_A::B_0x0,
            true => NRST_STOP_A::B_0x1,
        }
    }
    #[doc = "a reset is generated when entering Stop mode on core domain"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NRST_STOP_A::B_0x0
    }
    #[doc = "no reset generated when entering Stop mode on core domain."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NRST_STOP_A::B_0x1
    }
}
#[doc = "Core domain Standby entry reset option status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRST_STDBY_A {
    #[doc = "0: a reset is generated when entering Standby mode on core domain"]
    B_0x0 = 0,
    #[doc = "1: no reset generated when entering Standby mode on core domain."]
    B_0x1 = 1,
}
impl From<NRST_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRST_STDBY` reader - Core domain Standby entry reset option status bit"]
pub type NRST_STDBY_R = crate::BitReader<NRST_STDBY_A>;
impl NRST_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NRST_STDBY_A {
        match self.bits {
            false => NRST_STDBY_A::B_0x0,
            true => NRST_STDBY_A::B_0x1,
        }
    }
    #[doc = "a reset is generated when entering Standby mode on core domain"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NRST_STDBY_A::B_0x0
    }
    #[doc = "no reset generated when entering Standby mode on core domain."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NRST_STDBY_A::B_0x1
    }
}
#[doc = "Field `PRODUCT_STATE` reader - Life state code (based on Hamming 8,4). More information in ."]
pub type PRODUCT_STATE_R = crate::FieldReader;
#[doc = "High-speed IO at low VDD voltage status bit. This bit can be set only with VDD below 2.5 V.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IO_VDD_HSLV_A {
    #[doc = "0: High-speed IO at low VDD voltage feature disabled (VDD can exceed 2.5 V)"]
    B_0x0 = 0,
    #[doc = "1: High-speed IO at low VDD voltage feature enabled (VDD remains below 2.5 V)"]
    B_0x1 = 1,
}
impl From<IO_VDD_HSLV_A> for bool {
    #[inline(always)]
    fn from(variant: IO_VDD_HSLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO_VDD_HSLV` reader - High-speed IO at low VDD voltage status bit. This bit can be set only with VDD below 2.5 V."]
pub type IO_VDD_HSLV_R = crate::BitReader<IO_VDD_HSLV_A>;
impl IO_VDD_HSLV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IO_VDD_HSLV_A {
        match self.bits {
            false => IO_VDD_HSLV_A::B_0x0,
            true => IO_VDD_HSLV_A::B_0x1,
        }
    }
    #[doc = "High-speed IO at low VDD voltage feature disabled (VDD can exceed 2.5 V)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IO_VDD_HSLV_A::B_0x0
    }
    #[doc = "High-speed IO at low VDD voltage feature enabled (VDD remains below 2.5 V)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IO_VDD_HSLV_A::B_0x1
    }
}
#[doc = "High-speed IO at low VDDIO2 voltage status bit. This bit can be set only with VDDIO2 below 2.5 V.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IO_VDDIO2_HSLV_A {
    #[doc = "0: High-speed IO at low VDDIO2 voltage feature disabled (VDDIO2 can exceed 2.5 V)"]
    B_0x0 = 0,
    #[doc = "1: High-speed IO at low VDDIO2 voltage feature enabled (VDDIO2 remains below 2.5 V)"]
    B_0x1 = 1,
}
impl From<IO_VDDIO2_HSLV_A> for bool {
    #[inline(always)]
    fn from(variant: IO_VDDIO2_HSLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO_VDDIO2_HSLV` reader - High-speed IO at low VDDIO2 voltage status bit. This bit can be set only with VDDIO2 below 2.5 V."]
pub type IO_VDDIO2_HSLV_R = crate::BitReader<IO_VDDIO2_HSLV_A>;
impl IO_VDDIO2_HSLV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IO_VDDIO2_HSLV_A {
        match self.bits {
            false => IO_VDDIO2_HSLV_A::B_0x0,
            true => IO_VDDIO2_HSLV_A::B_0x1,
        }
    }
    #[doc = "High-speed IO at low VDDIO2 voltage feature disabled (VDDIO2 can exceed 2.5 V)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IO_VDDIO2_HSLV_A::B_0x0
    }
    #[doc = "High-speed IO at low VDDIO2 voltage feature enabled (VDDIO2 remains below 2.5 V)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IO_VDDIO2_HSLV_A::B_0x1
    }
}
#[doc = "IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STOP_A {
    #[doc = "0: Independent watchdog frozen in system Stop mode"]
    B_0x0 = 0,
    #[doc = "1: Independent watchdog keep running in system Stop mode."]
    B_0x1 = 1,
}
impl From<IWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDG_STOP` reader - IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode."]
pub type IWDG_STOP_R = crate::BitReader<IWDG_STOP_A>;
impl IWDG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_STOP_A {
        match self.bits {
            false => IWDG_STOP_A::B_0x0,
            true => IWDG_STOP_A::B_0x1,
        }
    }
    #[doc = "Independent watchdog frozen in system Stop mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IWDG_STOP_A::B_0x0
    }
    #[doc = "Independent watchdog keep running in system Stop mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IWDG_STOP_A::B_0x1
    }
}
#[doc = "IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STDBY_A {
    #[doc = "0: Independent watchdog frozen in Standby mode"]
    B_0x0 = 0,
    #[doc = "1: Independent watchdog keep running in Standby mode."]
    B_0x1 = 1,
}
impl From<IWDG_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDG_STDBY` reader - IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode."]
pub type IWDG_STDBY_R = crate::BitReader<IWDG_STDBY_A>;
impl IWDG_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_STDBY_A {
        match self.bits {
            false => IWDG_STDBY_A::B_0x0,
            true => IWDG_STDBY_A::B_0x1,
        }
    }
    #[doc = "Independent watchdog frozen in Standby mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IWDG_STDBY_A::B_0x0
    }
    #[doc = "Independent watchdog keep running in Standby mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IWDG_STDBY_A::B_0x1
    }
}
#[doc = "Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP_BANK_A {
    #[doc = "0: Bank1 and Bank2 not swapped"]
    B_0x0 = 0,
    #[doc = "1: Bank1 and Bank2 swapped"]
    B_0x1 = 1,
}
impl From<SWAP_BANK_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_BANK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP_BANK` reader - Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset."]
pub type SWAP_BANK_R = crate::BitReader<SWAP_BANK_A>;
impl SWAP_BANK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWAP_BANK_A {
        match self.bits {
            false => SWAP_BANK_A::B_0x0,
            true => SWAP_BANK_A::B_0x1,
        }
    }
    #[doc = "Bank1 and Bank2 not swapped"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWAP_BANK_A::B_0x0
    }
    #[doc = "Bank1 and Bank2 swapped"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWAP_BANK_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:1 - Brownout level option status bit These bits reflects the power level that generates a system reset."]
    #[inline(always)]
    pub fn BOR_LEV(&self) -> BOR_LEV_R {
        BOR_LEV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Brownout high enable status bit"]
    #[inline(always)]
    pub fn BORH_EN(&self) -> BORH_EN_R {
        BORH_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IWDG control mode option status bit"]
    #[inline(always)]
    pub fn IWDG_SW(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WWDG control mode option status bit"]
    #[inline(always)]
    pub fn WWDG_SW(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core domain Shutdown entry reset option status bit"]
    #[inline(always)]
    pub fn NRST_SHDW(&self) -> NRST_SHDW_R {
        NRST_SHDW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core domain Stop entry reset option status bit"]
    #[inline(always)]
    pub fn NRST_STOP(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core domain Standby entry reset option status bit"]
    #[inline(always)]
    pub fn NRST_STDBY(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Life state code (based on Hamming 8,4). More information in ."]
    #[inline(always)]
    pub fn PRODUCT_STATE(&self) -> PRODUCT_STATE_R {
        PRODUCT_STATE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - High-speed IO at low VDD voltage status bit. This bit can be set only with VDD below 2.5 V."]
    #[inline(always)]
    pub fn IO_VDD_HSLV(&self) -> IO_VDD_HSLV_R {
        IO_VDD_HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High-speed IO at low VDDIO2 voltage status bit. This bit can be set only with VDDIO2 below 2.5 V."]
    #[inline(always)]
    pub fn IO_VDDIO2_HSLV(&self) -> IO_VDDIO2_HSLV_R {
        IO_VDDIO2_HSLV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode."]
    #[inline(always)]
    pub fn IWDG_STOP(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode."]
    #[inline(always)]
    pub fn IWDG_STDBY(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset."]
    #[inline(always)]
    pub fn SWAP_BANK(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FLASH option status register\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTSR_CUR_SPEC;
impl crate::RegisterSpec for OPTSR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optsr_cur::R`](R) reader structure"]
impl crate::Readable for OPTSR_CUR_SPEC {}
#[doc = "`reset()` method sets OPTSR_CUR to value 0"]
impl crate::Resettable for OPTSR_CUR_SPEC {}
