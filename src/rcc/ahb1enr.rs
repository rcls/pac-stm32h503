#[doc = "Register `AHB1ENR` reader"]
pub type R = crate::R<AHB1ENR_SPEC>;
#[doc = "Register `AHB1ENR` writer"]
pub type W = crate::W<AHB1ENR_SPEC>;
#[doc = "GPDMA1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1EN_A {
    #[doc = "0: GPDMA1 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: GPDMA1 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<GPDMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPDMA1EN` reader - GPDMA1 clock enable Set and reset by software."]
pub type GPDMA1EN_R = crate::BitReader<GPDMA1EN_A>;
impl GPDMA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1EN_A {
        match self.bits {
            false => GPDMA1EN_A::B_0x0,
            true => GPDMA1EN_A::B_0x1,
        }
    }
    #[doc = "GPDMA1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPDMA1EN_A::B_0x0
    }
    #[doc = "GPDMA1 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPDMA1EN_A::B_0x1
    }
}
#[doc = "Field `GPDMA1EN` writer - GPDMA1 clock enable Set and reset by software."]
pub type GPDMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1EN_A>;
impl<'a, REG> GPDMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPDMA1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1EN_A::B_0x0)
    }
    #[doc = "GPDMA1 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1EN_A::B_0x1)
    }
}
#[doc = "GPDMA2 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA2EN_A {
    #[doc = "0: GPDMA2 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: GPDMA2 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<GPDMA2EN_A> for bool {
    #[inline(always)]
    fn from(variant: GPDMA2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPDMA2EN` reader - GPDMA2 clock enable Set and reset by software."]
pub type GPDMA2EN_R = crate::BitReader<GPDMA2EN_A>;
impl GPDMA2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA2EN_A {
        match self.bits {
            false => GPDMA2EN_A::B_0x0,
            true => GPDMA2EN_A::B_0x1,
        }
    }
    #[doc = "GPDMA2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPDMA2EN_A::B_0x0
    }
    #[doc = "GPDMA2 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPDMA2EN_A::B_0x1
    }
}
#[doc = "Field `GPDMA2EN` writer - GPDMA2 clock enable Set and reset by software."]
pub type GPDMA2EN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA2EN_A>;
impl<'a, REG> GPDMA2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPDMA2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA2EN_A::B_0x0)
    }
    #[doc = "GPDMA2 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA2EN_A::B_0x1)
    }
}
#[doc = "Flash interface clock enable Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLITFEN_A {
    #[doc = "0: FLASH interface clock disabled"]
    B_0x0 = 0,
    #[doc = "1: FLASH interface clock enabled (default after reset)"]
    B_0x1 = 1,
}
impl From<FLITFEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLITFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLITFEN` reader - Flash interface clock enable Set and reset by software."]
pub type FLITFEN_R = crate::BitReader<FLITFEN_A>;
impl FLITFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLITFEN_A {
        match self.bits {
            false => FLITFEN_A::B_0x0,
            true => FLITFEN_A::B_0x1,
        }
    }
    #[doc = "FLASH interface clock disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FLITFEN_A::B_0x0
    }
    #[doc = "FLASH interface clock enabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FLITFEN_A::B_0x1
    }
}
#[doc = "Field `FLITFEN` writer - Flash interface clock enable Set and reset by software."]
pub type FLITFEN_W<'a, REG> = crate::BitWriter<'a, REG, FLITFEN_A>;
impl<'a, REG> FLITFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLASH interface clock disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FLITFEN_A::B_0x0)
    }
    #[doc = "FLASH interface clock enabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FLITFEN_A::B_0x1)
    }
}
#[doc = "CRC clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN_A {
    #[doc = "0: CRC peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: CRC peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEN` reader - CRC clock enable Set and reset by software."]
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
impl CRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::B_0x0,
            true => CRCEN_A::B_0x1,
        }
    }
    #[doc = "CRC peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRCEN_A::B_0x0
    }
    #[doc = "CRC peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRCEN_A::B_0x1
    }
}
#[doc = "Field `CRCEN` writer - CRC clock enable Set and reset by software."]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCEN_A>;
impl<'a, REG> CRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN_A::B_0x0)
    }
    #[doc = "CRC peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN_A::B_0x1)
    }
}
#[doc = "RAMCFG clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMCFGEN_A {
    #[doc = "0: RAMCFG peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: RAMCFG peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<RAMCFGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RAMCFGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMCFGEN` reader - RAMCFG clock enable Set and reset by software."]
