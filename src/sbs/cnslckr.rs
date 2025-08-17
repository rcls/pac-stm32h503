#[doc = "Register `CNSLCKR` reader"]
pub type R = crate::R<CNSLCKR_SPEC>;
#[doc = "Register `CNSLCKR` writer"]
pub type W = crate::W<CNSLCKR_SPEC>;
#[doc = "VTOR_NS register lock This bit is set by software and cleared only by a system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKNSVTOR_A {
    #[doc = "0: VTOR_NS register write enabled"]
    B_0x0 = 0,
    #[doc = "1: VTOR_NS register write disabled"]
    B_0x1 = 1,
}
impl From<LOCKNSVTOR_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKNSVTOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKNSVTOR` reader - VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
pub type LOCKNSVTOR_R = crate::BitReader<LOCKNSVTOR_A>;
impl LOCKNSVTOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCKNSVTOR_A {
        match self.bits {
            false => LOCKNSVTOR_A::B_0x0,
            true => LOCKNSVTOR_A::B_0x1,
        }
    }
    #[doc = "VTOR_NS register write enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LOCKNSVTOR_A::B_0x0
    }
    #[doc = "VTOR_NS register write disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LOCKNSVTOR_A::B_0x1
    }
}
#[doc = "Field `LOCKNSVTOR` writer - VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
pub type LOCKNSVTOR_W<'a, REG> = crate::BitWriter<'a, REG, LOCKNSVTOR_A>;
impl<'a, REG> LOCKNSVTOR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VTOR_NS register write enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSVTOR_A::B_0x0)
    }
    #[doc = "VTOR_NS register write disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSVTOR_A::B_0x1)
    }
}
#[doc = "MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKNSMPU_A {
    #[doc = "0: MPU registers write enabled"]
    B_0x0 = 0,
    #[doc = "1: MPU registers write disabled"]
    B_0x1 = 1,
}
impl From<LOCKNSMPU_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKNSMPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKNSMPU` reader - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
pub type LOCKNSMPU_R = crate::BitReader<LOCKNSMPU_A>;
impl LOCKNSMPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCKNSMPU_A {
        match self.bits {
            false => LOCKNSMPU_A::B_0x0,
            true => LOCKNSMPU_A::B_0x1,
        }
    }
    #[doc = "MPU registers write enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LOCKNSMPU_A::B_0x0
    }
    #[doc = "MPU registers write disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LOCKNSMPU_A::B_0x1
    }
}
#[doc = "Field `LOCKNSMPU` writer - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
pub type LOCKNSMPU_W<'a, REG> = crate::BitWriter<'a, REG, LOCKNSMPU_A>;
impl<'a, REG> LOCKNSMPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPU registers write enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSMPU_A::B_0x0)
    }
    #[doc = "MPU registers write disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKNSMPU_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
    #[inline(always)]
    pub fn LOCKNSVTOR(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
    #[inline(always)]
    pub fn LOCKNSMPU(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
    #[inline(always)]
    pub fn LOCKNSVTOR(&mut self) -> LOCKNSVTOR_W<'_, CNSLCKR_SPEC> {
        LOCKNSVTOR_W::new(self, 0)
    }
    #[doc = "Bit 1 - MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
    #[inline(always)]
    pub fn LOCKNSMPU(&mut self) -> LOCKNSMPU_W<'_, CNSLCKR_SPEC> {
        LOCKNSMPU_W::new(self, 1)
    }
}
#[doc = "SBS CPU lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnslckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnslckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNSLCKR_SPEC;
impl crate::RegisterSpec for CNSLCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnslckr::R`](R) reader structure"]
impl crate::Readable for CNSLCKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnslckr::W`](W) writer structure"]
impl crate::Writable for CNSLCKR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CNSLCKR to value 0"]
impl crate::Resettable for CNSLCKR_SPEC {}
