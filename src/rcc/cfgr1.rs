#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1_SPEC>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1_SPEC>;
#[doc = "system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW_A {
    #[doc = "0: HSI selected as system clock (hsi_ck) (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: CSI selected as system clock (csi_ck)"]
    B_0x1 = 1,
    #[doc = "2: HSE selected as system clock (hse_ck)"]
    B_0x2 = 2,
    #[doc = "3: PLL1 selected as system clock (pll1_p_ck for sys_ck)"]
    B_0x3 = 3,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SW_A {
    type Ux = u8;
}
impl crate::IsEnum for SW_A {}
#[doc = "Field `SW` reader - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
pub type SW_R = crate::FieldReader<SW_A>;
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SW_A> {
        match self.bits {
            0 => Some(SW_A::B_0x0),
            1 => Some(SW_A::B_0x1),
            2 => Some(SW_A::B_0x2),
            3 => Some(SW_A::B_0x3),
            _ => None,
        }
    }
    #[doc = "HSI selected as system clock (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SW_A::B_0x0
    }
    #[doc = "CSI selected as system clock (csi_ck)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SW_A::B_0x1
    }
    #[doc = "HSE selected as system clock (hse_ck)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SW_A::B_0x2
    }
    #[doc = "PLL1 selected as system clock (pll1_p_ck for sys_ck)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SW_A::B_0x3
    }
}
#[doc = "Field `SW` writer - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SW_A>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI selected as system clock (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SW_A::B_0x0)
    }
    #[doc = "CSI selected as system clock (csi_ck)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SW_A::B_0x1)
    }
    #[doc = "HSE selected as system clock (hse_ck)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SW_A::B_0x2)
    }
    #[doc = "PLL1 selected as system clock (pll1_p_ck for sys_ck)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SW_A::B_0x3)
    }
}
#[doc = "system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset). others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWS_A {
    #[doc = "1: CSI used as system clock (csi_ck)"]
    B_0x1 = 1,
    #[doc = "2: HSE used as system clock (hse_ck)"]
    B_0x2 = 2,
    #[doc = "3: PLL1 used as system clock (pll1_p_ck)"]
    B_0x3 = 3,
}
impl From<SWS_A> for u8 {
    #[inline(always)]
    fn from(variant: SWS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWS_A {
    type Ux = u8;
}
impl crate::IsEnum for SWS_A {}
#[doc = "Field `SWS` reader - system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset). others: reserved"]
pub type SWS_R = crate::FieldReader<SWS_A>;
impl SWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWS_A> {
        match self.bits {
            1 => Some(SWS_A::B_0x1),
            2 => Some(SWS_A::B_0x2),
            3 => Some(SWS_A::B_0x3),
            _ => None,
        }
    }
    #[doc = "CSI used as system clock (csi_ck)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWS_A::B_0x1
    }
    #[doc = "HSE used as system clock (hse_ck)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SWS_A::B_0x2
    }
    #[doc = "PLL1 used as system clock (pll1_p_ck)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SWS_A::B_0x3
    }
}
#[doc = "system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK_A {
    #[doc = "1: CSI selected as wakeup clock from system Stop"]
    B_0x1 = 1,
}
impl From<STOPWUCK_A> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPWUCK` reader - system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
pub type STOPWUCK_R = crate::BitReader<STOPWUCK_A>;
impl STOPWUCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STOPWUCK_A> {
        match self.bits {
            true => Some(STOPWUCK_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "CSI selected as wakeup clock from system Stop"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STOPWUCK_A::B_0x1
    }
}
#[doc = "Field `STOPWUCK` writer - system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG, STOPWUCK_A>;
impl<'a, REG> STOPWUCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSI selected as wakeup clock from system Stop"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(STOPWUCK_A::B_0x1)
    }
}
#[doc = "kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPKERWUCK_A {
    #[doc = "0: HSI selected as wakeup clock from system Stop (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: CSI selected as wakeup clock from system Stop"]
    B_0x1 = 1,
}
impl From<STOPKERWUCK_A> for bool {
    #[inline(always)]
    fn from(variant: STOPKERWUCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPKERWUCK` reader - kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop."]
