#[doc = "Register `MESR` reader"]
pub type R = crate::R<MESR_SPEC>;
#[doc = "Register `MESR` writer"]
pub type W = crate::W<MESR_SPEC>;
#[doc = "erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLR_A {
    #[doc = "0: Memory erase on going (if not yet cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: Memory erase done"]
    B_0x1 = 1,
}
impl From<MCLR_A> for bool {
    #[inline(always)]
    fn from(variant: MCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLR` reader - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
pub type MCLR_R = crate::BitReader<MCLR_A>;
impl MCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCLR_A {
        match self.bits {
            false => MCLR_A::B_0x0,
            true => MCLR_A::B_0x1,
        }
    }
    #[doc = "Memory erase on going (if not yet cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MCLR_A::B_0x0
    }
    #[doc = "Memory erase done"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MCLR_A::B_0x1
    }
}
#[doc = "Field `MCLR` writer - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
pub type MCLR_W<'a, REG> = crate::BitWriter<'a, REG, MCLR_A>;
impl<'a, REG> MCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory erase on going (if not yet cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCLR_A::B_0x0)
    }
    #[doc = "Memory erase done"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCLR_A::B_0x1)
    }
}
#[doc = "end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPMEE_A {
    #[doc = "0: ICACHE erase on going"]
    B_0x0 = 0,
    #[doc = "1: ICACHE erase ended"]
    B_0x1 = 1,
}
impl From<IPMEE_A> for bool {
    #[inline(always)]
    fn from(variant: IPMEE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPMEE` reader - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
pub type IPMEE_R = crate::BitReader<IPMEE_A>;
impl IPMEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IPMEE_A {
        match self.bits {
            false => IPMEE_A::B_0x0,
            true => IPMEE_A::B_0x1,
        }
    }
    #[doc = "ICACHE erase on going"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IPMEE_A::B_0x0
    }
    #[doc = "ICACHE erase ended"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IPMEE_A::B_0x1
    }
}
#[doc = "Field `IPMEE` writer - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
pub type IPMEE_W<'a, REG> = crate::BitWriter<'a, REG, IPMEE_A>;
impl<'a, REG> IPMEE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ICACHE erase on going"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IPMEE_A::B_0x0)
    }
    #[doc = "ICACHE erase ended"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IPMEE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
    #[inline(always)]
    pub fn MCLR(&self) -> MCLR_R {
        MCLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
    #[inline(always)]
    pub fn IPMEE(&self) -> IPMEE_R {
        IPMEE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
    #[inline(always)]
    pub fn MCLR(&mut self) -> MCLR_W<'_, MESR_SPEC> {
        MCLR_W::new(self, 0)
    }
    #[doc = "Bit 16 - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
    #[inline(always)]
    pub fn IPMEE(&mut self) -> IPMEE_W<'_, MESR_SPEC> {
        IPMEE_W::new(self, 16)
    }
}
#[doc = "SBS memory erase status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mesr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mesr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MESR_SPEC;
impl crate::RegisterSpec for MESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mesr::R`](R) reader structure"]
impl crate::Readable for MESR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mesr::W`](W) writer structure"]
impl crate::Writable for MESR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MESR to value 0"]
impl crate::Resettable for MESR_SPEC {}
