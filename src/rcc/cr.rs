#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION_A {
    #[doc = "0: HSI is OFF"]
    B_0x0 = 0,
    #[doc = "1: HSI is ON (default after reset)"]
    B_0x1 = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub type HSION_R = crate::BitReader<HSION_A>;
impl HSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::B_0x0,
            true => HSION_A::B_0x1,
        }
    }
    #[doc = "HSI is OFF"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSION_A::B_0x0
    }
    #[doc = "HSI is ON (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSION_A::B_0x1
    }
}
#[doc = "Field `HSION` writer - HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION_A>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI is OFF"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSION_A::B_0x0)
    }
    #[doc = "HSI is ON (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSION_A::B_0x1)
    }
}
#[doc = "HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDY_A {
    #[doc = "0: HSI clock is not ready (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSI clock is ready"]
    B_0x1 = 1,
}
impl From<HSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable."]
pub type HSIRDY_R = crate::BitReader<HSIRDY_A>;
impl HSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDY_A {
        match self.bits {
            false => HSIRDY_A::B_0x0,
            true => HSIRDY_A::B_0x1,
        }
    }
    #[doc = "HSI clock is not ready (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSIRDY_A::B_0x0
    }
    #[doc = "HSI clock is ready"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSIRDY_A::B_0x1
    }
}
#[doc = "HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON_A {
    #[doc = "0: no effect on HSI (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSI is forced to ON even in Stop mode"]
    B_0x1 = 1,
}
impl From<HSIKERON_A> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIKERON` reader - HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
pub type HSIKERON_R = crate::BitReader<HSIKERON_A>;
impl HSIKERON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIKERON_A {
        match self.bits {
            false => HSIKERON_A::B_0x0,
            true => HSIKERON_A::B_0x1,
        }
    }
    #[doc = "no effect on HSI (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSIKERON_A::B_0x0
    }
    #[doc = "HSI is forced to ON even in Stop mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSIKERON_A::B_0x1
    }
}
#[doc = "Field `HSIKERON` writer - HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG, HSIKERON_A>;
impl<'a, REG> HSIKERON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect on HSI (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON_A::B_0x0)
    }
    #[doc = "HSI is forced to ON even in Stop mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON_A::B_0x1)
    }
}
#[doc = "HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSIDIV_A {
    #[doc = "0: division by 1, hsi(_ker)_ck = 64 MHz (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: division by 2, hsi(_ker)_ck = 32 MHz"]
    B_0x1 = 1,
    #[doc = "2: division by 4, hsi(_ker)_ck = 16 MHz"]
    B_0x2 = 2,
    #[doc = "3: division by 8, hsi(_ker)_ck = 8 MHz"]
    B_0x3 = 3,
}
impl From<HSIDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HSIDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSIDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for HSIDIV_A {}
#[doc = "Field `HSIDIV` reader - HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
pub type HSIDIV_R = crate::FieldReader<HSIDIV_A>;
impl HSIDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIDIV_A {
        match self.bits {
            0 => HSIDIV_A::B_0x0,
            1 => HSIDIV_A::B_0x1,
            2 => HSIDIV_A::B_0x2,
            3 => HSIDIV_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "division by 1, hsi(_ker)_ck = 64 MHz (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSIDIV_A::B_0x0
    }
    #[doc = "division by 2, hsi(_ker)_ck = 32 MHz"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSIDIV_A::B_0x1
    }
    #[doc = "division by 4, hsi(_ker)_ck = 16 MHz"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == HSIDIV_A::B_0x2
    }
    #[doc = "division by 8, hsi(_ker)_ck = 8 MHz"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == HSIDIV_A::B_0x3
    }
}
#[doc = "Field `HSIDIV` writer - HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
pub type HSIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HSIDIV_A, crate::Safe>;
impl<'a, REG> HSIDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "division by 1, hsi(_ker)_ck = 64 MHz (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV_A::B_0x0)
    }
    #[doc = "division by 2, hsi(_ker)_ck = 32 MHz"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV_A::B_0x1)
    }
    #[doc = "division by 4, hsi(_ker)_ck = 16 MHz"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV_A::B_0x2)
    }
    #[doc = "division by 8, hsi(_ker)_ck = 8 MHz"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV_A::B_0x3)
    }
}
#[doc = "HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIDIVF_A {
    #[doc = "0: new division ratio not yet propagated to hsi(_ker)_ck (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: hsi(_ker)_ck clock frequency reflects the new HSIDIV value (default register value when the clock setting is completed)."]
    B_0x1 = 1,
}
impl From<HSIDIVF_A> for bool {
    #[inline(always)]
    fn from(variant: HSIDIVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIDIVF` reader - HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV."]