pub type STOPKERWUCK_R = crate::BitReader<STOPKERWUCK_A>;
impl STOPKERWUCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPKERWUCK_A {
        match self.bits {
            false => STOPKERWUCK_A::B_0x0,
            true => STOPKERWUCK_A::B_0x1,
        }
    }
    #[doc = "HSI selected as wakeup clock from system Stop (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STOPKERWUCK_A::B_0x0
    }
    #[doc = "CSI selected as wakeup clock from system Stop"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STOPKERWUCK_A::B_0x1
    }
}
#[doc = "Field `STOPKERWUCK` writer - kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop."]
pub type STOPKERWUCK_W<'a, REG> = crate::BitWriter<'a, REG, STOPKERWUCK_A>;
impl<'a, REG> STOPKERWUCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI selected as wakeup clock from system Stop (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(STOPKERWUCK_A::B_0x0)
    }
    #[doc = "CSI selected as wakeup clock from system Stop"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(STOPKERWUCK_A::B_0x1)
    }
}
#[doc = "HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCPRE_A {
    #[doc = "0: no clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: no clock"]
    B_0x1 = 1,
    #[doc = "2: HSE/2"]
    B_0x2 = 2,
    #[doc = "3: HSE/3"]
    B_0x3 = 3,
    #[doc = "4: HSE/4"]
    B_0x4 = 4,
    #[doc = "62: HSE/62"]
    B_0x3E = 62,
    #[doc = "63: HSE/63"]
    B_0x3F = 63,
}
impl From<RTCPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPRE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCPRE_A {
    type Ux = u8;
}
impl crate::IsEnum for RTCPRE_A {}
#[doc = "Field `RTCPRE` reader - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
pub type RTCPRE_R = crate::FieldReader<RTCPRE_A>;
impl RTCPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTCPRE_A> {
        match self.bits {
            0 => Some(RTCPRE_A::B_0x0),
            1 => Some(RTCPRE_A::B_0x1),
            2 => Some(RTCPRE_A::B_0x2),
            3 => Some(RTCPRE_A::B_0x3),
            4 => Some(RTCPRE_A::B_0x4),
            62 => Some(RTCPRE_A::B_0x3E),
            63 => Some(RTCPRE_A::B_0x3F),
            _ => None,
        }
    }
    #[doc = "no clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RTCPRE_A::B_0x0
    }
    #[doc = "no clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RTCPRE_A::B_0x1
    }
    #[doc = "HSE/2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == RTCPRE_A::B_0x2
    }
    #[doc = "HSE/3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == RTCPRE_A::B_0x3
    }
    #[doc = "HSE/4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == RTCPRE_A::B_0x4
    }
    #[doc = "HSE/62"]
    #[inline(always)]
    pub fn is_B_0x3E(&self) -> bool {
        *self == RTCPRE_A::B_0x3E
    }
    #[doc = "HSE/63"]
    #[inline(always)]
    pub fn is_B_0x3F(&self) -> bool {
        *self == RTCPRE_A::B_0x3F
    }
}
#[doc = "Field `RTCPRE` writer - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
pub type RTCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 6, RTCPRE_A>;
impl<'a, REG> RTCPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE_A::B_0x0)
    }
    #[doc = "no clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE_A::B_0x1)
    }
    #[doc = "HSE/2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE_A::B_0x2)
    }
    #[doc = "HSE/3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE_A::B_0x3)
    }
    #[doc = "HSE/4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE_A::B_0x4)
    }
    #[doc = "HSE/62"]
    #[inline(always)]
    pub fn B_0x3E(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE_A::B_0x3E)
    }
    #[doc = "HSE/63"]
    #[inline(always)]
    pub fn B_0x3F(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE_A::B_0x3F)
    }
}
#[doc = "timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE_A {
    #[doc = "0: The timers kernel clock is equal to rcc_hclk1 if PPRE1 or PPRE2 corresponds to a division by 1 or 2, else it is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: The timers kernel clock is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 if PPRE1 or PPRE2 corresponds to a division by 1, 2 or 4, else it is equal to 4 x Frcc_pclk1 or 4 x Frcc_pclk2"]
    B_0x1 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMPRE` reader - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains."]
pub type TIMPRE_R = crate::BitReader<TIMPRE_A>;
impl TIMPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::B_0x0,
            true => TIMPRE_A::B_0x1,
        }
    }
    #[doc = "The timers kernel clock is equal to rcc_hclk1 if PPRE1 or PPRE2 corresponds to a division by 1 or 2, else it is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIMPRE_A::B_0x0
    }
    #[doc = "The timers kernel clock is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 if PPRE1 or PPRE2 corresponds to a division by 1, 2 or 4, else it is equal to 4 x Frcc_pclk1 or 4 x Frcc_pclk2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIMPRE_A::B_0x1
    }
}
#[doc = "Field `TIMPRE` writer - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains."]
pub type TIMPRE_W<'a, REG> = crate::BitWriter<'a, REG, TIMPRE_A>;
impl<'a, REG> TIMPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timers kernel clock is equal to rcc_hclk1 if PPRE1 or PPRE2 corresponds to a division by 1 or 2, else it is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE_A::B_0x0)
    }
    #[doc = "The timers kernel clock is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 if PPRE1 or PPRE2 corresponds to a division by 1, 2 or 4, else it is equal to 4 x Frcc_pclk1 or 4 x Frcc_pclk2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIMPRE_A::B_0x1)
    }
}
#[doc = "MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1PRE_A {
    #[doc = "0: prescaler disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: division by 1 (bypass)"]
    B_0x1 = 1,
    #[doc = "2: division by 2"]
    B_0x2 = 2,
    #[doc = "3: division by 3"]
    B_0x3 = 3,
    #[doc = "4: division by 4"]
    B_0x4 = 4,
    #[doc = "15: division by 15"]
    B_0xF = 15,
}
impl From<MCO1PRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO1PRE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO1PRE_A {
    type Ux = u8;
}
impl crate::IsEnum for MCO1PRE_A {}
#[doc = "Field `MCO1PRE` reader - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO1PRE_R = crate::FieldReader<MCO1PRE_A>;
impl MCO1PRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO1PRE_A> {
        match self.bits {
            0 => Some(MCO1PRE_A::B_0x0),
            1 => Some(MCO1PRE_A::B_0x1),
            2 => Some(MCO1PRE_A::B_0x2),
            3 => Some(MCO1PRE_A::B_0x3),
            4 => Some(MCO1PRE_A::B_0x4),
            15 => Some(MCO1PRE_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "prescaler disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MCO1PRE_A::B_0x0
    }
    #[doc = "division by 1 (bypass)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MCO1PRE_A::B_0x1
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MCO1PRE_A::B_0x2
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MCO1PRE_A::B_0x3
    }
    #[doc = "division by 4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MCO1PRE_A::B_0x4
    }
    #[doc = "division by 15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == MCO1PRE_A::B_0xF
    }
}
#[doc = "Field `MCO1PRE` writer - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO1PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCO1PRE_A>;
impl<'a, REG> MCO1PRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "prescaler disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE_A::B_0x0)
    }
    #[doc = "division by 1 (bypass)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE_A::B_0x1)
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE_A::B_0x2)
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE_A::B_0x3)
    }
    #[doc = "division by 4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE_A::B_0x4)
    }
    #[doc = "division by 15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1PRE_A::B_0xF)
    }
}
#[doc = "Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1SEL_A {
    #[doc = "0: HSI clock selected (hsi_ck) (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: LSE oscillator clock selected (lse_ck)"]
    B_0x1 = 1,
    #[doc = "2: HSE clock selected (hse_ck)"]
    B_0x2 = 2,
    #[doc = "3: PLL1 clock selected (pll1_q_ck)"]
    B_0x3 = 3,
    #[doc = "4: HSI48 clock selected (hsi48_ck)"]
    B_0x4 = 4,
}
impl From<MCO1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO1SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for MCO1SEL_A {}
#[doc = "Field `MCO1SEL` reader - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO1SEL_R = crate::FieldReader<MCO1SEL_A>;
impl MCO1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO1SEL_A> {
        match self.bits {
            0 => Some(MCO1SEL_A::B_0x0),
            1 => Some(MCO1SEL_A::B_0x1),
            2 => Some(MCO1SEL_A::B_0x2),
            3 => Some(MCO1SEL_A::B_0x3),
            4 => Some(MCO1SEL_A::B_0x4),
            _ => None,
        }
    }
    #[doc = "HSI clock selected (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MCO1SEL_A::B_0x0
    }
    #[doc = "LSE oscillator clock selected (lse_ck)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MCO1SEL_A::B_0x1
    }
    #[doc = "HSE clock selected (hse_ck)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MCO1SEL_A::B_0x2
    }
    #[doc = "PLL1 clock selected (pll1_q_ck)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MCO1SEL_A::B_0x3
    }
    #[doc = "HSI48 clock selected (hsi48_ck)"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MCO1SEL_A::B_0x4
    }
}
#[doc = "Field `MCO1SEL` writer - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCO1SEL_A>;
impl<'a, REG> MCO1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI clock selected (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL_A::B_0x0)
    }
    #[doc = "LSE oscillator clock selected (lse_ck)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL_A::B_0x1)
    }
    #[doc = "HSE clock selected (hse_ck)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL_A::B_0x2)
    }
    #[doc = "PLL1 clock selected (pll1_q_ck)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL_A::B_0x3)
    }
    #[doc = "HSI48 clock selected (hsi48_ck)"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MCO1SEL_A::B_0x4)
    }
}
#[doc = "MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2PRE_A {
    #[doc = "0: prescaler disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: division by 1 (bypass)"]
    B_0x1 = 1,
    #[doc = "2: division by 2"]
    B_0x2 = 2,
    #[doc = "3: division by 3"]
    B_0x3 = 3,
    #[doc = "4: division by 4"]
    B_0x4 = 4,
    #[doc = "15: division by 15"]
    B_0xF = 15,
}
impl From<MCO2PRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO2PRE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO2PRE_A {
    type Ux = u8;
}
impl crate::IsEnum for MCO2PRE_A {}
#[doc = "Field `MCO2PRE` reader - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO2PRE_R = crate::FieldReader<MCO2PRE_A>;
impl MCO2PRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO2PRE_A> {
        match self.bits {
            0 => Some(MCO2PRE_A::B_0x0),
            1 => Some(MCO2PRE_A::B_0x1),
            2 => Some(MCO2PRE_A::B_0x2),
            3 => Some(MCO2PRE_A::B_0x3),
            4 => Some(MCO2PRE_A::B_0x4),
            15 => Some(MCO2PRE_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "prescaler disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MCO2PRE_A::B_0x0
    }
    #[doc = "division by 1 (bypass)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MCO2PRE_A::B_0x1
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MCO2PRE_A::B_0x2
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MCO2PRE_A::B_0x3
    }
    #[doc = "division by 4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MCO2PRE_A::B_0x4
    }
    #[doc = "division by 15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == MCO2PRE_A::B_0xF
    }
}
#[doc = "Field `MCO2PRE` writer - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCO2PRE_A>;
impl<'a, REG> MCO2PRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "prescaler disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE_A::B_0x0)
    }
    #[doc = "division by 1 (bypass)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE_A::B_0x1)
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE_A::B_0x2)
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE_A::B_0x3)
    }
    #[doc = "division by 4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE_A::B_0x4)
    }
    #[doc = "division by 15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE_A::B_0xF)
    }
}
#[doc = "microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2SEL_A {
    #[doc = "0: system clock selected (sys_ck) (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: PLL2 oscillator clock selected (pll2_p_ck)"]
    B_0x1 = 1,
    #[doc = "2: HSE clock selected (hse_ck)"]
    B_0x2 = 2,
    #[doc = "3: PLL1 clock selected (pll1_p_ck)"]
    B_0x3 = 3,
    #[doc = "4: CSI clock selected (csi_ck)"]
    B_0x4 = 4,
    #[doc = "5: LSI clock selected (lsi_ck)"]
    B_0x5 = 5,
}
impl From<MCO2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO2SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO2SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for MCO2SEL_A {}
#[doc = "Field `MCO2SEL` reader - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO2SEL_R = crate::FieldReader<MCO2SEL_A>;
impl MCO2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO2SEL_A> {
        match self.bits {
            0 => Some(MCO2SEL_A::B_0x0),
            1 => Some(MCO2SEL_A::B_0x1),
            2 => Some(MCO2SEL_A::B_0x2),
            3 => Some(MCO2SEL_A::B_0x3),
            4 => Some(MCO2SEL_A::B_0x4),
            5 => Some(MCO2SEL_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "system clock selected (sys_ck) (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MCO2SEL_A::B_0x0
    }
    #[doc = "PLL2 oscillator clock selected (pll2_p_ck)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MCO2SEL_A::B_0x1
    }
    #[doc = "HSE clock selected (hse_ck)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MCO2SEL_A::B_0x2
    }
    #[doc = "PLL1 clock selected (pll1_p_ck)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MCO2SEL_A::B_0x3
    }
    #[doc = "CSI clock selected (csi_ck)"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MCO2SEL_A::B_0x4
    }
    #[doc = "LSI clock selected (lsi_ck)"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == MCO2SEL_A::B_0x5
    }
}
#[doc = "Field `MCO2SEL` writer - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCO2SEL_A>;
impl<'a, REG> MCO2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "system clock selected (sys_ck) (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL_A::B_0x0)
    }
    #[doc = "PLL2 oscillator clock selected (pll2_p_ck)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL_A::B_0x1)
    }
    #[doc = "HSE clock selected (hse_ck)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL_A::B_0x2)
    }
    #[doc = "PLL1 clock selected (pll1_p_ck)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL_A::B_0x3)
    }
    #[doc = "CSI clock selected (csi_ck)"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL_A::B_0x4)
    }
    #[doc = "LSI clock selected (lsi_ck)"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL_A::B_0x5)
    }
}
impl R {
    #[doc = "Bits 0:2 - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
    #[inline(always)]
    pub fn SW(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset). others: reserved"]
    #[inline(always)]
    pub fn SWS(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
    #[inline(always)]
    pub fn STOPWUCK(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop."]
    #[inline(always)]
    pub fn STOPKERWUCK(&self) -> STOPKERWUCK_R {
        STOPKERWUCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
    #[inline(always)]
    pub fn RTCPRE(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains."]
    #[inline(always)]
    pub fn TIMPRE(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:21 - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    pub fn MCO1PRE(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:24 - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    pub fn MCO1SEL(&self) -> MCO1SEL_R {
        MCO1SEL_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:28 - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    pub fn MCO2PRE(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    pub fn MCO2SEL(&self) -> MCO2SEL_R {
        MCO2SEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
    #[inline(always)]
    pub fn SW(&mut self) -> SW_W<'_, CFGR1_SPEC> {
        SW_W::new(self, 0)
    }
    #[doc = "Bit 6 - system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
    #[inline(always)]
    pub fn STOPWUCK(&mut self) -> STOPWUCK_W<'_, CFGR1_SPEC> {
        STOPWUCK_W::new(self, 6)
    }
    #[doc = "Bit 7 - kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop."]
    #[inline(always)]
    pub fn STOPKERWUCK(&mut self) -> STOPKERWUCK_W<'_, CFGR1_SPEC> {
        STOPKERWUCK_W::new(self, 7)
    }
    #[doc = "Bits 8:13 - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
    #[inline(always)]
    pub fn RTCPRE(&mut self) -> RTCPRE_W<'_, CFGR1_SPEC> {
        RTCPRE_W::new(self, 8)
    }
    #[doc = "Bit 15 - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains."]
    #[inline(always)]
    pub fn TIMPRE(&mut self) -> TIMPRE_W<'_, CFGR1_SPEC> {
        TIMPRE_W::new(self, 15)
    }
    #[doc = "Bits 18:21 - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    pub fn MCO1PRE(&mut self) -> MCO1PRE_W<'_, CFGR1_SPEC> {
        MCO1PRE_W::new(self, 18)
    }
    #[doc = "Bits 22:24 - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    pub fn MCO1SEL(&mut self) -> MCO1SEL_W<'_, CFGR1_SPEC> {
        MCO1SEL_W::new(self, 22)
    }
    #[doc = "Bits 25:28 - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    pub fn MCO2PRE(&mut self) -> MCO2PRE_W<'_, CFGR1_SPEC> {
        MCO2PRE_W::new(self, 25)
    }
    #[doc = "Bits 29:31 - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    pub fn MCO2SEL(&mut self) -> MCO2SEL_W<'_, CFGR1_SPEC> {
        MCO2SEL_W::new(self, 29)
    }
}
#[doc = "RCC clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {}
