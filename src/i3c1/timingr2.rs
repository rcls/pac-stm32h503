#[doc = "Register `TIMINGR2` reader"]
pub type R = crate::R<TIMINGR2_SPEC>;
#[doc = "Register `TIMINGR2` writer"]
pub type W = crate::W<TIMINGR2_SPEC>;
#[doc = "Field `STALLT` reader - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
pub type STALLT_R = crate::BitReader;
#[doc = "Field `STALLT` writer - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
pub type STALLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLD` reader - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
pub type STALLD_R = crate::BitReader;
#[doc = "Field `STALLD` writer - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
pub type STALLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLC` reader - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
pub type STALLC_R = crate::BitReader;
#[doc = "Field `STALLC` writer - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
pub type STALLC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STALLA_A {
    #[doc = "0: no stall"]
    B_0x0 = 0,
    #[doc = "1: stall enabled"]
    B_0x1 = 1,
}
impl From<STALLA_A> for bool {
    #[inline(always)]
    fn from(variant: STALLA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALLA` reader - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
pub type STALLA_R = crate::BitReader<STALLA_A>;
impl STALLA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STALLA_A {
        match self.bits {
            false => STALLA_A::B_0x0,
            true => STALLA_A::B_0x1,
        }
    }
    #[doc = "no stall"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STALLA_A::B_0x0
    }
    #[doc = "stall enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STALLA_A::B_0x1
    }
}
#[doc = "Field `STALLA` writer - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
pub type STALLA_W<'a, REG> = crate::BitWriter<'a, REG, STALLA_A>;
impl<'a, REG> STALLA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no stall"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(STALLA_A::B_0x0)
    }
    #[doc = "stall enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(STALLA_A::B_0x1)
    }
}
#[doc = "Field `STALL` reader - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK"]
pub type STALL_R = crate::FieldReader;
#[doc = "Field `STALL` writer - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK"]
pub type STALL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
    #[inline(always)]
    pub fn STALLT(&self) -> STALLT_R {
        STALLT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
    #[inline(always)]
    pub fn STALLD(&self) -> STALLD_R {
        STALLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
    #[inline(always)]
    pub fn STALLC(&self) -> STALLC_R {
        STALLC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
    #[inline(always)]
    pub fn STALLA(&self) -> STALLA_R {
        STALLA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK"]
    #[inline(always)]
    pub fn STALL(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
    #[inline(always)]
    pub fn STALLT(&mut self) -> STALLT_W<'_, TIMINGR2_SPEC> {
        STALLT_W::new(self, 0)
    }
    #[doc = "Bit 1 - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
    #[inline(always)]
    pub fn STALLD(&mut self) -> STALLD_W<'_, TIMINGR2_SPEC> {
        STALLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
    #[inline(always)]
    pub fn STALLC(&mut self) -> STALLC_W<'_, TIMINGR2_SPEC> {
        STALLC_W::new(self, 2)
    }
    #[doc = "Bit 3 - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
    #[inline(always)]
    pub fn STALLA(&mut self) -> STALLA_W<'_, TIMINGR2_SPEC> {
        STALLA_W::new(self, 3)
    }
    #[doc = "Bits 8:15 - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK"]
    #[inline(always)]
    pub fn STALL(&mut self) -> STALL_W<'_, TIMINGR2_SPEC> {
        STALL_W::new(self, 8)
    }
}
#[doc = "I3C timing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`timingr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMINGR2_SPEC;
impl crate::RegisterSpec for TIMINGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timingr2::R`](R) reader structure"]
impl crate::Readable for TIMINGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timingr2::W`](W) writer structure"]
impl crate::Writable for TIMINGR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TIMINGR2 to value 0"]
impl crate::Resettable for TIMINGR2_SPEC {}
