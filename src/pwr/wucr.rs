#[doc = "Register `WUCR` reader"]
pub type R = crate::R<WUCR_SPEC>;
#[doc = "Register `WUCR` writer"]
pub type W = crate::W<WUCR_SPEC>;
#[doc = "enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN1_A {
    #[doc = "0: an event on WUPx pin does not wakeup the system from Standby mode."]
    B_0x0 = 0,
    #[doc = "1: a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    B_0x1 = 1,
}
impl From<WUPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN1` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN1_R = crate::BitReader<WUPEN1_A>;
impl WUPEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN1_A {
        match self.bits {
            false => WUPEN1_A::B_0x0,
            true => WUPEN1_A::B_0x1,
        }
    }
    #[doc = "an event on WUPx pin does not wakeup the system from Standby mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPEN1_A::B_0x0
    }
    #[doc = "a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPEN1_A::B_0x1
    }
}
#[doc = "Field `WUPEN1` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN1_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN1_A>;
impl<'a, REG> WUPEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "an event on WUPx pin does not wakeup the system from Standby mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN1_A::B_0x0)
    }
    #[doc = "a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN1_A::B_0x1)
    }
}
#[doc = "enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN2_A {
    #[doc = "0: an event on WUPx pin does not wakeup the system from Standby mode."]
    B_0x0 = 0,
    #[doc = "1: a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    B_0x1 = 1,
}
impl From<WUPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN2` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN2_R = crate::BitReader<WUPEN2_A>;
impl WUPEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN2_A {
        match self.bits {
            false => WUPEN2_A::B_0x0,
            true => WUPEN2_A::B_0x1,
        }
    }
    #[doc = "an event on WUPx pin does not wakeup the system from Standby mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPEN2_A::B_0x0
    }
    #[doc = "a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPEN2_A::B_0x1
    }
}
#[doc = "Field `WUPEN2` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN2_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN2_A>;
impl<'a, REG> WUPEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "an event on WUPx pin does not wakeup the system from Standby mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN2_A::B_0x0)
    }
    #[doc = "a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN2_A::B_0x1)
    }
}
#[doc = "enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN3_A {
    #[doc = "0: an event on WUPx pin does not wakeup the system from Standby mode."]
    B_0x0 = 0,
    #[doc = "1: a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    B_0x1 = 1,
}
impl From<WUPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN3` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN3_R = crate::BitReader<WUPEN3_A>;
impl WUPEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN3_A {
        match self.bits {
            false => WUPEN3_A::B_0x0,
            true => WUPEN3_A::B_0x1,
        }
    }
    #[doc = "an event on WUPx pin does not wakeup the system from Standby mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPEN3_A::B_0x0
    }
    #[doc = "a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPEN3_A::B_0x1
    }
}
#[doc = "Field `WUPEN3` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN3_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN3_A>;
impl<'a, REG> WUPEN3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "an event on WUPx pin does not wakeup the system from Standby mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN3_A::B_0x0)
    }
    #[doc = "a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN3_A::B_0x1)
    }
}
#[doc = "enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN4_A {
    #[doc = "0: an event on WUPx pin does not wakeup the system from Standby mode."]
    B_0x0 = 0,
    #[doc = "1: a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    B_0x1 = 1,
}
impl From<WUPEN4_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN4` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN4_R = crate::BitReader<WUPEN4_A>;
impl WUPEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN4_A {
        match self.bits {
            false => WUPEN4_A::B_0x0,
            true => WUPEN4_A::B_0x1,
        }
    }
    #[doc = "an event on WUPx pin does not wakeup the system from Standby mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPEN4_A::B_0x0
    }
    #[doc = "a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPEN4_A::B_0x1
    }
}
#[doc = "Field `WUPEN4` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN4_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN4_A>;
impl<'a, REG> WUPEN4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "an event on WUPx pin does not wakeup the system from Standby mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN4_A::B_0x0)
    }
    #[doc = "a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN4_A::B_0x1)
    }
}
#[doc = "enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN5_A {
    #[doc = "0: an event on WUPx pin does not wakeup the system from Standby mode."]
    B_0x0 = 0,
    #[doc = "1: a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    B_0x1 = 1,
}
impl From<WUPEN5_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN5` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN5_R = crate::BitReader<WUPEN5_A>;
impl WUPEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN5_A {
        match self.bits {
            false => WUPEN5_A::B_0x0,
            true => WUPEN5_A::B_0x1,
        }
    }
    #[doc = "an event on WUPx pin does not wakeup the system from Standby mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPEN5_A::B_0x0
    }
    #[doc = "a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPEN5_A::B_0x1
    }
}
#[doc = "Field `WUPEN5` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN5_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN5_A>;
impl<'a, REG> WUPEN5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "an event on WUPx pin does not wakeup the system from Standby mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN5_A::B_0x0)
    }
    #[doc = "a rising or falling edge on WUPx pin wakes up the system from Standby mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN5_A::B_0x1)
    }
}
#[doc = "wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPP1_A {
    #[doc = "0: detection on high level (rising edge)"]
    B_0x0 = 0,
    #[doc = "1: detection on low level (falling edge)"]
    B_0x1 = 1,
}
impl From<WUPP1_A> for bool {
    #[inline(always)]
    fn from(variant: WUPP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPP1` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP1_R = crate::BitReader<WUPP1_A>;