pub type HSIDIVF_R = crate::BitReader<HSIDIVF_A>;
impl HSIDIVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIDIVF_A {
        match self.bits {
            false => HSIDIVF_A::B_0x0,
            true => HSIDIVF_A::B_0x1,
        }
    }
    #[doc = "new division ratio not yet propagated to hsi(_ker)_ck (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSIDIVF_A::B_0x0
    }
    #[doc = "hsi(_ker)_ck clock frequency reflects the new HSIDIV value (default register value when the clock setting is completed)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSIDIVF_A::B_0x1
    }
}
#[doc = "CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSION_A {
    #[doc = "0: CSI is OFF (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: CSI is ON"]
    B_0x1 = 1,
}
impl From<CSION_A> for bool {
    #[inline(always)]
    fn from(variant: CSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSION` reader - CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub type CSION_R = crate::BitReader<CSION_A>;
impl CSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSION_A {
        match self.bits {
            false => CSION_A::B_0x0,
            true => CSION_A::B_0x1,
        }
    }
    #[doc = "CSI is OFF (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CSION_A::B_0x0
    }
    #[doc = "CSI is ON"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CSION_A::B_0x1
    }
}
#[doc = "Field `CSION` writer - CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub type CSION_W<'a, REG> = crate::BitWriter<'a, REG, CSION_A>;
impl<'a, REG> CSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSI is OFF (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSION_A::B_0x0)
    }
    #[doc = "CSI is ON"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSION_A::B_0x1)
    }
}
#[doc = "CSI clock ready flag Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSIRDY_A {
    #[doc = "0: CSI clock is not ready (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: CSI clock is ready"]
    B_0x1 = 1,
}
impl From<CSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSIRDY` reader - CSI clock ready flag Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request)."]
pub type CSIRDY_R = crate::BitReader<CSIRDY_A>;
impl CSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSIRDY_A {
        match self.bits {
            false => CSIRDY_A::B_0x0,
            true => CSIRDY_A::B_0x1,
        }
    }
    #[doc = "CSI clock is not ready (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CSIRDY_A::B_0x0
    }
    #[doc = "CSI clock is ready"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CSIRDY_A::B_0x1
    }
}
#[doc = "CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSIKERON_A {
    #[doc = "0: no effect on CSI (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: CSI is forced to ON even in Stop mode"]
    B_0x1 = 1,
}
impl From<CSIKERON_A> for bool {
    #[inline(always)]
    fn from(variant: CSIKERON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSIKERON` reader - CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
pub type CSIKERON_R = crate::BitReader<CSIKERON_A>;
impl CSIKERON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSIKERON_A {
        match self.bits {
            false => CSIKERON_A::B_0x0,
            true => CSIKERON_A::B_0x1,
        }
    }
    #[doc = "no effect on CSI (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CSIKERON_A::B_0x0
    }
    #[doc = "CSI is forced to ON even in Stop mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CSIKERON_A::B_0x1
    }
}
#[doc = "Field `CSIKERON` writer - CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
pub type CSIKERON_W<'a, REG> = crate::BitWriter<'a, REG, CSIKERON_A>;
impl<'a, REG> CSIKERON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect on CSI (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSIKERON_A::B_0x0)
    }
    #[doc = "CSI is forced to ON even in Stop mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSIKERON_A::B_0x1)
    }
}
#[doc = "HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48ON_A {
    #[doc = "0: HSI48 is OFF (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSI48 is ON"]
    B_0x1 = 1,
}
impl From<HSI48ON_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48ON` reader - HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
pub type HSI48ON_R = crate::BitReader<HSI48ON_A>;
impl HSI48ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI48ON_A {
        match self.bits {
            false => HSI48ON_A::B_0x0,
            true => HSI48ON_A::B_0x1,
        }
    }
    #[doc = "HSI48 is OFF (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSI48ON_A::B_0x0
    }
    #[doc = "HSI48 is ON"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSI48ON_A::B_0x1
    }
}
#[doc = "Field `HSI48ON` writer - HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
pub type HSI48ON_W<'a, REG> = crate::BitWriter<'a, REG, HSI48ON_A>;
impl<'a, REG> HSI48ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI48 is OFF (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48ON_A::B_0x0)
    }
    #[doc = "HSI48 is ON"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48ON_A::B_0x1)
    }
}
#[doc = "HSI48 clock ready flag Set by hardware to indicate that the HSI48 oscillator is stable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDY_A {
    #[doc = "0: HSI48 clock is not ready (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSI48 clock is ready"]
    B_0x1 = 1,
}
impl From<HSI48RDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDY` reader - HSI48 clock ready flag Set by hardware to indicate that the HSI48 oscillator is stable."]
pub type HSI48RDY_R = crate::BitReader<HSI48RDY_A>;
impl HSI48RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDY_A {
        match self.bits {
            false => HSI48RDY_A::B_0x0,
            true => HSI48RDY_A::B_0x1,
        }
    }
    #[doc = "HSI48 clock is not ready (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSI48RDY_A::B_0x0
    }
    #[doc = "HSI48 clock is ready"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSI48RDY_A::B_0x1
    }
}
#[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON_A {
    #[doc = "0: HSE is OFF (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSE is ON"]
    B_0x1 = 1,
}
impl From<HSEON_A> for bool {
    #[inline(always)]
    fn from(variant: HSEON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEON` reader - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub type HSEON_R = crate::BitReader<HSEON_A>;
