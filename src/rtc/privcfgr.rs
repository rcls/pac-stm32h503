#[doc = "Register `PRIVCFGR` reader"]
pub type R = crate::R<PRIVCFGR_SPEC>;
#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGR_SPEC>;
#[doc = "Alarm A and SSR underflow privilege protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAPRIV_A {
    #[doc = "0: RTC Alarm A and SSR underflow configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    B_0x0 = 0,
    #[doc = "1: RTC Alarm A and SSR underflow configuration and interrupt clear can be written only when the APB access is privileged."]
    B_0x1 = 1,
}
impl From<ALRAPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAPRIV` reader - Alarm A and SSR underflow privilege protection"]
pub type ALRAPRIV_R = crate::BitReader<ALRAPRIV_A>;
impl ALRAPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRAPRIV_A {
        match self.bits {
            false => ALRAPRIV_A::B_0x0,
            true => ALRAPRIV_A::B_0x1,
        }
    }
    #[doc = "RTC Alarm A and SSR underflow configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ALRAPRIV_A::B_0x0
    }
    #[doc = "RTC Alarm A and SSR underflow configuration and interrupt clear can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ALRAPRIV_A::B_0x1
    }
}
#[doc = "Field `ALRAPRIV` writer - Alarm A and SSR underflow privilege protection"]
pub type ALRAPRIV_W<'a, REG> = crate::BitWriter<'a, REG, ALRAPRIV_A>;
impl<'a, REG> ALRAPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC Alarm A and SSR underflow configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAPRIV_A::B_0x0)
    }
    #[doc = "RTC Alarm A and SSR underflow configuration and interrupt clear can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAPRIV_A::B_0x1)
    }
}
#[doc = "Alarm B privilege protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBPRIV_A {
    #[doc = "0: RTC Alarm B configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    B_0x0 = 0,
    #[doc = "1: RTC Alarm B configuration and interrupt clear can be written only when the APB access is privileged."]
    B_0x1 = 1,
}
impl From<ALRBPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBPRIV` reader - Alarm B privilege protection"]
pub type ALRBPRIV_R = crate::BitReader<ALRBPRIV_A>;
impl ALRBPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRBPRIV_A {
        match self.bits {
            false => ALRBPRIV_A::B_0x0,
            true => ALRBPRIV_A::B_0x1,
        }
    }
    #[doc = "RTC Alarm B configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ALRBPRIV_A::B_0x0
    }
    #[doc = "RTC Alarm B configuration and interrupt clear can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ALRBPRIV_A::B_0x1
    }
}
#[doc = "Field `ALRBPRIV` writer - Alarm B privilege protection"]
pub type ALRBPRIV_W<'a, REG> = crate::BitWriter<'a, REG, ALRBPRIV_A>;
impl<'a, REG> ALRBPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC Alarm B configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBPRIV_A::B_0x0)
    }
    #[doc = "RTC Alarm B configuration and interrupt clear can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBPRIV_A::B_0x1)
    }
}
#[doc = "Wakeup timer privilege protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTPRIV_A {
    #[doc = "0: RTC Wakeup timer configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    B_0x0 = 0,
    #[doc = "1: RTC Wakeup timer configuration and interrupt clear can be written only when the APB access is privileged."]
    B_0x1 = 1,
}
impl From<WUTPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: WUTPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTPRIV` reader - Wakeup timer privilege protection"]
pub type WUTPRIV_R = crate::BitReader<WUTPRIV_A>;
impl WUTPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUTPRIV_A {
        match self.bits {
            false => WUTPRIV_A::B_0x0,
            true => WUTPRIV_A::B_0x1,
        }
    }
    #[doc = "RTC Wakeup timer configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUTPRIV_A::B_0x0
    }
    #[doc = "RTC Wakeup timer configuration and interrupt clear can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUTPRIV_A::B_0x1
    }
}
#[doc = "Field `WUTPRIV` writer - Wakeup timer privilege protection"]
pub type WUTPRIV_W<'a, REG> = crate::BitWriter<'a, REG, WUTPRIV_A>;
impl<'a, REG> WUTPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC Wakeup timer configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUTPRIV_A::B_0x0)
    }
    #[doc = "RTC Wakeup timer configuration and interrupt clear can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUTPRIV_A::B_0x1)
    }
}
#[doc = "Timestamp privilege protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSPRIV_A {
    #[doc = "0: RTC Timestamp configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    B_0x0 = 0,
    #[doc = "1: RTC Timestamp configuration and interrupt clear can be written only when the APB access is privileged."]
    B_0x1 = 1,
}
impl From<TSPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: TSPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSPRIV` reader - Timestamp privilege protection"]
pub type TSPRIV_R = crate::BitReader<TSPRIV_A>;
impl TSPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSPRIV_A {
        match self.bits {
            false => TSPRIV_A::B_0x0,
            true => TSPRIV_A::B_0x1,
        }
    }
    #[doc = "RTC Timestamp configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TSPRIV_A::B_0x0
    }
    #[doc = "RTC Timestamp configuration and interrupt clear can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TSPRIV_A::B_0x1
    }
}
#[doc = "Field `TSPRIV` writer - Timestamp privilege protection"]
pub type TSPRIV_W<'a, REG> = crate::BitWriter<'a, REG, TSPRIV_A>;
impl<'a, REG> TSPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC Timestamp configuration and interrupt clear can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSPRIV_A::B_0x0)
    }
    #[doc = "RTC Timestamp configuration and interrupt clear can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSPRIV_A::B_0x1)
    }
}
#[doc = "Shift register, Delight saving, calibration and reference clock privilege protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALPRIV_A {
    #[doc = "0: Shift register, Delight saving, calibration and reference clock can be written when the APB access is privileged or non-privileged."]
    B_0x0 = 0,
    #[doc = "1: Shift register, Delight saving, calibration and reference clock can be written only when the APB access is privileged."]
    B_0x1 = 1,
}
impl From<CALPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: CALPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALPRIV` reader - Shift register, Delight saving, calibration and reference clock privilege protection"]
pub type CALPRIV_R = crate::BitReader<CALPRIV_A>;
impl CALPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALPRIV_A {
        match self.bits {
            false => CALPRIV_A::B_0x0,
            true => CALPRIV_A::B_0x1,
        }
    }
    #[doc = "Shift register, Delight saving, calibration and reference clock can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CALPRIV_A::B_0x0
    }
    #[doc = "Shift register, Delight saving, calibration and reference clock can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CALPRIV_A::B_0x1
    }
}
#[doc = "Field `CALPRIV` writer - Shift register, Delight saving, calibration and reference clock privilege protection"]
pub type CALPRIV_W<'a, REG> = crate::BitWriter<'a, REG, CALPRIV_A>;
impl<'a, REG> CALPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shift register, Delight saving, calibration and reference clock can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CALPRIV_A::B_0x0)
    }
    #[doc = "Shift register, Delight saving, calibration and reference clock can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CALPRIV_A::B_0x1)
    }
}
#[doc = "Initialization privilege protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITPRIV_A {
    #[doc = "0: RTC Initialization mode, calendar and prescalers registers can be written when the APB access is privileged or non-privileged."]
    B_0x0 = 0,
    #[doc = "1: RTC Initialization mode, calendar and prescalers registers can be written only when the APB access is privileged."]
    B_0x1 = 1,
}
impl From<INITPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: INITPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITPRIV` reader - Initialization privilege protection"]
pub type INITPRIV_R = crate::BitReader<INITPRIV_A>;
impl INITPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INITPRIV_A {
        match self.bits {
            false => INITPRIV_A::B_0x0,
            true => INITPRIV_A::B_0x1,
        }
    }
    #[doc = "RTC Initialization mode, calendar and prescalers registers can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == INITPRIV_A::B_0x0
    }
    #[doc = "RTC Initialization mode, calendar and prescalers registers can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == INITPRIV_A::B_0x1
    }
}
#[doc = "Field `INITPRIV` writer - Initialization privilege protection"]
pub type INITPRIV_W<'a, REG> = crate::BitWriter<'a, REG, INITPRIV_A>;
impl<'a, REG> INITPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC Initialization mode, calendar and prescalers registers can be written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(INITPRIV_A::B_0x0)
    }
    #[doc = "RTC Initialization mode, calendar and prescalers registers can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(INITPRIV_A::B_0x1)
    }
}
#[doc = "RTC privilege protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV_A {
    #[doc = "0: All RTC registers can be written when the APB access is privileged or non-privileged, except the registers protected by other privilege protection bits."]
    B_0x0 = 0,
    #[doc = "1: All RTC registers can be written only when the APB access is privileged."]
    B_0x1 = 1,
}
impl From<PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV` reader - RTC privilege protection"]
pub type PRIV_R = crate::BitReader<PRIV_A>;
impl PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV_A {
        match self.bits {
            false => PRIV_A::B_0x0,
            true => PRIV_A::B_0x1,
        }
    }
    #[doc = "All RTC registers can be written when the APB access is privileged or non-privileged, except the registers protected by other privilege protection bits."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV_A::B_0x0
    }
    #[doc = "All RTC registers can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV_A::B_0x1
    }
}
#[doc = "Field `PRIV` writer - RTC privilege protection"]
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG, PRIV_A>;
impl<'a, REG> PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All RTC registers can be written when the APB access is privileged or non-privileged, except the registers protected by other privilege protection bits."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV_A::B_0x0)
    }
    #[doc = "All RTC registers can be written only when the APB access is privileged."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm A and SSR underflow privilege protection"]
    #[inline(always)]
    pub fn ALRAPRIV(&self) -> ALRAPRIV_R {
        ALRAPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B privilege protection"]
    #[inline(always)]
    pub fn ALRBPRIV(&self) -> ALRBPRIV_R {
        ALRBPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer privilege protection"]
    #[inline(always)]
    pub fn WUTPRIV(&self) -> WUTPRIV_R {
        WUTPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp privilege protection"]
    #[inline(always)]
    pub fn TSPRIV(&self) -> TSPRIV_R {
        TSPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - Shift register, Delight saving, calibration and reference clock privilege protection"]
    #[inline(always)]
    pub fn CALPRIV(&self) -> CALPRIV_R {
        CALPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Initialization privilege protection"]
    #[inline(always)]
    pub fn INITPRIV(&self) -> INITPRIV_R {
        INITPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC privilege protection"]
    #[inline(always)]
    pub fn PRIV(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm A and SSR underflow privilege protection"]
    #[inline(always)]
    pub fn ALRAPRIV(&mut self) -> ALRAPRIV_W<'_, PRIVCFGR_SPEC> {
        ALRAPRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm B privilege protection"]
    #[inline(always)]
    pub fn ALRBPRIV(&mut self) -> ALRBPRIV_W<'_, PRIVCFGR_SPEC> {
        ALRBPRIV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup timer privilege protection"]
    #[inline(always)]
    pub fn WUTPRIV(&mut self) -> WUTPRIV_W<'_, PRIVCFGR_SPEC> {
        WUTPRIV_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timestamp privilege protection"]
    #[inline(always)]
    pub fn TSPRIV(&mut self) -> TSPRIV_W<'_, PRIVCFGR_SPEC> {
        TSPRIV_W::new(self, 3)
    }
    #[doc = "Bit 13 - Shift register, Delight saving, calibration and reference clock privilege protection"]
    #[inline(always)]
    pub fn CALPRIV(&mut self) -> CALPRIV_W<'_, PRIVCFGR_SPEC> {
        CALPRIV_W::new(self, 13)
    }
    #[doc = "Bit 14 - Initialization privilege protection"]
    #[inline(always)]
    pub fn INITPRIV(&mut self) -> INITPRIV_W<'_, PRIVCFGR_SPEC> {
        INITPRIV_W::new(self, 14)
    }
    #[doc = "Bit 15 - RTC privilege protection"]
    #[inline(always)]
    pub fn PRIV(&mut self) -> PRIV_W<'_, PRIVCFGR_SPEC> {
        PRIV_W::new(self, 15)
    }
}
#[doc = "RTC privilege mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR_SPEC;
impl crate::RegisterSpec for PRIVCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PRIVCFGR to value 0"]
impl crate::Resettable for PRIVCFGR_SPEC {}