impl WUPP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPP1_A {
        match self.bits {
            false => WUPP1_A::B_0x0,
            true => WUPP1_A::B_0x1,
        }
    }
    #[doc = "detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPP1_A::B_0x0
    }
    #[doc = "detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPP1_A::B_0x1
    }
}
#[doc = "Field `WUPP1` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP1_W<'a, REG> = crate::BitWriter<'a, REG, WUPP1_A>;
impl<'a, REG> WUPP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "detection on high level (rising edge)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP1_A::B_0x0)
    }
    #[doc = "detection on low level (falling edge)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP1_A::B_0x1)
    }
}
#[doc = "wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPP2_A {
    #[doc = "0: detection on high level (rising edge)"]
    B_0x0 = 0,
    #[doc = "1: detection on low level (falling edge)"]
    B_0x1 = 1,
}
impl From<WUPP2_A> for bool {
    #[inline(always)]
    fn from(variant: WUPP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPP2` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP2_R = crate::BitReader<WUPP2_A>;
impl WUPP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPP2_A {
        match self.bits {
            false => WUPP2_A::B_0x0,
            true => WUPP2_A::B_0x1,
        }
    }
    #[doc = "detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPP2_A::B_0x0
    }
    #[doc = "detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPP2_A::B_0x1
    }
}
#[doc = "Field `WUPP2` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP2_W<'a, REG> = crate::BitWriter<'a, REG, WUPP2_A>;
impl<'a, REG> WUPP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "detection on high level (rising edge)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP2_A::B_0x0)
    }
    #[doc = "detection on low level (falling edge)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP2_A::B_0x1)
    }
}
#[doc = "wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPP3_A {
    #[doc = "0: detection on high level (rising edge)"]
    B_0x0 = 0,
    #[doc = "1: detection on low level (falling edge)"]
    B_0x1 = 1,
}
impl From<WUPP3_A> for bool {
    #[inline(always)]
    fn from(variant: WUPP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPP3` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP3_R = crate::BitReader<WUPP3_A>;
impl WUPP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPP3_A {
        match self.bits {
            false => WUPP3_A::B_0x0,
            true => WUPP3_A::B_0x1,
        }
    }
    #[doc = "detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPP3_A::B_0x0
    }
    #[doc = "detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPP3_A::B_0x1
    }
}
#[doc = "Field `WUPP3` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP3_W<'a, REG> = crate::BitWriter<'a, REG, WUPP3_A>;
impl<'a, REG> WUPP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "detection on high level (rising edge)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP3_A::B_0x0)
    }
    #[doc = "detection on low level (falling edge)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP3_A::B_0x1)
    }
}
#[doc = "wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPP4_A {
    #[doc = "0: detection on high level (rising edge)"]
    B_0x0 = 0,
    #[doc = "1: detection on low level (falling edge)"]
    B_0x1 = 1,
}
impl From<WUPP4_A> for bool {
    #[inline(always)]
    fn from(variant: WUPP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPP4` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP4_R = crate::BitReader<WUPP4_A>;
impl WUPP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPP4_A {
        match self.bits {
            false => WUPP4_A::B_0x0,
            true => WUPP4_A::B_0x1,
        }
    }
    #[doc = "detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPP4_A::B_0x0
    }
    #[doc = "detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPP4_A::B_0x1
    }
}
#[doc = "Field `WUPP4` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP4_W<'a, REG> = crate::BitWriter<'a, REG, WUPP4_A>;
impl<'a, REG> WUPP4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "detection on high level (rising edge)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP4_A::B_0x0)
    }
    #[doc = "detection on low level (falling edge)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP4_A::B_0x1)
    }
}
#[doc = "wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPP5_A {
    #[doc = "0: detection on high level (rising edge)"]
    B_0x0 = 0,
    #[doc = "1: detection on low level (falling edge)"]
    B_0x1 = 1,
}
impl From<WUPP5_A> for bool {
    #[inline(always)]
    fn from(variant: WUPP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPP5` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP5_R = crate::BitReader<WUPP5_A>;
impl WUPP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPP5_A {
        match self.bits {
            false => WUPP5_A::B_0x0,
            true => WUPP5_A::B_0x1,
        }
    }
    #[doc = "detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPP5_A::B_0x0
    }
    #[doc = "detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPP5_A::B_0x1
    }
}
#[doc = "Field `WUPP5` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP5_W<'a, REG> = crate::BitWriter<'a, REG, WUPP5_A>;
impl<'a, REG> WUPP5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "detection on high level (rising edge)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP5_A::B_0x0)
    }
    #[doc = "detection on low level (falling edge)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP5_A::B_0x1)
    }
}
#[doc = "wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPPUPD1_A {
    #[doc = "0: no pull-up"]
    B_0x0 = 0,
    #[doc = "1: pull-up"]
    B_0x1 = 1,
    #[doc = "2: pull-down"]
    B_0x2 = 2,
}
impl From<WUPPUPD1_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPPUPD1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUPPUPD1_A {
    type Ux = u8;
}
impl crate::IsEnum for WUPPUPD1_A {}
#[doc = "Field `WUPPUPD1` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD1_R = crate::FieldReader<WUPPUPD1_A>;
impl WUPPUPD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUPPUPD1_A> {
        match self.bits {
            0 => Some(WUPPUPD1_A::B_0x0),
            1 => Some(WUPPUPD1_A::B_0x1),
            2 => Some(WUPPUPD1_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "no pull-up"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPPUPD1_A::B_0x0
    }
    #[doc = "pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPPUPD1_A::B_0x1
    }
    #[doc = "pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == WUPPUPD1_A::B_0x2
    }
}
#[doc = "Field `WUPPUPD1` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WUPPUPD1_A>;
impl<'a, REG> WUPPUPD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no pull-up"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD1_A::B_0x0)
    }
    #[doc = "pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD1_A::B_0x1)
    }
    #[doc = "pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD1_A::B_0x2)
    }
}
#[doc = "wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPPUPD2_A {
    #[doc = "0: no pull-up"]
    B_0x0 = 0,
    #[doc = "1: pull-up"]
    B_0x1 = 1,
    #[doc = "2: pull-down"]
    B_0x2 = 2,
}
impl From<WUPPUPD2_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPPUPD2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUPPUPD2_A {
    type Ux = u8;
}
impl crate::IsEnum for WUPPUPD2_A {}
#[doc = "Field `WUPPUPD2` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD2_R = crate::FieldReader<WUPPUPD2_A>;
impl WUPPUPD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUPPUPD2_A> {
        match self.bits {
            0 => Some(WUPPUPD2_A::B_0x0),
            1 => Some(WUPPUPD2_A::B_0x1),
            2 => Some(WUPPUPD2_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "no pull-up"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPPUPD2_A::B_0x0
    }
    #[doc = "pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPPUPD2_A::B_0x1
    }
    #[doc = "pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == WUPPUPD2_A::B_0x2
    }
}
#[doc = "Field `WUPPUPD2` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WUPPUPD2_A>;
impl<'a, REG> WUPPUPD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no pull-up"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD2_A::B_0x0)
    }
    #[doc = "pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD2_A::B_0x1)
    }
    #[doc = "pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD2_A::B_0x2)
    }
}
#[doc = "wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPPUPD3_A {
    #[doc = "0: no pull-up"]
    B_0x0 = 0,
    #[doc = "1: pull-up"]
    B_0x1 = 1,
    #[doc = "2: pull-down"]
    B_0x2 = 2,
}
impl From<WUPPUPD3_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPPUPD3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUPPUPD3_A {
    type Ux = u8;
}
impl crate::IsEnum for WUPPUPD3_A {}
#[doc = "Field `WUPPUPD3` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD3_R = crate::FieldReader<WUPPUPD3_A>;
impl WUPPUPD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUPPUPD3_A> {
        match self.bits {
            0 => Some(WUPPUPD3_A::B_0x0),
            1 => Some(WUPPUPD3_A::B_0x1),
            2 => Some(WUPPUPD3_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "no pull-up"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPPUPD3_A::B_0x0
    }
    #[doc = "pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPPUPD3_A::B_0x1
    }
    #[doc = "pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == WUPPUPD3_A::B_0x2
    }
}
#[doc = "Field `WUPPUPD3` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WUPPUPD3_A>;
impl<'a, REG> WUPPUPD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no pull-up"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD3_A::B_0x0)
    }
    #[doc = "pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD3_A::B_0x1)
    }
    #[doc = "pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD3_A::B_0x2)
    }
}
#[doc = "wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPPUPD4_A {
    #[doc = "0: no pull-up"]
    B_0x0 = 0,
    #[doc = "1: pull-up"]
    B_0x1 = 1,
    #[doc = "2: pull-down"]
    B_0x2 = 2,
}
impl From<WUPPUPD4_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPPUPD4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUPPUPD4_A {
    type Ux = u8;
}
impl crate::IsEnum for WUPPUPD4_A {}
#[doc = "Field `WUPPUPD4` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD4_R = crate::FieldReader<WUPPUPD4_A>;
impl WUPPUPD4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUPPUPD4_A> {
        match self.bits {
            0 => Some(WUPPUPD4_A::B_0x0),
            1 => Some(WUPPUPD4_A::B_0x1),
            2 => Some(WUPPUPD4_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "no pull-up"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPPUPD4_A::B_0x0
    }
    #[doc = "pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPPUPD4_A::B_0x1
    }
    #[doc = "pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == WUPPUPD4_A::B_0x2
    }
}
#[doc = "Field `WUPPUPD4` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WUPPUPD4_A>;
impl<'a, REG> WUPPUPD4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no pull-up"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD4_A::B_0x0)
    }
    #[doc = "pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD4_A::B_0x1)
    }
    #[doc = "pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD4_A::B_0x2)
    }
}
#[doc = "wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPPUPD5_A {
    #[doc = "0: no pull-up"]
    B_0x0 = 0,
    #[doc = "1: pull-up"]
    B_0x1 = 1,
    #[doc = "2: pull-down"]
    B_0x2 = 2,
}
impl From<WUPPUPD5_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPPUPD5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUPPUPD5_A {
    type Ux = u8;
}
impl crate::IsEnum for WUPPUPD5_A {}
#[doc = "Field `WUPPUPD5` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD5_R = crate::FieldReader<WUPPUPD5_A>;
impl WUPPUPD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUPPUPD5_A> {
        match self.bits {
            0 => Some(WUPPUPD5_A::B_0x0),
            1 => Some(WUPPUPD5_A::B_0x1),
            2 => Some(WUPPUPD5_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "no pull-up"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPPUPD5_A::B_0x0
    }
    #[doc = "pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPPUPD5_A::B_0x1
    }
    #[doc = "pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == WUPPUPD5_A::B_0x2
    }
}
#[doc = "Field `WUPPUPD5` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WUPPUPD5_A>;
impl<'a, REG> WUPPUPD5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no pull-up"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD5_A::B_0x0)
    }
    #[doc = "pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD5_A::B_0x1)
    }
    #[doc = "pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD5_A::B_0x2)
    }
}
impl R {
    #[doc = "Bit 0 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn WUPEN1(&self) -> WUPEN1_R {
        WUPEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn WUPEN2(&self) -> WUPEN2_R {
        WUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn WUPEN3(&self) -> WUPEN3_R {
        WUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn WUPEN4(&self) -> WUPEN4_R {
        WUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn WUPEN5(&self) -> WUPEN5_R {
        WUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn WUPP1(&self) -> WUPP1_R {
        WUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn WUPP2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn WUPP3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn WUPP4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn WUPP5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn WUPPUPD1(&self) -> WUPPUPD1_R {
        WUPPUPD1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn WUPPUPD2(&self) -> WUPPUPD2_R {
        WUPPUPD2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn WUPPUPD3(&self) -> WUPPUPD3_R {
        WUPPUPD3_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn WUPPUPD4(&self) -> WUPPUPD4_R {
        WUPPUPD4_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn WUPPUPD5(&self) -> WUPPUPD5_R {
        WUPPUPD5_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn WUPEN1(&mut self) -> WUPEN1_W<'_, WUCR_SPEC> {
        WUPEN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn WUPEN2(&mut self) -> WUPEN2_W<'_, WUCR_SPEC> {
        WUPEN2_W::new(self, 1)
    }
    #[doc = "Bit 2 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn WUPEN3(&mut self) -> WUPEN3_W<'_, WUCR_SPEC> {
        WUPEN3_W::new(self, 2)
    }
    #[doc = "Bit 3 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn WUPEN4(&mut self) -> WUPEN4_W<'_, WUCR_SPEC> {
        WUPEN4_W::new(self, 3)
    }
    #[doc = "Bit 4 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn WUPEN5(&mut self) -> WUPEN5_W<'_, WUCR_SPEC> {
        WUPEN5_W::new(self, 4)
    }
    #[doc = "Bit 8 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn WUPP1(&mut self) -> WUPP1_W<'_, WUCR_SPEC> {
        WUPP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn WUPP2(&mut self) -> WUPP2_W<'_, WUCR_SPEC> {
        WUPP2_W::new(self, 9)
    }
    #[doc = "Bit 10 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn WUPP3(&mut self) -> WUPP3_W<'_, WUCR_SPEC> {
        WUPP3_W::new(self, 10)
    }
    #[doc = "Bit 11 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn WUPP4(&mut self) -> WUPP4_W<'_, WUCR_SPEC> {
        WUPP4_W::new(self, 11)
    }
    #[doc = "Bit 12 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn WUPP5(&mut self) -> WUPP5_W<'_, WUCR_SPEC> {
        WUPP5_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn WUPPUPD1(&mut self) -> WUPPUPD1_W<'_, WUCR_SPEC> {
        WUPPUPD1_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn WUPPUPD2(&mut self) -> WUPPUPD2_W<'_, WUCR_SPEC> {
        WUPPUPD2_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn WUPPUPD3(&mut self) -> WUPPUPD3_W<'_, WUCR_SPEC> {
        WUPPUPD3_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn WUPPUPD4(&mut self) -> WUPPUPD4_W<'_, WUCR_SPEC> {
        WUPPUPD4_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn WUPPUPD5(&mut self) -> WUPPUPD5_W<'_, WUCR_SPEC> {
        WUPPUPD5_W::new(self, 24)
    }
}
#[doc = "PWR wakeup configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`wucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUCR_SPEC;
impl crate::RegisterSpec for WUCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wucr::R`](R) reader structure"]
impl crate::Readable for WUCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wucr::W`](W) writer structure"]
impl crate::Writable for WUCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WUCR to value 0"]
impl crate::Resettable for WUCR_SPEC {}
