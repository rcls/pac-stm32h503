#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BDCR_SPEC>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BDCR_SPEC>;
#[doc = "LSE oscillator enabled Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON_A {
    #[doc = "0: LSE oscillator OFF (default after Backup domain reset)"]
    B_0x0 = 0,
    #[doc = "1: LSE oscillator ON"]
    B_0x1 = 1,
}
impl From<LSEON_A> for bool {
    #[inline(always)]
    fn from(variant: LSEON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEON` reader - LSE oscillator enabled Set and reset by software."]
pub type LSEON_R = crate::BitReader<LSEON_A>;
impl LSEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEON_A {
        match self.bits {
            false => LSEON_A::B_0x0,
            true => LSEON_A::B_0x1,
        }
    }
    #[doc = "LSE oscillator OFF (default after Backup domain reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSEON_A::B_0x0
    }
    #[doc = "LSE oscillator ON"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSEON_A::B_0x1
    }
}
#[doc = "Field `LSEON` writer - LSE oscillator enabled Set and reset by software."]
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG, LSEON_A>;
impl<'a, REG> LSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator OFF (default after Backup domain reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON_A::B_0x0)
    }
    #[doc = "LSE oscillator ON"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON_A::B_0x1)
    }
}
#[doc = "LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDY_A {
    #[doc = "0: LSE oscillator not ready (default after Backup domain reset)"]
    B_0x0 = 0,
    #[doc = "1: LSE oscillator ready"]
    B_0x1 = 1,
}
impl From<LSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDY` reader - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
pub type LSERDY_R = crate::BitReader<LSERDY_A>;
impl LSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSERDY_A {
        match self.bits {
            false => LSERDY_A::B_0x0,
            true => LSERDY_A::B_0x1,
        }
    }
    #[doc = "LSE oscillator not ready (default after Backup domain reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSERDY_A::B_0x0
    }
    #[doc = "LSE oscillator ready"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSERDY_A::B_0x1
    }
}
#[doc = "Field `LSERDY` writer - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
pub type LSERDY_W<'a, REG> = crate::BitWriter<'a, REG, LSERDY_A>;
impl<'a, REG> LSERDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator not ready (default after Backup domain reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDY_A::B_0x0)
    }
    #[doc = "LSE oscillator ready"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDY_A::B_0x1)
    }
}
#[doc = "LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP_A {
    #[doc = "0: LSE oscillator not bypassed (default after Backup domain reset)"]
    B_0x0 = 0,
    #[doc = "1: LSE oscillator bypassed"]
    B_0x1 = 1,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
