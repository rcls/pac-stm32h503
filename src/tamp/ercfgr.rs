#[doc = "Register `ERCFGR` reader"]
pub type R = crate::R<ERCFGR_SPEC>;
#[doc = "Register `ERCFGR` writer"]
pub type W = crate::W<ERCFGR_SPEC>;
#[doc = "Configurable device secrets configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERCFG0_A {
    #[doc = "0: Configurable device secrets are not included in the device secrets protected by TAMP peripheral"]
    B_0x0 = 0,
    #[doc = "1: Configurable device secrets are is included in the device secrets protected by TAMP peripheral"]
    B_0x1 = 1,
}
impl From<ERCFG0_A> for bool {
    #[inline(always)]
    fn from(variant: ERCFG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERCFG0` reader - Configurable device secrets configuration"]
pub type ERCFG0_R = crate::BitReader<ERCFG0_A>;
impl ERCFG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERCFG0_A {
        match self.bits {
            false => ERCFG0_A::B_0x0,
            true => ERCFG0_A::B_0x1,
        }
    }
    #[doc = "Configurable device secrets are not included in the device secrets protected by TAMP peripheral"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ERCFG0_A::B_0x0
    }
    #[doc = "Configurable device secrets are is included in the device secrets protected by TAMP peripheral"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ERCFG0_A::B_0x1
    }
}
#[doc = "Field `ERCFG0` writer - Configurable device secrets configuration"]
pub type ERCFG0_W<'a, REG> = crate::BitWriter<'a, REG, ERCFG0_A>;
impl<'a, REG> ERCFG0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configurable device secrets are not included in the device secrets protected by TAMP peripheral"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ERCFG0_A::B_0x0)
    }
    #[doc = "Configurable device secrets are is included in the device secrets protected by TAMP peripheral"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ERCFG0_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Configurable device secrets configuration"]
    #[inline(always)]
    pub fn ERCFG0(&self) -> ERCFG0_R {
        ERCFG0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configurable device secrets configuration"]
    #[inline(always)]
    pub fn ERCFG0(&mut self) -> ERCFG0_W<'_, ERCFGR_SPEC> {
        ERCFG0_W::new(self, 0)
    }
}
#[doc = "TAMP erase configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ercfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ercfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERCFGR_SPEC;
impl crate::RegisterSpec for ERCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ercfgr::R`](R) reader structure"]
impl crate::Readable for ERCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ercfgr::W`](W) writer structure"]
impl crate::Writable for ERCFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ERCFGR to value 0"]
impl crate::Resettable for ERCFGR_SPEC {}