pub type RAMCFGEN_R = crate::BitReader<RAMCFGEN_A>;
impl RAMCFGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMCFGEN_A {
        match self.bits {
            false => RAMCFGEN_A::B_0x0,
            true => RAMCFGEN_A::B_0x1,
        }
    }
    #[doc = "RAMCFG peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RAMCFGEN_A::B_0x0
    }
    #[doc = "RAMCFG peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RAMCFGEN_A::B_0x1
    }
}
#[doc = "Field `RAMCFGEN` writer - RAMCFG clock enable Set and reset by software."]
pub type RAMCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, RAMCFGEN_A>;
impl<'a, REG> RAMCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAMCFG peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGEN_A::B_0x0)
    }
    #[doc = "RAMCFG peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGEN_A::B_0x1)
    }
}
#[doc = "BKPRAM clock enable Set and reset by software\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKPRAMEN_A {
    #[doc = "0: BKPRAM peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: BKPRAM peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<BKPRAMEN_A> for bool {
    #[inline(always)]
    fn from(variant: BKPRAMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPRAMEN` reader - BKPRAM clock enable Set and reset by software"]
pub type BKPRAMEN_R = crate::BitReader<BKPRAMEN_A>;
impl BKPRAMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKPRAMEN_A {
        match self.bits {
            false => BKPRAMEN_A::B_0x0,
            true => BKPRAMEN_A::B_0x1,
        }
    }
    #[doc = "BKPRAM peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKPRAMEN_A::B_0x0
    }
    #[doc = "BKPRAM peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKPRAMEN_A::B_0x1
    }
}
#[doc = "Field `BKPRAMEN` writer - BKPRAM clock enable Set and reset by software"]
pub type BKPRAMEN_W<'a, REG> = crate::BitWriter<'a, REG, BKPRAMEN_A>;
impl<'a, REG> BKPRAMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKPRAM peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKPRAMEN_A::B_0x0)
    }
    #[doc = "BKPRAM peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKPRAMEN_A::B_0x1)
    }
}
#[doc = "SRAM1 clock enable Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1EN_A {
    #[doc = "0: SRAM1 clock disabled"]
    B_0x0 = 0,
    #[doc = "1: SRAM1 clock enabled (default after reset)"]
    B_0x1 = 1,
}
impl From<SRAM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1EN` reader - SRAM1 clock enable Set and reset by software."]
pub type SRAM1EN_R = crate::BitReader<SRAM1EN_A>;
impl SRAM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1EN_A {
        match self.bits {
            false => SRAM1EN_A::B_0x0,
            true => SRAM1EN_A::B_0x1,
        }
    }
    #[doc = "SRAM1 clock disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM1EN_A::B_0x0
    }
    #[doc = "SRAM1 clock enabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM1EN_A::B_0x1
    }
}
#[doc = "Field `SRAM1EN` writer - SRAM1 clock enable Set and reset by software."]
pub type SRAM1EN_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1EN_A>;
impl<'a, REG> SRAM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM1 clock disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1EN_A::B_0x0)
    }
    #[doc = "SRAM1 clock enabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1EN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - GPDMA1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA1EN(&self) -> GPDMA1EN_R {
        GPDMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA2EN(&self) -> GPDMA2EN_R {
        GPDMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash interface clock enable Set and reset by software."]
    #[inline(always)]
    pub fn FLITFEN(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn CRCEN(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn RAMCFGEN(&self) -> RAMCFGEN_R {
        RAMCFGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 28 - BKPRAM clock enable Set and reset by software"]
    #[inline(always)]
    pub fn BKPRAMEN(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SRAM1EN(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA1EN(&mut self) -> GPDMA1EN_W<'_, AHB1ENR_SPEC> {
        GPDMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPDMA2EN(&mut self) -> GPDMA2EN_W<'_, AHB1ENR_SPEC> {
        GPDMA2EN_W::new(self, 1)
    }
    #[doc = "Bit 8 - Flash interface clock enable Set and reset by software."]
    #[inline(always)]
    pub fn FLITFEN(&mut self) -> FLITFEN_W<'_, AHB1ENR_SPEC> {
        FLITFEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn CRCEN(&mut self) -> CRCEN_W<'_, AHB1ENR_SPEC> {
        CRCEN_W::new(self, 12)
    }
    #[doc = "Bit 17 - RAMCFG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn RAMCFGEN(&mut self) -> RAMCFGEN_W<'_, AHB1ENR_SPEC> {
        RAMCFGEN_W::new(self, 17)
    }
    #[doc = "Bit 28 - BKPRAM clock enable Set and reset by software"]
    #[inline(always)]
    pub fn BKPRAMEN(&mut self) -> BKPRAMEN_W<'_, AHB1ENR_SPEC> {
        BKPRAMEN_W::new(self, 28)
    }
    #[doc = "Bit 31 - SRAM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SRAM1EN(&mut self) -> SRAM1EN_W<'_, AHB1ENR_SPEC> {
        SRAM1EN_W::new(self, 31)
    }
}
#[doc = "RCC AHB1 peripherals clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1ENR_SPEC;
impl crate::RegisterSpec for AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1enr::R`](R) reader structure"]
impl crate::Readable for AHB1ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure"]
impl crate::Writable for AHB1ENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AHB1ENR to value 0x9000_0100"]
impl crate::Resettable for AHB1ENR_SPEC {
    const RESET_VALUE: u32 = 0x9000_0100;
}