pub type LSEBYP_R = crate::BitReader<LSEBYP_A>;
impl LSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::B_0x0,
            true => LSEBYP_A::B_0x1,
        }
    }
    #[doc = "LSE oscillator not bypassed (default after Backup domain reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSEBYP_A::B_0x0
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSEBYP_A::B_0x1
    }
}
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, LSEBYP_A>;
impl<'a, REG> LSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator not bypassed (default after Backup domain reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP_A::B_0x0)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP_A::B_0x1)
    }
}
#[doc = "LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV_A {
    #[doc = "0: lowest drive (default after Backup domain reset)"]
    B_0x0 = 0,
    #[doc = "1: medium-low drive"]
    B_0x1 = 1,
    #[doc = "2: medium-high drive"]
    B_0x2 = 2,
    #[doc = "3: highest drive"]
    B_0x3 = 3,
}
impl From<LSEDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSEDRV_A {
    type Ux = u8;
}
impl crate::IsEnum for LSEDRV_A {}
#[doc = "Field `LSEDRV` reader - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
pub type LSEDRV_R = crate::FieldReader<LSEDRV_A>;
impl LSEDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEDRV_A {
        match self.bits {
            0 => LSEDRV_A::B_0x0,
            1 => LSEDRV_A::B_0x1,
            2 => LSEDRV_A::B_0x2,
            3 => LSEDRV_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "lowest drive (default after Backup domain reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSEDRV_A::B_0x0
    }
    #[doc = "medium-low drive"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSEDRV_A::B_0x1
    }
    #[doc = "medium-high drive"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == LSEDRV_A::B_0x2
    }
    #[doc = "highest drive"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == LSEDRV_A::B_0x3
    }
}
#[doc = "Field `LSEDRV` writer - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LSEDRV_A, crate::Safe>;
impl<'a, REG> LSEDRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "lowest drive (default after Backup domain reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV_A::B_0x0)
    }
    #[doc = "medium-low drive"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV_A::B_0x1)
    }
    #[doc = "medium-high drive"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV_A::B_0x2)
    }
    #[doc = "highest drive"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV_A::B_0x3)
    }
}
#[doc = "LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON_A {
    #[doc = "0: CSS on 32 kHz oscillator OFF (default after Backup domain reset)"]
    B_0x0 = 0,
    #[doc = "1: CSS on 32 kHz oscillator ON"]
    B_0x1 = 1,
}
impl From<LSECSSON_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSON` reader - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
pub type LSECSSON_R = crate::BitReader<LSECSSON_A>;
impl LSECSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSON_A {
        match self.bits {
            false => LSECSSON_A::B_0x0,
            true => LSECSSON_A::B_0x1,
        }
    }
    #[doc = "CSS on 32 kHz oscillator OFF (default after Backup domain reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSECSSON_A::B_0x0
    }
    #[doc = "CSS on 32 kHz oscillator ON"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSECSSON_A::B_0x1
    }
}
#[doc = "Field `LSECSSON` writer - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSON_A>;
impl<'a, REG> LSECSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSS on 32 kHz oscillator OFF (default after Backup domain reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON_A::B_0x0)
    }
    #[doc = "CSS on 32 kHz oscillator ON"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON_A::B_0x1)
    }
}
#[doc = "LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSD_A {
    #[doc = "0: no failure detected on 32 kHz oscillator (default after Backup domain reset)"]
    B_0x0 = 0,
    #[doc = "1: failure detected on 32 kHz oscillator"]
    B_0x1 = 1,
}
impl From<LSECSSD_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSD` reader - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
pub type LSECSSD_R = crate::BitReader<LSECSSD_A>;
impl LSECSSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSD_A {
        match self.bits {
            false => LSECSSD_A::B_0x0,
            true => LSECSSD_A::B_0x1,
        }
    }
    #[doc = "no failure detected on 32 kHz oscillator (default after Backup domain reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSECSSD_A::B_0x0
    }
    #[doc = "failure detected on 32 kHz oscillator"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSECSSD_A::B_0x1
    }
}
#[doc = "Field `LSECSSD` writer - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
pub type LSECSSD_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSD_A>;
impl<'a, REG> LSECSSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no failure detected on 32 kHz oscillator (default after Backup domain reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSD_A::B_0x0)
    }
    #[doc = "failure detected on 32 kHz oscillator"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSD_A::B_0x1)
    }
}
#[doc = "low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEEXT_A {
    #[doc = "0: LSE in analog mode (default after Backup domain reset)"]
    B_0x0 = 0,
    #[doc = "1: LSE in digital mode (do not use if RTC is active)."]
    B_0x1 = 1,
}
impl From<LSEEXT_A> for bool {
    #[inline(always)]
    fn from(variant: LSEEXT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEEXT` reader - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
pub type LSEEXT_R = crate::BitReader<LSEEXT_A>;
impl LSEEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEEXT_A {
        match self.bits {
            false => LSEEXT_A::B_0x0,
            true => LSEEXT_A::B_0x1,
        }
    }
    #[doc = "LSE in analog mode (default after Backup domain reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSEEXT_A::B_0x0
    }
    #[doc = "LSE in digital mode (do not use if RTC is active)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSEEXT_A::B_0x1
    }
}
#[doc = "Field `LSEEXT` writer - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
pub type LSEEXT_W<'a, REG> = crate::BitWriter<'a, REG, LSEEXT_A>;
impl<'a, REG> LSEEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE in analog mode (default after Backup domain reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSEEXT_A::B_0x0)
    }
    #[doc = "LSE in digital mode (do not use if RTC is active)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSEEXT_A::B_0x1)
    }
}
#[doc = "RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL_A {
    #[doc = "0: no clock (default after Backup domain reset)"]
    B_0x0 = 0,
    #[doc = "1: LSE selected as RTC clock"]
    B_0x1 = 1,
    #[doc = "2: LSI selected as RTC clock"]
    B_0x2 = 2,
    #[doc = "3: HSE divided by RTCPRE value selected as RTC clock"]
    B_0x3 = 3,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for RTCSEL_A {}
#[doc = "Field `RTCSEL` reader - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
pub type RTCSEL_R = crate::FieldReader<RTCSEL_A>;
impl RTCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCSEL_A {
        match self.bits {
            0 => RTCSEL_A::B_0x0,
            1 => RTCSEL_A::B_0x1,
            2 => RTCSEL_A::B_0x2,
            3 => RTCSEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no clock (default after Backup domain reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RTCSEL_A::B_0x0
    }
    #[doc = "LSE selected as RTC clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RTCSEL_A::B_0x1
    }
    #[doc = "LSI selected as RTC clock"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == RTCSEL_A::B_0x2
    }
    #[doc = "HSE divided by RTCPRE value selected as RTC clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == RTCSEL_A::B_0x3
    }
}
#[doc = "Field `RTCSEL` writer - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RTCSEL_A, crate::Safe>;
impl<'a, REG> RTCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no clock (default after Backup domain reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL_A::B_0x0)
    }
    #[doc = "LSE selected as RTC clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL_A::B_0x1)
    }
    #[doc = "LSI selected as RTC clock"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL_A::B_0x2)
    }
    #[doc = "HSE divided by RTCPRE value selected as RTC clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL_A::B_0x3)
    }
}
#[doc = "RTC clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN_A {
    #[doc = "0: rtc_ck disabled (default after Backup domain reset)"]
    B_0x0 = 0,
    #[doc = "1: rtc_ck enabled"]
    B_0x1 = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable Set and reset by software."]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::B_0x0,
            true => RTCEN_A::B_0x1,
        }
    }
    #[doc = "rtc_ck disabled (default after Backup domain reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RTCEN_A::B_0x0
    }
    #[doc = "rtc_ck enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RTCEN_A::B_0x1
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable Set and reset by software."]
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCEN_A>;
impl<'a, REG> RTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "rtc_ck disabled (default after Backup domain reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN_A::B_0x0)
    }
    #[doc = "rtc_ck enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN_A::B_0x1)
    }
}
#[doc = "VSwitch domain software reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSWRST_A {
    #[doc = "0: reset not activated (default after Backup domain reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the entire VSW domain"]
    B_0x1 = 1,
}
impl From<VSWRST_A> for bool {
    #[inline(always)]
    fn from(variant: VSWRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSWRST` reader - VSwitch domain software reset Set and reset by software."]
pub type VSWRST_R = crate::BitReader<VSWRST_A>;
impl VSWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VSWRST_A {
        match self.bits {
            false => VSWRST_A::B_0x0,
            true => VSWRST_A::B_0x1,
        }
    }
    #[doc = "reset not activated (default after Backup domain reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VSWRST_A::B_0x0
    }
    #[doc = "resets the entire VSW domain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VSWRST_A::B_0x1
    }
}
#[doc = "Field `VSWRST` writer - VSwitch domain software reset Set and reset by software."]
pub type VSWRST_W<'a, REG> = crate::BitWriter<'a, REG, VSWRST_A>;
impl<'a, REG> VSWRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "reset not activated (default after Backup domain reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VSWRST_A::B_0x0)
    }
    #[doc = "resets the entire VSW domain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VSWRST_A::B_0x1)
    }
}
#[doc = "Low-speed clock output (LSCO) enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOEN_A {
    #[doc = "0: LSCO output disabled"]
    B_0x0 = 0,
    #[doc = "1: LSCO output enabled"]
    B_0x1 = 1,
}
impl From<LSCOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LSCOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSCOEN` reader - Low-speed clock output (LSCO) enable Set and cleared by software."]
pub type LSCOEN_R = crate::BitReader<LSCOEN_A>;
impl LSCOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSCOEN_A {
        match self.bits {
            false => LSCOEN_A::B_0x0,
            true => LSCOEN_A::B_0x1,
        }
    }
    #[doc = "LSCO output disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSCOEN_A::B_0x0
    }
    #[doc = "LSCO output enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSCOEN_A::B_0x1
    }
}
#[doc = "Field `LSCOEN` writer - Low-speed clock output (LSCO) enable Set and cleared by software."]
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG, LSCOEN_A>;
impl<'a, REG> LSCOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSCO output disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOEN_A::B_0x0)
    }
    #[doc = "LSCO output enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOEN_A::B_0x1)
    }
}
#[doc = "Low-speed clock output selection Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOSEL_A {
    #[doc = "0: LSI clock selected"]
    B_0x0 = 0,
    #[doc = "1: LSE clock selected"]
    B_0x1 = 1,
}
impl From<LSCOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: LSCOSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSCOSEL` reader - Low-speed clock output selection Set and cleared by software."]
pub type LSCOSEL_R = crate::BitReader<LSCOSEL_A>;
impl LSCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSCOSEL_A {
        match self.bits {
            false => LSCOSEL_A::B_0x0,
            true => LSCOSEL_A::B_0x1,
        }
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSCOSEL_A::B_0x0
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSCOSEL_A::B_0x1
    }
}
#[doc = "Field `LSCOSEL` writer - Low-speed clock output selection Set and cleared by software."]
pub type LSCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, LSCOSEL_A>;
impl<'a, REG> LSCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOSEL_A::B_0x0)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOSEL_A::B_0x1)
    }
}
#[doc = "LSI oscillator enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION_A {
    #[doc = "0: LSI oscillator off"]
    B_0x0 = 0,
    #[doc = "1: LSI oscillator on"]
    B_0x1 = 1,
}
impl From<LSION_A> for bool {
    #[inline(always)]
    fn from(variant: LSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSION` reader - LSI oscillator enable Set and cleared by software."]
pub type LSION_R = crate::BitReader<LSION_A>;
impl LSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSION_A {
        match self.bits {
            false => LSION_A::B_0x0,
            true => LSION_A::B_0x1,
        }
    }
    #[doc = "LSI oscillator off"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSION_A::B_0x0
    }
    #[doc = "LSI oscillator on"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSION_A::B_0x1
    }
}
#[doc = "Field `LSION` writer - LSI oscillator enable Set and cleared by software."]
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION_A>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI oscillator off"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSION_A::B_0x0)
    }
    #[doc = "LSI oscillator on"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSION_A::B_0x1)
    }
}
#[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDY_A {
    #[doc = "0: LSI oscillator not ready"]
    B_0x0 = 0,
    #[doc = "1: LSI oscillator ready"]
    B_0x1 = 1,
}
impl From<LSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDY` reader - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
pub type LSIRDY_R = crate::BitReader<LSIRDY_A>;
impl LSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDY_A {
        match self.bits {
            false => LSIRDY_A::B_0x0,
            true => LSIRDY_A::B_0x1,
        }
    }
    #[doc = "LSI oscillator not ready"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSIRDY_A::B_0x0
    }
    #[doc = "LSI oscillator ready"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSIRDY_A::B_0x1
    }
}
#[doc = "Field `LSIRDY` writer - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
pub type LSIRDY_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDY_A>;
impl<'a, REG> LSIRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI oscillator not ready"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDY_A::B_0x0)
    }
    #[doc = "LSI oscillator ready"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDY_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - LSE oscillator enabled Set and reset by software."]
    #[inline(always)]
    pub fn LSEON(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
    #[inline(always)]
    pub fn LSERDY(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
    #[inline(always)]
    pub fn LSEBYP(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
    #[inline(always)]
    pub fn LSEDRV(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
    #[inline(always)]
    pub fn LSECSSON(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
    #[inline(always)]
    pub fn LSECSSD(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
    #[inline(always)]
    pub fn LSEEXT(&self) -> LSEEXT_R {
        LSEEXT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
    #[inline(always)]
    pub fn RTCSEL(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn RTCEN(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - VSwitch domain software reset Set and reset by software."]
    #[inline(always)]
    pub fn VSWRST(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software."]
    #[inline(always)]
    pub fn LSCOEN(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low-speed clock output selection Set and cleared by software."]
    #[inline(always)]
    pub fn LSCOSEL(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn LSION(&self) -> LSION_R {
        LSION_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
    #[inline(always)]
    pub fn LSIRDY(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enabled Set and reset by software."]
    #[inline(always)]
    pub fn LSEON(&mut self) -> LSEON_W<'_, BDCR_SPEC> {
        LSEON_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
    #[inline(always)]
    pub fn LSERDY(&mut self) -> LSERDY_W<'_, BDCR_SPEC> {
        LSERDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
    #[inline(always)]
    pub fn LSEBYP(&mut self) -> LSEBYP_W<'_, BDCR_SPEC> {
        LSEBYP_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
    #[inline(always)]
    pub fn LSEDRV(&mut self) -> LSEDRV_W<'_, BDCR_SPEC> {
        LSEDRV_W::new(self, 3)
    }
    #[doc = "Bit 5 - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
    #[inline(always)]
    pub fn LSECSSON(&mut self) -> LSECSSON_W<'_, BDCR_SPEC> {
        LSECSSON_W::new(self, 5)
    }
    #[doc = "Bit 6 - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
    #[inline(always)]
    pub fn LSECSSD(&mut self) -> LSECSSD_W<'_, BDCR_SPEC> {
        LSECSSD_W::new(self, 6)
    }
    #[doc = "Bit 7 - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
    #[inline(always)]
    pub fn LSEEXT(&mut self) -> LSEEXT_W<'_, BDCR_SPEC> {
        LSEEXT_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
    #[inline(always)]
    pub fn RTCSEL(&mut self) -> RTCSEL_W<'_, BDCR_SPEC> {
        RTCSEL_W::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn RTCEN(&mut self) -> RTCEN_W<'_, BDCR_SPEC> {
        RTCEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - VSwitch domain software reset Set and reset by software."]
    #[inline(always)]
    pub fn VSWRST(&mut self) -> VSWRST_W<'_, BDCR_SPEC> {
        VSWRST_W::new(self, 16)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software."]
    #[inline(always)]
    pub fn LSCOEN(&mut self) -> LSCOEN_W<'_, BDCR_SPEC> {
        LSCOEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Low-speed clock output selection Set and cleared by software."]
    #[inline(always)]
    pub fn LSCOSEL(&mut self) -> LSCOSEL_W<'_, BDCR_SPEC> {
        LSCOSEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn LSION(&mut self) -> LSION_W<'_, BDCR_SPEC> {
        LSION_W::new(self, 26)
    }
    #[doc = "Bit 27 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
    #[inline(always)]
    pub fn LSIRDY(&mut self) -> LSIRDY_W<'_, BDCR_SPEC> {
        LSIRDY_W::new(self, 27)
    }
}
#[doc = "RCC Backup domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcr::R`](R) reader structure"]
impl crate::Readable for BDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bdcr::W`](W) writer structure"]
impl crate::Writable for BDCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCR_SPEC {}
