#[doc = "Register `TZSC_MPCWM4BCFGR` reader"]
pub type R = crate::R<TZSC_MPCWM4BCFGR_SPEC>;
#[doc = "Register `TZSC_MPCWM4BCFGR` writer"]
pub type W = crate::W<TZSC_MPCWM4BCFGR_SPEC>;
#[doc = "Sub-region z enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SREN_A {
    #[doc = "0: Sub-region z is disabled. Access control of base region applies to any access between this sub-region start- and end-addresses."]
    B_0x0 = 0,
    #[doc = "1: Sub-region z is enabled. Access control defined in GTZC1_TZSC_MPCWMzCFGR applies to any access between this sub-region start- and end-addresses, both defined in GTZC1_TZSC_MPCWMAR and GTZC1_TZSC_MPCWMBR."]
    B_0x1 = 1,
}
impl From<SREN_A> for bool {
    #[inline(always)]
    fn from(variant: SREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SREN` reader - Sub-region z enable"]
pub type SREN_R = crate::BitReader<SREN_A>;
impl SREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SREN_A {
        match self.bits {
            false => SREN_A::B_0x0,
            true => SREN_A::B_0x1,
        }
    }
    #[doc = "Sub-region z is disabled. Access control of base region applies to any access between this sub-region start- and end-addresses."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SREN_A::B_0x0
    }
    #[doc = "Sub-region z is enabled. Access control defined in GTZC1_TZSC_MPCWMzCFGR applies to any access between this sub-region start- and end-addresses, both defined in GTZC1_TZSC_MPCWMAR and GTZC1_TZSC_MPCWMBR."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SREN_A::B_0x1
    }
}
#[doc = "Field `SREN` writer - Sub-region z enable"]
pub type SREN_W<'a, REG> = crate::BitWriter<'a, REG, SREN_A>;
impl<'a, REG> SREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sub-region z is disabled. Access control of base region applies to any access between this sub-region start- and end-addresses."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SREN_A::B_0x0)
    }
    #[doc = "Sub-region z is enabled. Access control defined in GTZC1_TZSC_MPCWMzCFGR applies to any access between this sub-region start- and end-addresses, both defined in GTZC1_TZSC_MPCWMAR and GTZC1_TZSC_MPCWMBR."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SREN_A::B_0x1)
    }
}
#[doc = "Sub-region z lock This bit, once set, can be cleared only by a system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRLOCK_A {
    #[doc = "0: GTZC1_TZSC_MPCWMzCFGR, GTZC1_TZSC_MPCWMAR and GTZC1_TZSC_MPCWMBR can be written."]
    B_0x0 = 0,
    #[doc = "1: Writes to GTZC1_TZSC_MPCWMzCFGR, GTZC1_TZSC_MPCWMAR and GTZC1_TZSC_MPCWMBR are ignored."]
    B_0x1 = 1,
}
impl From<SRLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SRLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRLOCK` reader - Sub-region z lock This bit, once set, can be cleared only by a system reset."]
pub type SRLOCK_R = crate::BitReader<SRLOCK_A>;
impl SRLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRLOCK_A {
        match self.bits {
            false => SRLOCK_A::B_0x0,
            true => SRLOCK_A::B_0x1,
        }
    }
    #[doc = "GTZC1_TZSC_MPCWMzCFGR, GTZC1_TZSC_MPCWMAR and GTZC1_TZSC_MPCWMBR can be written."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRLOCK_A::B_0x0
    }
    #[doc = "Writes to GTZC1_TZSC_MPCWMzCFGR, GTZC1_TZSC_MPCWMAR and GTZC1_TZSC_MPCWMBR are ignored."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRLOCK_A::B_0x1
    }
}
#[doc = "Field `SRLOCK` writer - Sub-region z lock This bit, once set, can be cleared only by a system reset."]
pub type SRLOCK_W<'a, REG> = crate::BitWriter<'a, REG, SRLOCK_A>;
impl<'a, REG> SRLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTZC1_TZSC_MPCWMzCFGR, GTZC1_TZSC_MPCWMAR and GTZC1_TZSC_MPCWMBR can be written."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRLOCK_A::B_0x0)
    }
    #[doc = "Writes to GTZC1_TZSC_MPCWMzCFGR, GTZC1_TZSC_MPCWMAR and GTZC1_TZSC_MPCWMBR are ignored."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRLOCK_A::B_0x1)
    }
}
#[doc = "Privileged sub-region z This bit is taken into account only if SREN is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV_A {
    #[doc = "0: Privileged and unprivileged accesses are granted in sub-region z."]
    B_0x0 = 0,
    #[doc = "1: Only privileged accesses are granted in sub-region z."]
    B_0x1 = 1,
}
impl From<PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV` reader - Privileged sub-region z This bit is taken into account only if SREN is set."]
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
    #[doc = "Privileged and unprivileged accesses are granted in sub-region z."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV_A::B_0x0
    }
    #[doc = "Only privileged accesses are granted in sub-region z."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV_A::B_0x1
    }
}
#[doc = "Field `PRIV` writer - Privileged sub-region z This bit is taken into account only if SREN is set."]
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG, PRIV_A>;
impl<'a, REG> PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged accesses are granted in sub-region z."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV_A::B_0x0)
    }
    #[doc = "Only privileged accesses are granted in sub-region z."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Sub-region z enable"]
    #[inline(always)]
    pub fn SREN(&self) -> SREN_R {
        SREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sub-region z lock This bit, once set, can be cleared only by a system reset."]
    #[inline(always)]
    pub fn SRLOCK(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Privileged sub-region z This bit is taken into account only if SREN is set."]
    #[inline(always)]
    pub fn PRIV(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub-region z enable"]
    #[inline(always)]
    pub fn SREN(&mut self) -> SREN_W<'_, TZSC_MPCWM4BCFGR_SPEC> {
        SREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sub-region z lock This bit, once set, can be cleared only by a system reset."]
    #[inline(always)]
    pub fn SRLOCK(&mut self) -> SRLOCK_W<'_, TZSC_MPCWM4BCFGR_SPEC> {
        SRLOCK_W::new(self, 1)
    }
    #[doc = "Bit 9 - Privileged sub-region z This bit is taken into account only if SREN is set."]
    #[inline(always)]
    pub fn PRIV(&mut self) -> PRIV_W<'_, TZSC_MPCWM4BCFGR_SPEC> {
        PRIV_W::new(self, 9)
    }
}
#[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4bcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4bcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_MPCWM4BCFGR_SPEC;
impl crate::RegisterSpec for TZSC_MPCWM4BCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_mpcwm4bcfgr::R`](R) reader structure"]
impl crate::Readable for TZSC_MPCWM4BCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzsc_mpcwm4bcfgr::W`](W) writer structure"]
impl crate::Writable for TZSC_MPCWM4BCFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TZSC_MPCWM4BCFGR to value 0"]
impl crate::Resettable for TZSC_MPCWM4BCFGR_SPEC {}
