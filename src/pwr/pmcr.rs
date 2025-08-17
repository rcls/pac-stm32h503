#[doc = "Register `PMCR` reader"]
pub type R = crate::R<PMCR_SPEC>;
#[doc = "Register `PMCR` writer"]
pub type W = crate::W<PMCR_SPEC>;
#[doc = "low-power mode selection This bit defines the Deepsleep mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMS_A {
    #[doc = "0: Keeps Stop mode when entering DeepSleep."]
    B_0x0 = 0,
    #[doc = "1: Allows Standby mode when entering DeepSleep."]
    B_0x1 = 1,
}
impl From<LPMS_A> for bool {
    #[inline(always)]
    fn from(variant: LPMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMS` reader - low-power mode selection This bit defines the Deepsleep mode."]
pub type LPMS_R = crate::BitReader<LPMS_A>;
impl LPMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPMS_A {
        match self.bits {
            false => LPMS_A::B_0x0,
            true => LPMS_A::B_0x1,
        }
    }
    #[doc = "Keeps Stop mode when entering DeepSleep."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPMS_A::B_0x0
    }
    #[doc = "Allows Standby mode when entering DeepSleep."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPMS_A::B_0x1
    }
}
#[doc = "Field `LPMS` writer - low-power mode selection This bit defines the Deepsleep mode."]
pub type LPMS_W<'a, REG> = crate::BitWriter<'a, REG, LPMS_A>;
impl<'a, REG> LPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Keeps Stop mode when entering DeepSleep."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS_A::B_0x0)
    }
    #[doc = "Allows Standby mode when entering DeepSleep."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS_A::B_0x1)
    }
}
#[doc = "system Stop mode voltage scaling selection These bits control the V CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SVOS_A {
    #[doc = "1: SVOS5 scale 5"]
    B_0x1 = 1,
    #[doc = "2: SVOS4 scale 4"]
    B_0x2 = 2,
    #[doc = "3: SVOS3 scale 3 (default)."]
    B_0x3 = 3,
}
impl From<SVOS_A> for u8 {
    #[inline(always)]
    fn from(variant: SVOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SVOS_A {
    type Ux = u8;
}
impl crate::IsEnum for SVOS_A {}
#[doc = "Field `SVOS` reader - system Stop mode voltage scaling selection These bits control the V CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
pub type SVOS_R = crate::FieldReader<SVOS_A>;
impl SVOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SVOS_A> {
        match self.bits {
            1 => Some(SVOS_A::B_0x1),
            2 => Some(SVOS_A::B_0x2),
            3 => Some(SVOS_A::B_0x3),
            _ => None,
        }
    }
    #[doc = "SVOS5 scale 5"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SVOS_A::B_0x1
    }
    #[doc = "SVOS4 scale 4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SVOS_A::B_0x2
    }
    #[doc = "SVOS3 scale 3 (default)."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SVOS_A::B_0x3
    }
}
#[doc = "Field `SVOS` writer - system Stop mode voltage scaling selection These bits control the V CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
pub type SVOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SVOS_A>;
impl<'a, REG> SVOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SVOS5 scale 5"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SVOS_A::B_0x1)
    }
    #[doc = "SVOS4 scale 4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SVOS_A::B_0x2)
    }
    #[doc = "SVOS3 scale 3 (default)."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SVOS_A::B_0x3)
    }
}
#[doc = "clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: STOPF and SBF flags cleared."]
    B_0x1 = 1,
}
impl From<CSSF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSF` reader - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CSSF_R = crate::BitReader<CSSF_A>;
impl CSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSF_A {
        match self.bits {
            false => CSSF_A::B_0x0,
            true => CSSF_A::B_0x1,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CSSF_A::B_0x0
    }
    #[doc = "STOPF and SBF flags cleared."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CSSF_A::B_0x1
    }
}
#[doc = "Field `CSSF` writer - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG, CSSF_A>;
impl<'a, REG> CSSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSSF_A::B_0x0)
    }
    #[doc = "STOPF and SBF flags cleared."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSSF_A::B_0x1)
    }
}
#[doc = "Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLPS_A {
    #[doc = "0: Flash memory remains in normal mode when the CPU domain enters Stop mode (quick restart time)."]
    B_0x0 = 0,
    #[doc = "1: Flash memory enters low-power mode when the CPU domain enters Stop mode (low power consumption)."]
    B_0x1 = 1,
}
impl From<FLPS_A> for bool {
    #[inline(always)]
    fn from(variant: FLPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLPS` reader - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode."]
pub type FLPS_R = crate::BitReader<FLPS_A>;
impl FLPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLPS_A {
        match self.bits {
            false => FLPS_A::B_0x0,
            true => FLPS_A::B_0x1,
        }
    }
    #[doc = "Flash memory remains in normal mode when the CPU domain enters Stop mode (quick restart time)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FLPS_A::B_0x0
    }
    #[doc = "Flash memory enters low-power mode when the CPU domain enters Stop mode (low power consumption)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FLPS_A::B_0x1
    }
}
#[doc = "Field `FLPS` writer - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode."]
pub type FLPS_W<'a, REG> = crate::BitWriter<'a, REG, FLPS_A>;
impl<'a, REG> FLPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash memory remains in normal mode when the CPU domain enters Stop mode (quick restart time)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FLPS_A::B_0x0)
    }
    #[doc = "Flash memory enters low-power mode when the CPU domain enters Stop mode (low power consumption)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FLPS_A::B_0x1)
    }
}
#[doc = "analog switch V BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V DD supply voltage can be monitored through the PVD and the PLS bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTE_A {
    #[doc = "0: booster disabled (default)"]
    B_0x0 = 0,
    #[doc = "1: booster enabled if analog voltage ready (AVD_READY = 1)."]
    B_0x1 = 1,
}
impl From<BOOSTE_A> for bool {
    #[inline(always)]
    fn from(variant: BOOSTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOSTE` reader - analog switch V BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V DD supply voltage can be monitored through the PVD and the PLS bits."]
pub type BOOSTE_R = crate::BitReader<BOOSTE_A>;
impl BOOSTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOOSTE_A {
        match self.bits {
            false => BOOSTE_A::B_0x0,
            true => BOOSTE_A::B_0x1,
        }
    }
    #[doc = "booster disabled (default)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BOOSTE_A::B_0x0
    }
    #[doc = "booster enabled if analog voltage ready (AVD_READY = 1)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BOOSTE_A::B_0x1
    }
}
#[doc = "Field `BOOSTE` writer - analog switch V BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V DD supply voltage can be monitored through the PVD and the PLS bits."]
pub type BOOSTE_W<'a, REG> = crate::BitWriter<'a, REG, BOOSTE_A>;
impl<'a, REG> BOOSTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "booster disabled (default)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTE_A::B_0x0)
    }
    #[doc = "booster enabled if analog voltage ready (AVD_READY = 1)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTE_A::B_0x1)
    }
}
#[doc = "analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVD_READY_A {
    #[doc = "0: peripheral analog voltage V DDA not ready (default)"]
    B_0x0 = 0,
    #[doc = "1: peripheral analog voltage V DDA ready."]
    B_0x1 = 1,
}
impl From<AVD_READY_A> for bool {
    #[inline(always)]
    fn from(variant: AVD_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVD_READY` reader - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits)."]
pub type AVD_READY_R = crate::BitReader<AVD_READY_A>;
impl AVD_READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AVD_READY_A {
        match self.bits {
            false => AVD_READY_A::B_0x0,
            true => AVD_READY_A::B_0x1,
        }
    }
    #[doc = "peripheral analog voltage V DDA not ready (default)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AVD_READY_A::B_0x0
    }
    #[doc = "peripheral analog voltage V DDA ready."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AVD_READY_A::B_0x1
    }
}
#[doc = "Field `AVD_READY` writer - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits)."]
pub type AVD_READY_W<'a, REG> = crate::BitWriter<'a, REG, AVD_READY_A>;
impl<'a, REG> AVD_READY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "peripheral analog voltage V DDA not ready (default)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AVD_READY_A::B_0x0)
    }
    #[doc = "peripheral analog voltage V DDA ready."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AVD_READY_A::B_0x1)
    }
}
#[doc = "AHB SRAM2 shut-off in Stop mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2SO_A {
    #[doc = "0: AHB RAM2 content is kept in Stop mode."]
    B_0x0 = 0,
    #[doc = "1: AHB RAM2 content is lost in Stop mode."]
    B_0x1 = 1,
}
impl From<SRAM2SO_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2SO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2SO` reader - AHB SRAM2 shut-off in Stop mode."]
pub type SRAM2SO_R = crate::BitReader<SRAM2SO_A>;
impl SRAM2SO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2SO_A {
        match self.bits {
            false => SRAM2SO_A::B_0x0,
            true => SRAM2SO_A::B_0x1,
        }
    }
    #[doc = "AHB RAM2 content is kept in Stop mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM2SO_A::B_0x0
    }
    #[doc = "AHB RAM2 content is lost in Stop mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM2SO_A::B_0x1
    }
}
#[doc = "Field `SRAM2SO` writer - AHB SRAM2 shut-off in Stop mode."]
pub type SRAM2SO_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2SO_A>;
impl<'a, REG> SRAM2SO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB RAM2 content is kept in Stop mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2SO_A::B_0x0)
    }
    #[doc = "AHB RAM2 content is lost in Stop mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2SO_A::B_0x1)
    }
}
#[doc = "AHB SRAM1 shut-off in Stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1SO_A {
    #[doc = "0: AHB RAM1 content is kept in Stop mode."]
    B_0x0 = 0,
    #[doc = "1: AHB RAM1 content is lost in Stop mode."]
    B_0x1 = 1,
}
impl From<SRAM1SO_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1SO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1SO` reader - AHB SRAM1 shut-off in Stop mode"]
pub type SRAM1SO_R = crate::BitReader<SRAM1SO_A>;
impl SRAM1SO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1SO_A {
        match self.bits {
            false => SRAM1SO_A::B_0x0,
            true => SRAM1SO_A::B_0x1,
        }
    }
    #[doc = "AHB RAM1 content is kept in Stop mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM1SO_A::B_0x0
    }
    #[doc = "AHB RAM1 content is lost in Stop mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM1SO_A::B_0x1
    }
}
#[doc = "Field `SRAM1SO` writer - AHB SRAM1 shut-off in Stop mode"]
pub type SRAM1SO_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1SO_A>;
impl<'a, REG> SRAM1SO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB RAM1 content is kept in Stop mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1SO_A::B_0x0)
    }
    #[doc = "AHB RAM1 content is lost in Stop mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1SO_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - low-power mode selection This bit defines the Deepsleep mode."]
    #[inline(always)]
    pub fn LPMS(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - system Stop mode voltage scaling selection These bits control the V CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
    #[inline(always)]
    pub fn SVOS(&self) -> SVOS_R {
        SVOS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn CSSF(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode."]
    #[inline(always)]
    pub fn FLPS(&self) -> FLPS_R {
        FLPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - analog switch V BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V DD supply voltage can be monitored through the PVD and the PLS bits."]
    #[inline(always)]
    pub fn BOOSTE(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits)."]
    #[inline(always)]
    pub fn AVD_READY(&self) -> AVD_READY_R {
        AVD_READY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 25 - AHB SRAM2 shut-off in Stop mode."]
    #[inline(always)]
    pub fn SRAM2SO(&self) -> SRAM2SO_R {
        SRAM2SO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - AHB SRAM1 shut-off in Stop mode"]
    #[inline(always)]
    pub fn SRAM1SO(&self) -> SRAM1SO_R {
        SRAM1SO_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - low-power mode selection This bit defines the Deepsleep mode."]
    #[inline(always)]
    pub fn LPMS(&mut self) -> LPMS_W<'_, PMCR_SPEC> {
        LPMS_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - system Stop mode voltage scaling selection These bits control the V CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
    #[inline(always)]
    pub fn SVOS(&mut self) -> SVOS_W<'_, PMCR_SPEC> {
        SVOS_W::new(self, 2)
    }
    #[doc = "Bit 7 - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn CSSF(&mut self) -> CSSF_W<'_, PMCR_SPEC> {
        CSSF_W::new(self, 7)
    }
    #[doc = "Bit 9 - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode."]
    #[inline(always)]
    pub fn FLPS(&mut self) -> FLPS_W<'_, PMCR_SPEC> {
        FLPS_W::new(self, 9)
    }
    #[doc = "Bit 12 - analog switch V BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V DD supply voltage can be monitored through the PVD and the PLS bits."]
    #[inline(always)]
    pub fn BOOSTE(&mut self) -> BOOSTE_W<'_, PMCR_SPEC> {
        BOOSTE_W::new(self, 12)
    }
    #[doc = "Bit 13 - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits)."]
    #[inline(always)]
    pub fn AVD_READY(&mut self) -> AVD_READY_W<'_, PMCR_SPEC> {
        AVD_READY_W::new(self, 13)
    }
    #[doc = "Bit 25 - AHB SRAM2 shut-off in Stop mode."]
    #[inline(always)]
    pub fn SRAM2SO(&mut self) -> SRAM2SO_W<'_, PMCR_SPEC> {
        SRAM2SO_W::new(self, 25)
    }
    #[doc = "Bit 26 - AHB SRAM1 shut-off in Stop mode"]
    #[inline(always)]
    pub fn SRAM1SO(&mut self) -> SRAM1SO_W<'_, PMCR_SPEC> {
        SRAM1SO_W::new(self, 26)
    }
}
#[doc = "PWR power mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMCR_SPEC;
impl crate::RegisterSpec for PMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmcr::R`](R) reader structure"]
impl crate::Readable for PMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmcr::W`](W) writer structure"]
impl crate::Writable for PMCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PMCR to value 0x0c"]
impl crate::Resettable for PMCR_SPEC {
    const RESET_VALUE: u32 = 0x0c;
}