impl HSEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSEON_A {
        match self.bits {
            false => HSEON_A::B_0x0,
            true => HSEON_A::B_0x1,
        }
    }
    #[doc = "HSE is OFF (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSEON_A::B_0x0
    }
    #[doc = "HSE is ON"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSEON_A::B_0x1
    }
}
#[doc = "Field `HSEON` writer - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG, HSEON_A>;
impl<'a, REG> HSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE is OFF (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON_A::B_0x0)
    }
    #[doc = "HSE is ON"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON_A::B_0x1)
    }
}
#[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDY_A {
    #[doc = "0: HSE clock is not ready (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSE clock is ready"]
    B_0x1 = 1,
}
impl From<HSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDY` reader - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable."]
pub type HSERDY_R = crate::BitReader<HSERDY_A>;
impl HSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSERDY_A {
        match self.bits {
            false => HSERDY_A::B_0x0,
            true => HSERDY_A::B_0x1,
        }
    }
    #[doc = "HSE clock is not ready (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSERDY_A::B_0x0
    }
    #[doc = "HSE clock is ready"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSERDY_A::B_0x1
    }
}
#[doc = "HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP_A {
    #[doc = "0: HSE oscillator not bypassed (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSE oscillator bypassed with an external clock"]
    B_0x1 = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYP` reader - HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
