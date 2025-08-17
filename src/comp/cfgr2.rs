#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2_SPEC>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2_SPEC>;
#[doc = "Field `INPSEL0` reader - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table 145: COMP1 noninverting input assignment for more details."]
pub type INPSEL0_R = crate::BitReader;
#[doc = "Field `INPSEL0` writer - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table 145: COMP1 noninverting input assignment for more details."]
pub type INPSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: COMP_CFGR2\\[31:0\\] register is read/write"]
    B_0x0 = 0,
    #[doc = "1: COMP_CFGR2\\[31:0\\] is read-only"]
    B_0x1 = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::B_0x0,
            true => LOCK_A::B_0x1,
        }
    }
    #[doc = "COMP_CFGR2\\[31:0\\] register is read/write"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LOCK_A::B_0x0
    }
    #[doc = "COMP_CFGR2\\[31:0\\] is read-only"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LOCK_A::B_0x1
    }
}
#[doc = "Field `LOCK` writer - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK_A>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP_CFGR2\\[31:0\\] register is read/write"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::B_0x0)
    }
    #[doc = "COMP_CFGR2\\[31:0\\] is read-only"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 4 - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table 145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn INPSEL0(&self) -> INPSEL0_R {
        INPSEL0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
    #[inline(always)]
    pub fn LOCK(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table 145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn INPSEL0(&mut self) -> INPSEL0_W<'_, CFGR2_SPEC> {
        INPSEL0_W::new(self, 4)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
    #[inline(always)]
    pub fn LOCK(&mut self) -> LOCK_W<'_, CFGR2_SPEC> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Comparator configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {}
