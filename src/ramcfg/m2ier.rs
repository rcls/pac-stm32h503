#[doc = "Register `M2IER` reader"]
pub type R = crate::R<M2IER_SPEC>;
#[doc = "Register `M2IER` writer"]
pub type W = crate::W<M2IER_SPEC>;
#[doc = "ECC single error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEIE_A {
    #[doc = "0: Single error interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Single error interrupt enabled"]
    B_0x1 = 1,
}
impl From<SEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEIE` reader - ECC single error interrupt enable"]
pub type SEIE_R = crate::BitReader<SEIE_A>;
impl SEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEIE_A {
        match self.bits {
            false => SEIE_A::B_0x0,
            true => SEIE_A::B_0x1,
        }
    }
    #[doc = "Single error interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SEIE_A::B_0x0
    }
    #[doc = "Single error interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SEIE_A::B_0x1
    }
}
#[doc = "Field `SEIE` writer - ECC single error interrupt enable"]
pub type SEIE_W<'a, REG> = crate::BitWriter<'a, REG, SEIE_A>;
impl<'a, REG> SEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single error interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SEIE_A::B_0x0)
    }
    #[doc = "Single error interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SEIE_A::B_0x1)
    }
}
#[doc = "ECC double error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEIE_A {
    #[doc = "0: Double error interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Double error interrupt enabled"]
    B_0x1 = 1,
}
impl From<DEIE_A> for bool {
    #[inline(always)]
    fn from(variant: DEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEIE` reader - ECC double error interrupt enable"]
pub type DEIE_R = crate::BitReader<DEIE_A>;
impl DEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEIE_A {
        match self.bits {
            false => DEIE_A::B_0x0,
            true => DEIE_A::B_0x1,
        }
    }
    #[doc = "Double error interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DEIE_A::B_0x0
    }
    #[doc = "Double error interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DEIE_A::B_0x1
    }
}
#[doc = "Field `DEIE` writer - ECC double error interrupt enable"]
pub type DEIE_W<'a, REG> = crate::BitWriter<'a, REG, DEIE_A>;
impl<'a, REG> DEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Double error interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DEIE_A::B_0x0)
    }
    #[doc = "Double error interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DEIE_A::B_0x1)
    }
}
#[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCNMI_A {
    #[doc = "0: NMI not generated in case of ECC double error"]
    B_0x0 = 0,
    #[doc = "1: NMI generated in case of ECC double error"]
    B_0x1 = 1,
}
impl From<ECCNMI_A> for bool {
    #[inline(always)]
    fn from(variant: ECCNMI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCNMI` reader - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
pub type ECCNMI_R = crate::BitReader<ECCNMI_A>;
impl ECCNMI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCNMI_A {
        match self.bits {
            false => ECCNMI_A::B_0x0,
            true => ECCNMI_A::B_0x1,
        }
    }
    #[doc = "NMI not generated in case of ECC double error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ECCNMI_A::B_0x0
    }
    #[doc = "NMI generated in case of ECC double error"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ECCNMI_A::B_0x1
    }
}
#[doc = "Field `ECCNMI` writer - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
pub type ECCNMI_W<'a, REG> = crate::BitWriter<'a, REG, ECCNMI_A>;
impl<'a, REG> ECCNMI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NMI not generated in case of ECC double error"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCNMI_A::B_0x0)
    }
    #[doc = "NMI generated in case of ECC double error"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCNMI_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn SEIE(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn DEIE(&self) -> DEIE_R {
        DEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
    #[inline(always)]
    pub fn ECCNMI(&self) -> ECCNMI_R {
        ECCNMI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn SEIE(&mut self) -> SEIE_W<'_, M2IER_SPEC> {
        SEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn DEIE(&mut self) -> DEIE_W<'_, M2IER_SPEC> {
        DEIE_W::new(self, 1)
    }
    #[doc = "Bit 3 - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
    #[inline(always)]
    pub fn ECCNMI(&mut self) -> ECCNMI_W<'_, M2IER_SPEC> {
        ECCNMI_W::new(self, 3)
    }
}
#[doc = "RAMCFG memory 2 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2IER_SPEC;
impl crate::RegisterSpec for M2IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2ier::R`](R) reader structure"]
impl crate::Readable for M2IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m2ier::W`](W) writer structure"]
impl crate::Writable for M2IER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets M2IER to value 0"]
impl crate::Resettable for M2IER_SPEC {}