pub type HSEBYP_R = crate::BitReader<HSEBYP_A>;
impl HSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::B_0x0,
            true => HSEBYP_A::B_0x1,
        }
    }
    #[doc = "HSE oscillator not bypassed (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSEBYP_A::B_0x0
    }
    #[doc = "HSE oscillator bypassed with an external clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSEBYP_A::B_0x1
    }
}
#[doc = "Field `HSEBYP` writer - HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP_A>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE oscillator not bypassed (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP_A::B_0x0)
    }
    #[doc = "HSE oscillator bypassed with an external clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP_A::B_0x1)
    }
}
#[doc = "HSE clock security system enable Set by software to enable clock security system on HSE. This bit is 'set only' (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSECSSON_A {
    #[doc = "0: CSS on HSE OFF (clock detector OFF) (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: CSS on HSE ON (clock detector ON if the HSE oscillator is stable, OFF if not)."]
    B_0x1 = 1,
}
impl From<HSECSSON_A> for bool {
    #[inline(always)]
    fn from(variant: HSECSSON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSECSSON` reader - HSE clock security system enable Set by software to enable clock security system on HSE. This bit is 'set only' (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
pub type HSECSSON_R = crate::BitReader<HSECSSON_A>;
impl HSECSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSECSSON_A {
        match self.bits {
            false => HSECSSON_A::B_0x0,
            true => HSECSSON_A::B_0x1,
        }
    }
    #[doc = "CSS on HSE OFF (clock detector OFF) (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSECSSON_A::B_0x0
    }
    #[doc = "CSS on HSE ON (clock detector ON if the HSE oscillator is stable, OFF if not)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSECSSON_A::B_0x1
    }
}
#[doc = "Field `HSECSSON` writer - HSE clock security system enable Set by software to enable clock security system on HSE. This bit is 'set only' (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
pub type HSECSSON_W<'a, REG> = crate::BitWriter<'a, REG, HSECSSON_A>;
impl<'a, REG> HSECSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSS on HSE OFF (clock detector OFF) (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSECSSON_A::B_0x0)
    }
    #[doc = "CSS on HSE ON (clock detector ON if the HSE oscillator is stable, OFF if not)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSECSSON_A::B_0x1)
    }
}
#[doc = "external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEEXT_A {
    #[doc = "0: HSE in analog mode (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSE in digital mode"]
    B_0x1 = 1,
}
impl From<HSEEXT_A> for bool {
    #[inline(always)]
    fn from(variant: HSEEXT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEEXT` reader - external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
pub type HSEEXT_R = crate::BitReader<HSEEXT_A>;
impl HSEEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSEEXT_A {
        match self.bits {
            false => HSEEXT_A::B_0x0,
            true => HSEEXT_A::B_0x1,
        }
    }
    #[doc = "HSE in analog mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSEEXT_A::B_0x0
    }
    #[doc = "HSE in digital mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSEEXT_A::B_0x1
    }
}
#[doc = "Field `HSEEXT` writer - external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
pub type HSEEXT_W<'a, REG> = crate::BitWriter<'a, REG, HSEEXT_A>;
impl<'a, REG> HSEEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE in analog mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSEEXT_A::B_0x0)
    }
    #[doc = "HSE in digital mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSEEXT_A::B_0x1)
    }
}
#[doc = "PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1ON_A {
    #[doc = "0: PLL1 OFF (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: PLL1 ON"]
    B_0x1 = 1,
}
impl From<PLL1ON_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1ON` reader - PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock."]
pub type PLL1ON_R = crate::BitReader<PLL1ON_A>;
impl PLL1ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1ON_A {
        match self.bits {
            false => PLL1ON_A::B_0x0,
            true => PLL1ON_A::B_0x1,
        }
    }
    #[doc = "PLL1 OFF (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1ON_A::B_0x0
    }
    #[doc = "PLL1 ON"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1ON_A::B_0x1
    }
}
#[doc = "Field `PLL1ON` writer - PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock."]
pub type PLL1ON_W<'a, REG> = crate::BitWriter<'a, REG, PLL1ON_A>;
impl<'a, REG> PLL1ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL1 OFF (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1ON_A::B_0x0)
    }
    #[doc = "PLL1 ON"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1ON_A::B_0x1)
    }
}
#[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1RDY_A {
    #[doc = "0: PLL1 unlocked (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: PLL1 locked"]
    B_0x1 = 1,
}
impl From<PLL1RDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1RDY` reader - PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
pub type PLL1RDY_R = crate::BitReader<PLL1RDY_A>;
impl PLL1RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1RDY_A {
        match self.bits {
            false => PLL1RDY_A::B_0x0,
            true => PLL1RDY_A::B_0x1,
        }
    }
    #[doc = "PLL1 unlocked (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1RDY_A::B_0x0
    }
    #[doc = "PLL1 locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1RDY_A::B_0x1
    }
}
#[doc = "PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop or Standby mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2ON_A {
    #[doc = "0: PLL2 OFF (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: PLL2 ON"]
    B_0x1 = 1,
}
impl From<PLL2ON_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2ON` reader - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop or Standby mode."]
pub type PLL2ON_R = crate::BitReader<PLL2ON_A>;
impl PLL2ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2ON_A {
        match self.bits {
            false => PLL2ON_A::B_0x0,
            true => PLL2ON_A::B_0x1,
        }
    }
    #[doc = "PLL2 OFF (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2ON_A::B_0x0
    }
    #[doc = "PLL2 ON"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2ON_A::B_0x1
    }
}
#[doc = "Field `PLL2ON` writer - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop or Standby mode."]
pub type PLL2ON_W<'a, REG> = crate::BitWriter<'a, REG, PLL2ON_A>;
impl<'a, REG> PLL2ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL2 OFF (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2ON_A::B_0x0)
    }
    #[doc = "PLL2 ON"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2ON_A::B_0x1)
    }
}
#[doc = "PLL2 clock ready flag Set by hardware to indicate that the PLL is locked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2RDY_A {
    #[doc = "0: PLL2 unlocked"]
    B_0x0 = 0,
    #[doc = "1: PLL2 locked"]
    B_0x1 = 1,
}
impl From<PLL2RDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2RDY` reader - PLL2 clock ready flag Set by hardware to indicate that the PLL is locked."]
pub type PLL2RDY_R = crate::BitReader<PLL2RDY_A>;
impl PLL2RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2RDY_A {
        match self.bits {
            false => PLL2RDY_A::B_0x0,
            true => PLL2RDY_A::B_0x1,
        }
    }
    #[doc = "PLL2 unlocked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2RDY_A::B_0x0
    }
    #[doc = "PLL2 locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2RDY_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    pub fn HSION(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable."]
    #[inline(always)]
    pub fn HSIRDY(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
    #[inline(always)]
    pub fn HSIKERON(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
    #[inline(always)]
    pub fn HSIDIV(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV."]
    #[inline(always)]
    pub fn HSIDIVF(&self) -> HSIDIVF_R {
        HSIDIVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    pub fn CSION(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSI clock ready flag Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request)."]
    #[inline(always)]
    pub fn CSIRDY(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
    #[inline(always)]
    pub fn CSIKERON(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
    #[inline(always)]
    pub fn HSI48ON(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HSI48 clock ready flag Set by hardware to indicate that the HSI48 oscillator is stable."]
    #[inline(always)]
    pub fn HSI48RDY(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    pub fn HSEON(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable."]
    #[inline(always)]
    pub fn HSERDY(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub fn HSEBYP(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE clock security system enable Set by software to enable clock security system on HSE. This bit is 'set only' (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
    #[inline(always)]
    pub fn HSECSSON(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub fn HSEEXT(&self) -> HSEEXT_R {
        HSEEXT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock."]
    #[inline(always)]
    pub fn PLL1ON(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
    #[inline(always)]
    pub fn PLL1RDY(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop or Standby mode."]
    #[inline(always)]
    pub fn PLL2ON(&self) -> PLL2ON_R {
        PLL2ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL2 clock ready flag Set by hardware to indicate that the PLL is locked."]
    #[inline(always)]
    pub fn PLL2RDY(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    pub fn HSION(&mut self) -> HSION_W<'_, CR_SPEC> {
        HSION_W::new(self, 0)
    }
    #[doc = "Bit 2 - HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
    #[inline(always)]
    pub fn HSIKERON(&mut self) -> HSIKERON_W<'_, CR_SPEC> {
        HSIKERON_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
    #[inline(always)]
    pub fn HSIDIV(&mut self) -> HSIDIV_W<'_, CR_SPEC> {
        HSIDIV_W::new(self, 3)
    }
    #[doc = "Bit 8 - CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    pub fn CSION(&mut self) -> CSION_W<'_, CR_SPEC> {
        CSION_W::new(self, 8)
    }
    #[doc = "Bit 10 - CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
    #[inline(always)]
    pub fn CSIKERON(&mut self) -> CSIKERON_W<'_, CR_SPEC> {
        CSIKERON_W::new(self, 10)
    }
    #[doc = "Bit 12 - HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
    #[inline(always)]
    pub fn HSI48ON(&mut self) -> HSI48ON_W<'_, CR_SPEC> {
        HSI48ON_W::new(self, 12)
    }
    #[doc = "Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
    #[inline(always)]
    pub fn HSEON(&mut self) -> HSEON_W<'_, CR_SPEC> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 18 - HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub fn HSEBYP(&mut self) -> HSEBYP_W<'_, CR_SPEC> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSE clock security system enable Set by software to enable clock security system on HSE. This bit is 'set only' (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
    #[inline(always)]
    pub fn HSECSSON(&mut self) -> HSECSSON_W<'_, CR_SPEC> {
        HSECSSON_W::new(self, 19)
    }
    #[doc = "Bit 20 - external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub fn HSEEXT(&mut self) -> HSEEXT_W<'_, CR_SPEC> {
        HSEEXT_W::new(self, 20)
    }
    #[doc = "Bit 24 - PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock."]
    #[inline(always)]
    pub fn PLL1ON(&mut self) -> PLL1ON_W<'_, CR_SPEC> {
        PLL1ON_W::new(self, 24)
    }
    #[doc = "Bit 26 - PLL2 enable Set and cleared by software to enable PLL2. Cleared by hardware when entering Stop or Standby mode."]
    #[inline(always)]
    pub fn PLL2ON(&mut self) -> PLL2ON_W<'_, CR_SPEC> {
        PLL2ON_W::new(self, 26)
    }
}
#[doc = "RCC clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0x23"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x23;
}
