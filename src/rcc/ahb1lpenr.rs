#[doc = "Register `AHB1LPENR` reader"]
pub type R = crate::R<AHB1LPENR_SPEC>;
#[doc = "Register `AHB1LPENR` writer"]
pub type W = crate::W<AHB1LPENR_SPEC>;
#[doc = "GPDMA1 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1LPEN_A {
    #[doc = "0: GPDMA1 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: GPDMA1 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<GPDMA1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPDMA1LPEN` reader - GPDMA1 clock enable during sleep mode Set and reset by software."]
pub type GPDMA1LPEN_R = crate::BitReader<GPDMA1LPEN_A>;
impl GPDMA1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1LPEN_A {
        match self.bits {
            false => GPDMA1LPEN_A::B_0x0,
            true => GPDMA1LPEN_A::B_0x1,
        }
    }
    #[doc = "GPDMA1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPDMA1LPEN_A::B_0x0
    }
    #[doc = "GPDMA1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPDMA1LPEN_A::B_0x1
    }
}
#[doc = "Field `GPDMA1LPEN` writer - GPDMA1 clock enable during sleep mode Set and reset by software."]
pub type GPDMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1LPEN_A>;
impl<'a, REG> GPDMA1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPDMA1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1LPEN_A::B_0x0)
    }
    #[doc = "GPDMA1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1LPEN_A::B_0x1)
    }
}
#[doc = "GPDMA2 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA2LPEN_A {
    #[doc = "0: GPDMA2 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: GPDMA2 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<GPDMA2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPDMA2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPDMA2LPEN` reader - GPDMA2 clock enable during sleep mode Set and reset by software."]
pub type GPDMA2LPEN_R = crate::BitReader<GPDMA2LPEN_A>;
impl GPDMA2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA2LPEN_A {
        match self.bits {
            false => GPDMA2LPEN_A::B_0x0,
            true => GPDMA2LPEN_A::B_0x1,
        }
    }
    #[doc = "GPDMA2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPDMA2LPEN_A::B_0x0
    }
    #[doc = "GPDMA2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPDMA2LPEN_A::B_0x1
    }
}
#[doc = "Field `GPDMA2LPEN` writer - GPDMA2 clock enable during sleep mode Set and reset by software."]
pub type GPDMA2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA2LPEN_A>;
impl<'a, REG> GPDMA2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPDMA2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA2LPEN_A::B_0x0)
    }
    #[doc = "GPDMA2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA2LPEN_A::B_0x1)
    }
}
#[doc = "Flash interface (FLITF) clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLITFLPEN_A {
    #[doc = "0: FLITF peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: FLITF peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<FLITFLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLITFLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLITFLPEN` reader - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
pub type FLITFLPEN_R = crate::BitReader<FLITFLPEN_A>;
impl FLITFLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLITFLPEN_A {
        match self.bits {
            false => FLITFLPEN_A::B_0x0,
            true => FLITFLPEN_A::B_0x1,
        }
    }
    #[doc = "FLITF peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FLITFLPEN_A::B_0x0
    }
    #[doc = "FLITF peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FLITFLPEN_A::B_0x1
    }
}
#[doc = "Field `FLITFLPEN` writer - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
pub type FLITFLPEN_W<'a, REG> = crate::BitWriter<'a, REG, FLITFLPEN_A>;
impl<'a, REG> FLITFLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLITF peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FLITFLPEN_A::B_0x0)
    }
    #[doc = "FLITF peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FLITFLPEN_A::B_0x1)
    }
}
#[doc = "CRC clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCLPEN_A {
    #[doc = "0: CRC peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: CRC peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<CRCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCLPEN` reader - CRC clock enable during sleep mode Set and reset by software."]
pub type CRCLPEN_R = crate::BitReader<CRCLPEN_A>;
impl CRCLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCLPEN_A {
        match self.bits {
            false => CRCLPEN_A::B_0x0,
            true => CRCLPEN_A::B_0x1,
        }
    }
    #[doc = "CRC peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRCLPEN_A::B_0x0
    }
    #[doc = "CRC peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRCLPEN_A::B_0x1
    }
}
#[doc = "Field `CRCLPEN` writer - CRC clock enable during sleep mode Set and reset by software."]
pub type CRCLPEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCLPEN_A>;
impl<'a, REG> CRCLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCLPEN_A::B_0x0)
    }
    #[doc = "CRC peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCLPEN_A::B_0x1)
    }
}
#[doc = "RAMCFG clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMCFGLPEN_A {
    #[doc = "0: RAMCFG peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: RAMCFG peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<RAMCFGLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RAMCFGLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMCFGLPEN` reader - RAMCFG clock enable during sleep mode Set and reset by software."]
pub type RAMCFGLPEN_R = crate::BitReader<RAMCFGLPEN_A>;
impl RAMCFGLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMCFGLPEN_A {
        match self.bits {
            false => RAMCFGLPEN_A::B_0x0,
            true => RAMCFGLPEN_A::B_0x1,
        }
    }
    #[doc = "RAMCFG peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RAMCFGLPEN_A::B_0x0
    }
    #[doc = "RAMCFG peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RAMCFGLPEN_A::B_0x1
    }
}
#[doc = "Field `RAMCFGLPEN` writer - RAMCFG clock enable during sleep mode Set and reset by software."]
pub type RAMCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG, RAMCFGLPEN_A>;
impl<'a, REG> RAMCFGLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAMCFG peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGLPEN_A::B_0x0)
    }
    #[doc = "RAMCFG peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGLPEN_A::B_0x1)
    }
}
#[doc = "BKPRAM clock enable during sleep mode Set and reset by software\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKPRAMLPEN_A {
    #[doc = "0: BKPRAM peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: BKPRAM peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<BKPRAMLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: BKPRAMLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPRAMLPEN` reader - BKPRAM clock enable during sleep mode Set and reset by software"]
