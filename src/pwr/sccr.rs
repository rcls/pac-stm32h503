#[doc = "Register `SCCR` reader"]
pub type R = crate::R<SCCR_SPEC>;
#[doc = "Register `SCCR` writer"]
pub type W = crate::W<SCCR_SPEC>;
#[doc = "power management unit bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_A {
    #[doc = "0: Power management unit normal operation. Use the internal regulator."]
    B_0x0 = 0,
    #[doc = "1: Power management unit bypassed. Use the external power (voltage monitoring still active)"]
    B_0x1 = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - power management unit bypass"]
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
impl BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::B_0x0,
            true => BYPASS_A::B_0x1,
        }
    }
    #[doc = "Power management unit normal operation. Use the internal regulator."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BYPASS_A::B_0x0
    }
    #[doc = "Power management unit bypassed. Use the external power (voltage monitoring still active)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BYPASS_A::B_0x1
    }
}
#[doc = "Field `BYPASS` writer - power management unit bypass"]
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG, BYPASS_A>;
impl<'a, REG> BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power management unit normal operation. Use the internal regulator."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_A::B_0x0)
    }
    #[doc = "Power management unit bypassed. Use the external power (voltage monitoring still active)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_A::B_0x1)
    }
}
#[doc = "Field `LDOEN` reader - LDO enable The value is set by hardware when the package uses the LDO regulator."]
pub type LDOEN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - power management unit bypass"]
    #[inline(always)]
    pub fn BYPASS(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - LDO enable The value is set by hardware when the package uses the LDO regulator."]
    #[inline(always)]
    pub fn LDOEN(&self) -> LDOEN_R {
        LDOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - power management unit bypass"]
    #[inline(always)]
    pub fn BYPASS(&mut self) -> BYPASS_W<'_, SCCR_SPEC> {
        BYPASS_W::new(self, 0)
    }
}
#[doc = "PWR supply configuration control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCR_SPEC;
impl crate::RegisterSpec for SCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sccr::R`](R) reader structure"]
impl crate::Readable for SCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sccr::W`](W) writer structure"]
impl crate::Writable for SCCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SCCR to value 0x0100"]
impl crate::Resettable for SCCR_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
