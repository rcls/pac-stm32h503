#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2_SPEC>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2_SPEC>;
#[doc = "AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE_A {
    #[doc = "8: rcc_hclk = sys_ck / 2"]
    B_0x8 = 8,
    #[doc = "9: rcc_hclk = sys_ck / 4"]
    B_0x9 = 9,
    #[doc = "10: rcc_hclk = sys_ck / 8"]
    B_0xA = 10,
    #[doc = "11: rcc_hclk = sys_ck / 16"]
    B_0xB = 11,
    #[doc = "12: rcc_hclk = sys_ck / 64"]
    B_0xC = 12,
    #[doc = "13: rcc_hclk = sys_ck / 128"]
    B_0xD = 13,
    #[doc = "14: rcc_hclk = sys_ck / 256"]
    B_0xE = 14,
    #[doc = "15: rcc_hclk = sys_ck / 512"]
    B_0xF = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPRE_A {
    type Ux = u8;
}
impl crate::IsEnum for HPRE_A {}
#[doc = "Field `HPRE` reader - AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)"]
pub type HPRE_R = crate::FieldReader<HPRE_A>;
impl HPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            8 => Some(HPRE_A::B_0x8),
            9 => Some(HPRE_A::B_0x9),
            10 => Some(HPRE_A::B_0xA),
            11 => Some(HPRE_A::B_0xB),
            12 => Some(HPRE_A::B_0xC),
            13 => Some(HPRE_A::B_0xD),
            14 => Some(HPRE_A::B_0xE),
            15 => Some(HPRE_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "rcc_hclk = sys_ck / 2"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == HPRE_A::B_0x8
    }
    #[doc = "rcc_hclk = sys_ck / 4"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == HPRE_A::B_0x9
    }
    #[doc = "rcc_hclk = sys_ck / 8"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == HPRE_A::B_0xA
    }
    #[doc = "rcc_hclk = sys_ck / 16"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == HPRE_A::B_0xB
    }
    #[doc = "rcc_hclk = sys_ck / 64"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == HPRE_A::B_0xC
    }
    #[doc = "rcc_hclk = sys_ck / 128"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == HPRE_A::B_0xD
    }
    #[doc = "rcc_hclk = sys_ck / 256"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == HPRE_A::B_0xE
    }
    #[doc = "rcc_hclk = sys_ck / 512"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == HPRE_A::B_0xF
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)"]
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HPRE_A>;
impl<'a, REG> HPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_hclk = sys_ck / 2"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE_A::B_0x8)
    }
    #[doc = "rcc_hclk = sys_ck / 4"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE_A::B_0x9)
    }
    #[doc = "rcc_hclk = sys_ck / 8"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE_A::B_0xA)
    }
    #[doc = "rcc_hclk = sys_ck / 16"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE_A::B_0xB)
    }
    #[doc = "rcc_hclk = sys_ck / 64"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE_A::B_0xC)
    }
    #[doc = "rcc_hclk = sys_ck / 128"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE_A::B_0xD)
    }
    #[doc = "rcc_hclk = sys_ck / 256"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE_A::B_0xE)
    }
    #[doc = "rcc_hclk = sys_ck / 512"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE_A::B_0xF)
    }
}
#[doc = "APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE1_A {
    #[doc = "4: rcc_pclk1 = rcc_hclk1 / 2"]
    B_0x4 = 4,
    #[doc = "5: rcc_pclk1 = rcc_hclk1 / 4"]
    B_0x5 = 5,
    #[doc = "6: rcc_pclk1 = rcc_hclk1 / 8"]
    B_0x6 = 6,
    #[doc = "7: rcc_pclk1 = rcc_hclk1 / 16"]
    B_0x7 = 7,
}
impl From<PPRE1_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE1_A {
    type Ux = u8;
}
impl crate::IsEnum for PPRE1_A {}
#[doc = "Field `PPRE1` reader - APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
pub type PPRE1_R = crate::FieldReader<PPRE1_A>;
impl PPRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PPRE1_A> {
        match self.bits {
            4 => Some(PPRE1_A::B_0x4),
            5 => Some(PPRE1_A::B_0x5),
            6 => Some(PPRE1_A::B_0x6),
            7 => Some(PPRE1_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "rcc_pclk1 = rcc_hclk1 / 2"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PPRE1_A::B_0x4
    }
    #[doc = "rcc_pclk1 = rcc_hclk1 / 4"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PPRE1_A::B_0x5
    }
    #[doc = "rcc_pclk1 = rcc_hclk1 / 8"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == PPRE1_A::B_0x6
    }
    #[doc = "rcc_pclk1 = rcc_hclk1 / 16"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == PPRE1_A::B_0x7
    }
}
#[doc = "Field `PPRE1` writer - APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE1_A>;
impl<'a, REG> PPRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk1 = rcc_hclk1 / 2"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1_A::B_0x4)
    }
    #[doc = "rcc_pclk1 = rcc_hclk1 / 4"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1_A::B_0x5)
    }
    #[doc = "rcc_pclk1 = rcc_hclk1 / 8"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1_A::B_0x6)
    }
    #[doc = "rcc_pclk1 = rcc_hclk1 / 16"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1_A::B_0x7)
    }
}
#[doc = "APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE2_A {
    #[doc = "4: rcc_pclk2 = rcc_hclk1 / 2"]
    B_0x4 = 4,
    #[doc = "5: rcc_pclk2 = rcc_hclk1 / 4"]
    B_0x5 = 5,
    #[doc = "6: rcc_pclk2 = rcc_hclk1 / 8"]
    B_0x6 = 6,
    #[doc = "7: rcc_pclk2 = rcc_hclk1 / 16"]
    B_0x7 = 7,
}
impl From<PPRE2_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE2_A {
    type Ux = u8;
}
impl crate::IsEnum for PPRE2_A {}
#[doc = "Field `PPRE2` reader - APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1"]
pub type PPRE2_R = crate::FieldReader<PPRE2_A>;
impl PPRE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PPRE2_A> {
        match self.bits {
            4 => Some(PPRE2_A::B_0x4),
            5 => Some(PPRE2_A::B_0x5),
            6 => Some(PPRE2_A::B_0x6),
            7 => Some(PPRE2_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "rcc_pclk2 = rcc_hclk1 / 2"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PPRE2_A::B_0x4
    }
    #[doc = "rcc_pclk2 = rcc_hclk1 / 4"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PPRE2_A::B_0x5
    }
    #[doc = "rcc_pclk2 = rcc_hclk1 / 8"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == PPRE2_A::B_0x6
    }
    #[doc = "rcc_pclk2 = rcc_hclk1 / 16"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == PPRE2_A::B_0x7
    }
}
#[doc = "Field `PPRE2` writer - APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1"]
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE2_A>;
impl<'a, REG> PPRE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk2 = rcc_hclk1 / 2"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE2_A::B_0x4)
    }
    #[doc = "rcc_pclk2 = rcc_hclk1 / 4"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE2_A::B_0x5)
    }
    #[doc = "rcc_pclk2 = rcc_hclk1 / 8"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE2_A::B_0x6)
    }
    #[doc = "rcc_pclk2 = rcc_hclk1 / 16"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE2_A::B_0x7)
    }
}
#[doc = "APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE3_A {
    #[doc = "4: rcc_pclk3 = rcc_hclk1 / 2"]
    B_0x4 = 4,
    #[doc = "5: rcc_pclk3 = rcc_hclk1 / 4"]
    B_0x5 = 5,
    #[doc = "6: rcc_pclk3 = rcc_hclk1 / 8"]
    B_0x6 = 6,
    #[doc = "7: rcc_pclk3 = rcc_hclk1 / 16"]
    B_0x7 = 7,
}
impl From<PPRE3_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE3_A {
    type Ux = u8;
}
impl crate::IsEnum for PPRE3_A {}
#[doc = "Field `PPRE3` reader - APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1"]
pub type PPRE3_R = crate::FieldReader<PPRE3_A>;
impl PPRE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PPRE3_A> {
        match self.bits {
            4 => Some(PPRE3_A::B_0x4),
            5 => Some(PPRE3_A::B_0x5),
            6 => Some(PPRE3_A::B_0x6),
            7 => Some(PPRE3_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "rcc_pclk3 = rcc_hclk1 / 2"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PPRE3_A::B_0x4
    }
    #[doc = "rcc_pclk3 = rcc_hclk1 / 4"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PPRE3_A::B_0x5
    }
    #[doc = "rcc_pclk3 = rcc_hclk1 / 8"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == PPRE3_A::B_0x6
    }
    #[doc = "rcc_pclk3 = rcc_hclk1 / 16"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == PPRE3_A::B_0x7
    }
}
#[doc = "Field `PPRE3` writer - APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1"]
pub type PPRE3_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE3_A>;
impl<'a, REG> PPRE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk3 = rcc_hclk1 / 2"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE3_A::B_0x4)
    }
    #[doc = "rcc_pclk3 = rcc_hclk1 / 4"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE3_A::B_0x5)
    }
    #[doc = "rcc_pclk3 = rcc_hclk1 / 8"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE3_A::B_0x6)
    }
    #[doc = "rcc_pclk3 = rcc_hclk1 / 16"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE3_A::B_0x7)
    }
}
#[doc = "AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB1DIS_A {
    #[doc = "0: AHB1 clock enabled, distributed to peripherals according to their dedicated clock"]
    B_0x0 = 0,
    #[doc = "1: AHB1 clock disabled"]
    B_0x1 = 1,
}
impl From<AHB1DIS_A> for bool {
    #[inline(always)]
    fn from(variant: AHB1DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB1DIS` reader - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits"]
pub type AHB1DIS_R = crate::BitReader<AHB1DIS_A>;
impl AHB1DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHB1DIS_A {
        match self.bits {
            false => AHB1DIS_A::B_0x0,
            true => AHB1DIS_A::B_0x1,
        }
    }
    #[doc = "AHB1 clock enabled, distributed to peripherals according to their dedicated clock"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AHB1DIS_A::B_0x0
    }
    #[doc = "AHB1 clock disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AHB1DIS_A::B_0x1
    }
}
#[doc = "Field `AHB1DIS` writer - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits"]
pub type AHB1DIS_W<'a, REG> = crate::BitWriter<'a, REG, AHB1DIS_A>;
impl<'a, REG> AHB1DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB1 clock enabled, distributed to peripherals according to their dedicated clock"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AHB1DIS_A::B_0x0)
    }
    #[doc = "AHB1 clock disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AHB1DIS_A::B_0x1)
    }
}
#[doc = "AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB2DIS_A {
    #[doc = "0: AHB2 clock enabled, distributed to peripherals according to their dedicated clock"]
    B_0x0 = 0,
    #[doc = "1: AHB2 clock disabled"]
    B_0x1 = 1,
}
impl From<AHB2DIS_A> for bool {
    #[inline(always)]
    fn from(variant: AHB2DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB2DIS` reader - AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits"]
pub type AHB2DIS_R = crate::BitReader<AHB2DIS_A>;
impl AHB2DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHB2DIS_A {
        match self.bits {
            false => AHB2DIS_A::B_0x0,
            true => AHB2DIS_A::B_0x1,
        }
    }
    #[doc = "AHB2 clock enabled, distributed to peripherals according to their dedicated clock"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AHB2DIS_A::B_0x0
    }
    #[doc = "AHB2 clock disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AHB2DIS_A::B_0x1
    }
}
#[doc = "Field `AHB2DIS` writer - AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits"]
pub type AHB2DIS_W<'a, REG> = crate::BitWriter<'a, REG, AHB2DIS_A>;
impl<'a, REG> AHB2DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB2 clock enabled, distributed to peripherals according to their dedicated clock"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AHB2DIS_A::B_0x0)
    }
    #[doc = "AHB2 clock disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AHB2DIS_A::B_0x1)
    }
}
#[doc = "AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB4DIS_A {
    #[doc = "0: AHB4 clock enabled, distributed to peripherals according to their dedicated clock"]
    B_0x0 = 0,
    #[doc = "1: AHB4 clock disabled"]
    B_0x1 = 1,
}
impl From<AHB4DIS_A> for bool {
    #[inline(always)]
    fn from(variant: AHB4DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB4DIS` reader - AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits"]
pub type AHB4DIS_R = crate::BitReader<AHB4DIS_A>;
impl AHB4DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHB4DIS_A {
        match self.bits {
            false => AHB4DIS_A::B_0x0,
            true => AHB4DIS_A::B_0x1,
        }
    }
    #[doc = "AHB4 clock enabled, distributed to peripherals according to their dedicated clock"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AHB4DIS_A::B_0x0
    }
    #[doc = "AHB4 clock disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AHB4DIS_A::B_0x1
    }
}
#[doc = "Field `AHB4DIS` writer - AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits"]
pub type AHB4DIS_W<'a, REG> = crate::BitWriter<'a, REG, AHB4DIS_A>;
impl<'a, REG> AHB4DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB4 clock enabled, distributed to peripherals according to their dedicated clock"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AHB4DIS_A::B_0x0)
    }
    #[doc = "AHB4 clock disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AHB4DIS_A::B_0x1)
    }
}
#[doc = "APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APB1DIS_A {
    #[doc = "0: APB1 clock enabled, distributed to peripherals according to their dedicated clock enable"]
    B_0x0 = 0,
    #[doc = "1: APB1 clock disabled"]
    B_0x1 = 1,
}
impl From<APB1DIS_A> for bool {
    #[inline(always)]
    fn from(variant: APB1DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APB1DIS` reader - APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits"]
pub type APB1DIS_R = crate::BitReader<APB1DIS_A>;
impl APB1DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> APB1DIS_A {
        match self.bits {
            false => APB1DIS_A::B_0x0,
            true => APB1DIS_A::B_0x1,
        }
    }
    #[doc = "APB1 clock enabled, distributed to peripherals according to their dedicated clock enable"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == APB1DIS_A::B_0x0
    }
    #[doc = "APB1 clock disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == APB1DIS_A::B_0x1
    }
}
#[doc = "Field `APB1DIS` writer - APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits"]
pub type APB1DIS_W<'a, REG> = crate::BitWriter<'a, REG, APB1DIS_A>;
impl<'a, REG> APB1DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB1 clock enabled, distributed to peripherals according to their dedicated clock enable"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIS_A::B_0x0)
    }
    #[doc = "APB1 clock disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIS_A::B_0x1)
    }
}
#[doc = "APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APB2DIS_A {
    #[doc = "0: APB2 clock enabled, distributed to peripherals according to their dedicated clock enable"]
    B_0x0 = 0,
    #[doc = "1: APB2 clock disabled"]
    B_0x1 = 1,
}
impl From<APB2DIS_A> for bool {
    #[inline(always)]
    fn from(variant: APB2DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APB2DIS` reader - APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits"]
pub type APB2DIS_R = crate::BitReader<APB2DIS_A>;
impl APB2DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> APB2DIS_A {
        match self.bits {
            false => APB2DIS_A::B_0x0,
            true => APB2DIS_A::B_0x1,
        }
    }
    #[doc = "APB2 clock enabled, distributed to peripherals according to their dedicated clock enable"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == APB2DIS_A::B_0x0
    }
    #[doc = "APB2 clock disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == APB2DIS_A::B_0x1
    }
}
#[doc = "Field `APB2DIS` writer - APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits"]
pub type APB2DIS_W<'a, REG> = crate::BitWriter<'a, REG, APB2DIS_A>;
impl<'a, REG> APB2DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB2 clock enabled, distributed to peripherals according to their dedicated clock enable"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(APB2DIS_A::B_0x0)
    }
    #[doc = "APB2 clock disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(APB2DIS_A::B_0x1)
    }
}
#[doc = "APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APB3DIS_A {
    #[doc = "0: APB3 clock enabled, distributed to peripherals according to their dedicated clock enable"]
    B_0x0 = 0,
    #[doc = "1: APB3 clock disabled"]
    B_0x1 = 1,
}
impl From<APB3DIS_A> for bool {
    #[inline(always)]
    fn from(variant: APB3DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APB3DIS` reader - APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits"]
pub type APB3DIS_R = crate::BitReader<APB3DIS_A>;
impl APB3DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> APB3DIS_A {
        match self.bits {
            false => APB3DIS_A::B_0x0,
            true => APB3DIS_A::B_0x1,
        }
    }
    #[doc = "APB3 clock enabled, distributed to peripherals according to their dedicated clock enable"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == APB3DIS_A::B_0x0
    }
    #[doc = "APB3 clock disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == APB3DIS_A::B_0x1
    }
}
#[doc = "Field `APB3DIS` writer - APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits"]
pub type APB3DIS_W<'a, REG> = crate::BitWriter<'a, REG, APB3DIS_A>;
impl<'a, REG> APB3DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB3 clock enabled, distributed to peripherals according to their dedicated clock enable"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(APB3DIS_A::B_0x0)
    }
    #[doc = "APB3 clock disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(APB3DIS_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:3 - AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)"]
    #[inline(always)]
    pub fn HPRE(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    pub fn PPRE1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1"]
    #[inline(always)]
    pub fn PPRE2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1"]
    #[inline(always)]
    pub fn PPRE3(&self) -> PPRE3_R {
        PPRE3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits"]
    #[inline(always)]
    pub fn AHB1DIS(&self) -> AHB1DIS_R {
        AHB1DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits"]
    #[inline(always)]
    pub fn AHB2DIS(&self) -> AHB2DIS_R {
        AHB2DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits"]
    #[inline(always)]
    pub fn AHB4DIS(&self) -> AHB4DIS_R {
        AHB4DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits"]
    #[inline(always)]
    pub fn APB1DIS(&self) -> APB1DIS_R {
        APB1DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits"]
    #[inline(always)]
    pub fn APB2DIS(&self) -> APB2DIS_R {
        APB2DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits"]
    #[inline(always)]
    pub fn APB3DIS(&self) -> APB3DIS_R {
        APB3DIS_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)"]
    #[inline(always)]
    pub fn HPRE(&mut self) -> HPRE_W<'_, CFGR2_SPEC> {
        HPRE_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    pub fn PPRE1(&mut self) -> PPRE1_W<'_, CFGR2_SPEC> {
        PPRE1_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1"]
    #[inline(always)]
    pub fn PPRE2(&mut self) -> PPRE2_W<'_, CFGR2_SPEC> {
        PPRE2_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1"]
    #[inline(always)]
    pub fn PPRE3(&mut self) -> PPRE3_W<'_, CFGR2_SPEC> {
        PPRE3_W::new(self, 12)
    }
    #[doc = "Bit 16 - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits"]
    #[inline(always)]
    pub fn AHB1DIS(&mut self) -> AHB1DIS_W<'_, CFGR2_SPEC> {
        AHB1DIS_W::new(self, 16)
    }
    #[doc = "Bit 17 - AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits"]
    #[inline(always)]
    pub fn AHB2DIS(&mut self) -> AHB2DIS_W<'_, CFGR2_SPEC> {
        AHB2DIS_W::new(self, 17)
    }
    #[doc = "Bit 19 - AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits"]
    #[inline(always)]
    pub fn AHB4DIS(&mut self) -> AHB4DIS_W<'_, CFGR2_SPEC> {
        AHB4DIS_W::new(self, 19)
    }
    #[doc = "Bit 20 - APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits"]
    #[inline(always)]
    pub fn APB1DIS(&mut self) -> APB1DIS_W<'_, CFGR2_SPEC> {
        APB1DIS_W::new(self, 20)
    }
    #[doc = "Bit 21 - APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits"]
    #[inline(always)]
    pub fn APB2DIS(&mut self) -> APB2DIS_W<'_, CFGR2_SPEC> {
        APB2DIS_W::new(self, 21)
    }
    #[doc = "Bit 22 - APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits"]
    #[inline(always)]
    pub fn APB3DIS(&mut self) -> APB3DIS_W<'_, CFGR2_SPEC> {
        APB3DIS_W::new(self, 22)
    }
}
#[doc = "RCC CPU domain clock configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {}
