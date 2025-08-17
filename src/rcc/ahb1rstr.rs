#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<AHB1RSTR_SPEC>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<AHB1RSTR_SPEC>;
#[doc = "GPDMA1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1RST_A {
    #[doc = "0: does not reset GPDMA1 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets GPDMA1 block"]
    B_0x1 = 1,
}
impl From<GPDMA1RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPDMA1RST` reader - GPDMA1 block reset Set and reset by software."]
pub type GPDMA1RST_R = crate::BitReader<GPDMA1RST_A>;
impl GPDMA1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1RST_A {
        match self.bits {
            false => GPDMA1RST_A::B_0x0,
            true => GPDMA1RST_A::B_0x1,
        }
    }
    #[doc = "does not reset GPDMA1 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPDMA1RST_A::B_0x0
    }
    #[doc = "resets GPDMA1 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPDMA1RST_A::B_0x1
    }
}
#[doc = "Field `GPDMA1RST` writer - GPDMA1 block reset Set and reset by software."]
pub type GPDMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1RST_A>;
impl<'a, REG> GPDMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset GPDMA1 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1RST_A::B_0x0)
    }
    #[doc = "resets GPDMA1 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1RST_A::B_0x1)
    }
}
#[doc = "GPDMA2 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA2RST_A {
    #[doc = "0: does not reset GPDMA2 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets GPDMA2 block"]
    B_0x1 = 1,
}
impl From<GPDMA2RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPDMA2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPDMA2RST` reader - GPDMA2 block reset Set and reset by software."]
pub type GPDMA2RST_R = crate::BitReader<GPDMA2RST_A>;
impl GPDMA2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA2RST_A {
        match self.bits {
            false => GPDMA2RST_A::B_0x0,
            true => GPDMA2RST_A::B_0x1,
        }
    }
    #[doc = "does not reset GPDMA2 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPDMA2RST_A::B_0x0
    }
    #[doc = "resets GPDMA2 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPDMA2RST_A::B_0x1
    }
}
#[doc = "Field `GPDMA2RST` writer - GPDMA2 block reset Set and reset by software."]
pub type GPDMA2RST_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA2RST_A>;
impl<'a, REG> GPDMA2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset GPDMA2 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA2RST_A::B_0x0)
    }
    #[doc = "resets GPDMA2 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA2RST_A::B_0x1)
    }
}
#[doc = "CRC block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCRST_A {
    #[doc = "0: does not reset CRC block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets CRC block"]
    B_0x1 = 1,
}
impl From<CRCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCRST` reader - CRC block reset Set and reset by software."]
pub type CRCRST_R = crate::BitReader<CRCRST_A>;
impl CRCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCRST_A {
        match self.bits {
            false => CRCRST_A::B_0x0,
            true => CRCRST_A::B_0x1,
        }
    }
    #[doc = "does not reset CRC block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRCRST_A::B_0x0
    }
    #[doc = "resets CRC block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRCRST_A::B_0x1
    }
}
#[doc = "Field `CRCRST` writer - CRC block reset Set and reset by software."]
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG, CRCRST_A>;
impl<'a, REG> CRCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset CRC block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCRST_A::B_0x0)
    }
    #[doc = "resets CRC block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCRST_A::B_0x1)
    }
}
#[doc = "RAMCFG block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMCFGRST_A {
    #[doc = "0: does not reset RAMCFG block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets RAMCFG block"]
    B_0x1 = 1,
}
impl From<RAMCFGRST_A> for bool {
    #[inline(always)]
    fn from(variant: RAMCFGRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMCFGRST` reader - RAMCFG block reset Set and reset by software."]
pub type RAMCFGRST_R = crate::BitReader<RAMCFGRST_A>;
impl RAMCFGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMCFGRST_A {
        match self.bits {
            false => RAMCFGRST_A::B_0x0,
            true => RAMCFGRST_A::B_0x1,
        }
    }
    #[doc = "does not reset RAMCFG block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RAMCFGRST_A::B_0x0
    }
    #[doc = "resets RAMCFG block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RAMCFGRST_A::B_0x1
    }
}
#[doc = "Field `RAMCFGRST` writer - RAMCFG block reset Set and reset by software."]
pub type RAMCFGRST_W<'a, REG> = crate::BitWriter<'a, REG, RAMCFGRST_A>;
impl<'a, REG> RAMCFGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset RAMCFG block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGRST_A::B_0x0)
    }
    #[doc = "resets RAMCFG block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGRST_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - GPDMA1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA1RST(&self) -> GPDMA1RST_R {
        GPDMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDMA2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA2RST(&self) -> GPDMA2RST_R {
        GPDMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC block reset Set and reset by software."]
    #[inline(always)]
    pub fn CRCRST(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG block reset Set and reset by software."]
    #[inline(always)]
    pub fn RAMCFGRST(&self) -> RAMCFGRST_R {
        RAMCFGRST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA1RST(&mut self) -> GPDMA1RST_W<'_, AHB1RSTR_SPEC> {
        GPDMA1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPDMA2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA2RST(&mut self) -> GPDMA2RST_W<'_, AHB1RSTR_SPEC> {
        GPDMA2RST_W::new(self, 1)
    }
    #[doc = "Bit 12 - CRC block reset Set and reset by software."]
    #[inline(always)]
    pub fn CRCRST(&mut self) -> CRCRST_W<'_, AHB1RSTR_SPEC> {
        CRCRST_W::new(self, 12)
    }
    #[doc = "Bit 17 - RAMCFG block reset Set and reset by software."]
    #[inline(always)]
    pub fn RAMCFGRST(&mut self) -> RAMCFGRST_W<'_, AHB1RSTR_SPEC> {
        RAMCFGRST_W::new(self, 17)
    }
}
#[doc = "RCC AHB1 reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1RSTR_SPEC;
impl crate::RegisterSpec for AHB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rstr::R`](R) reader structure"]
impl crate::Readable for AHB1RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure"]
impl crate::Writable for AHB1RSTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTR_SPEC {}