pub type BKPRAMLPEN_R = crate::BitReader<BKPRAMLPEN_A>;
impl BKPRAMLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKPRAMLPEN_A {
        match self.bits {
            false => BKPRAMLPEN_A::B_0x0,
            true => BKPRAMLPEN_A::B_0x1,
        }
    }
    #[doc = "BKPRAM peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKPRAMLPEN_A::B_0x0
    }
    #[doc = "BKPRAM peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKPRAMLPEN_A::B_0x1
    }
}
#[doc = "Field `BKPRAMLPEN` writer - BKPRAM clock enable during sleep mode Set and reset by software"]
pub type BKPRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG, BKPRAMLPEN_A>;
impl<'a, REG> BKPRAMLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKPRAM peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKPRAMLPEN_A::B_0x0)
    }
    #[doc = "BKPRAM peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKPRAMLPEN_A::B_0x1)
    }
}
#[doc = "ICACHE clock enable during sleep mode Set and reset by software\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICACHELPEN_A {
    #[doc = "0: ICACHE peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: ICACHE peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<ICACHELPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHELPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICACHELPEN` reader - ICACHE clock enable during sleep mode Set and reset by software"]
pub type ICACHELPEN_R = crate::BitReader<ICACHELPEN_A>;
impl ICACHELPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICACHELPEN_A {
        match self.bits {
            false => ICACHELPEN_A::B_0x0,
            true => ICACHELPEN_A::B_0x1,
        }
    }
    #[doc = "ICACHE peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ICACHELPEN_A::B_0x0
    }
    #[doc = "ICACHE peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ICACHELPEN_A::B_0x1
    }
}
#[doc = "Field `ICACHELPEN` writer - ICACHE clock enable during sleep mode Set and reset by software"]
pub type ICACHELPEN_W<'a, REG> = crate::BitWriter<'a, REG, ICACHELPEN_A>;
impl<'a, REG> ICACHELPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ICACHE peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ICACHELPEN_A::B_0x0)
    }
    #[doc = "ICACHE peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ICACHELPEN_A::B_0x1)
    }
}
#[doc = "SRAM1 clock enable during sleep mode Set and reset by software\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1LPEN_A {
    #[doc = "0: SRAM1 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: SRAM1 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<SRAM1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1LPEN` reader - SRAM1 clock enable during sleep mode Set and reset by software"]
pub type SRAM1LPEN_R = crate::BitReader<SRAM1LPEN_A>;
impl SRAM1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1LPEN_A {
        match self.bits {
            false => SRAM1LPEN_A::B_0x0,
            true => SRAM1LPEN_A::B_0x1,
        }
    }
    #[doc = "SRAM1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM1LPEN_A::B_0x0
    }
    #[doc = "SRAM1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM1LPEN_A::B_0x1
    }
}
#[doc = "Field `SRAM1LPEN` writer - SRAM1 clock enable during sleep mode Set and reset by software"]
pub type SRAM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1LPEN_A>;
impl<'a, REG> SRAM1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1LPEN_A::B_0x0)
    }
    #[doc = "SRAM1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1LPEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - GPDMA1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA1LPEN(&self) -> GPDMA1LPEN_R {
        GPDMA1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA2LPEN(&self) -> GPDMA2LPEN_R {
        GPDMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn FLITFLPEN(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn CRCLPEN(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn RAMCFGLPEN(&self) -> RAMCFGLPEN_R {
        RAMCFGLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 28 - BKPRAM clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn BKPRAMLPEN(&self) -> BKPRAMLPEN_R {
        BKPRAMLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ICACHE clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn ICACHELPEN(&self) -> ICACHELPEN_R {
        ICACHELPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM1 clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn SRAM1LPEN(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA1LPEN(&mut self) -> GPDMA1LPEN_W<'_, AHB1LPENR_SPEC> {
        GPDMA1LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA2LPEN(&mut self) -> GPDMA2LPEN_W<'_, AHB1LPENR_SPEC> {
        GPDMA2LPEN_W::new(self, 1)
    }
    #[doc = "Bit 8 - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn FLITFLPEN(&mut self) -> FLITFLPEN_W<'_, AHB1LPENR_SPEC> {
        FLITFLPEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn CRCLPEN(&mut self) -> CRCLPEN_W<'_, AHB1LPENR_SPEC> {
        CRCLPEN_W::new(self, 12)
    }
    #[doc = "Bit 17 - RAMCFG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn RAMCFGLPEN(&mut self) -> RAMCFGLPEN_W<'_, AHB1LPENR_SPEC> {
        RAMCFGLPEN_W::new(self, 17)
    }
    #[doc = "Bit 28 - BKPRAM clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn BKPRAMLPEN(&mut self) -> BKPRAMLPEN_W<'_, AHB1LPENR_SPEC> {
        BKPRAMLPEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - ICACHE clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn ICACHELPEN(&mut self) -> ICACHELPEN_W<'_, AHB1LPENR_SPEC> {
        ICACHELPEN_W::new(self, 29)
    }
    #[doc = "Bit 31 - SRAM1 clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn SRAM1LPEN(&mut self) -> SRAM1LPEN_W<'_, AHB1LPENR_SPEC> {
        SRAM1LPEN_W::new(self, 31)
    }
}
#[doc = "RCC AHB1 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1LPENR_SPEC;
impl crate::RegisterSpec for AHB1LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1lpenr::R`](R) reader structure"]
impl crate::Readable for AHB1LPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb1lpenr::W`](W) writer structure"]
impl crate::Writable for AHB1LPENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AHB1LPENR to value 0xffff_ffff"]
impl crate::Resettable for AHB1LPENR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
