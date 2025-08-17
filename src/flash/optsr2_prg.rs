#[doc = "Register `OPTSR2_PRG` reader"]
pub type R = crate::R<OPTSR2_PRG_SPEC>;
#[doc = "Register `OPTSR2_PRG` writer"]
pub type W = crate::W<OPTSR2_PRG_SPEC>;
#[doc = "SRAM2 erase when system reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_RST_A {
    #[doc = "0: SRAM2 erased when a system reset occurs"]
    B_0x0 = 0,
    #[doc = "1: SRAM2 not erased when a system reset occurs."]
    B_0x1 = 1,
}
impl From<SRAM2_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2_RST` reader - SRAM2 erase when system reset"]
pub type SRAM2_RST_R = crate::BitReader<SRAM2_RST_A>;
impl SRAM2_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2_RST_A {
        match self.bits {
            false => SRAM2_RST_A::B_0x0,
            true => SRAM2_RST_A::B_0x1,
        }
    }
    #[doc = "SRAM2 erased when a system reset occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM2_RST_A::B_0x0
    }
    #[doc = "SRAM2 not erased when a system reset occurs."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM2_RST_A::B_0x1
    }
}
#[doc = "Field `SRAM2_RST` writer - SRAM2 erase when system reset"]
pub type SRAM2_RST_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2_RST_A>;
impl<'a, REG> SRAM2_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 erased when a system reset occurs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_RST_A::B_0x0)
    }
    #[doc = "SRAM2 not erased when a system reset occurs."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_RST_A::B_0x1)
    }
}
#[doc = "Backup RAM ECC detection and correction disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKPRAM_ECC_A {
    #[doc = "0: BKPRAM ECC check enabled"]
    B_0x0 = 0,
    #[doc = "1: BKPRAM ECC check disabled"]
    B_0x1 = 1,
}
impl From<BKPRAM_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: BKPRAM_ECC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction disable"]
pub type BKPRAM_ECC_R = crate::BitReader<BKPRAM_ECC_A>;
impl BKPRAM_ECC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKPRAM_ECC_A {
        match self.bits {
            false => BKPRAM_ECC_A::B_0x0,
            true => BKPRAM_ECC_A::B_0x1,
        }
    }
    #[doc = "BKPRAM ECC check enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKPRAM_ECC_A::B_0x0
    }
    #[doc = "BKPRAM ECC check disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKPRAM_ECC_A::B_0x1
    }
}
#[doc = "Field `BKPRAM_ECC` writer - Backup RAM ECC detection and correction disable"]
pub type BKPRAM_ECC_W<'a, REG> = crate::BitWriter<'a, REG, BKPRAM_ECC_A>;
impl<'a, REG> BKPRAM_ECC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKPRAM ECC check enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKPRAM_ECC_A::B_0x0)
    }
    #[doc = "BKPRAM ECC check disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKPRAM_ECC_A::B_0x1)
    }
}
#[doc = "SRAM2 ECC detection and correction disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_ECC_A {
    #[doc = "0: SRAM2 ECC check enabled"]
    B_0x0 = 0,
    #[doc = "1: SRAM2 ECC check disabled"]
    B_0x1 = 1,
}
impl From<SRAM2_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_ECC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction disable"]
pub type SRAM2_ECC_R = crate::BitReader<SRAM2_ECC_A>;
impl SRAM2_ECC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2_ECC_A {
        match self.bits {
            false => SRAM2_ECC_A::B_0x0,
            true => SRAM2_ECC_A::B_0x1,
        }
    }
    #[doc = "SRAM2 ECC check enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM2_ECC_A::B_0x0
    }
    #[doc = "SRAM2 ECC check disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM2_ECC_A::B_0x1
    }
}
#[doc = "Field `SRAM2_ECC` writer - SRAM2 ECC detection and correction disable"]
pub type SRAM2_ECC_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2_ECC_A>;
impl<'a, REG> SRAM2_ECC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 ECC check enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_ECC_A::B_0x0)
    }
    #[doc = "SRAM2 ECC check disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_ECC_A::B_0x1)
    }
}
#[doc = "SRAM1 erase upon system reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1_RST_A {
    #[doc = "0: SRAM1 erased when a system reset occurs"]
    B_0x0 = 0,
    #[doc = "1: SRAM1 not erased when a system reset occurs"]
    B_0x1 = 1,
}
impl From<SRAM1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1_RST` reader - SRAM1 erase upon system reset"]
pub type SRAM1_RST_R = crate::BitReader<SRAM1_RST_A>;
impl SRAM1_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1_RST_A {
        match self.bits {
            false => SRAM1_RST_A::B_0x0,
            true => SRAM1_RST_A::B_0x1,
        }
    }
    #[doc = "SRAM1 erased when a system reset occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM1_RST_A::B_0x0
    }
    #[doc = "SRAM1 not erased when a system reset occurs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM1_RST_A::B_0x1
    }
}
#[doc = "Field `SRAM1_RST` writer - SRAM1 erase upon system reset"]
pub type SRAM1_RST_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1_RST_A>;
impl<'a, REG> SRAM1_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM1 erased when a system reset occurs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1_RST_A::B_0x0)
    }
    #[doc = "SRAM1 not erased when a system reset occurs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1_RST_A::B_0x1)
    }
}
#[doc = "SRAM1 ECC detection and correction disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1_ECC_A {
    #[doc = "0: SRAM1 ECC check enabled"]
    B_0x0 = 0,
    #[doc = "1: SRAM1 ECC check disabled"]
    B_0x1 = 1,
}
impl From<SRAM1_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1_ECC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1_ECC` reader - SRAM1 ECC detection and correction disable"]
pub type SRAM1_ECC_R = crate::BitReader<SRAM1_ECC_A>;
impl SRAM1_ECC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1_ECC_A {
        match self.bits {
            false => SRAM1_ECC_A::B_0x0,
            true => SRAM1_ECC_A::B_0x1,
        }
    }
    #[doc = "SRAM1 ECC check enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM1_ECC_A::B_0x0
    }
    #[doc = "SRAM1 ECC check disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM1_ECC_A::B_0x1
    }
}
#[doc = "Field `SRAM1_ECC` writer - SRAM1 ECC detection and correction disable"]
pub type SRAM1_ECC_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1_ECC_A>;
impl<'a, REG> SRAM1_ECC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM1 ECC check enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1_ECC_A::B_0x0)
    }
    #[doc = "SRAM1 ECC check disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1_ECC_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 3 - SRAM2 erase when system reset"]
    #[inline(always)]
    pub fn SRAM2_RST(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Backup RAM ECC detection and correction disable"]
    #[inline(always)]
    pub fn BKPRAM_ECC(&self) -> BKPRAM_ECC_R {
        BKPRAM_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM2 ECC detection and correction disable"]
    #[inline(always)]
    pub fn SRAM2_ECC(&self) -> SRAM2_ECC_R {
        SRAM2_ECC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 erase upon system reset"]
    #[inline(always)]
    pub fn SRAM1_RST(&self) -> SRAM1_RST_R {
        SRAM1_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM1 ECC detection and correction disable"]
    #[inline(always)]
    pub fn SRAM1_ECC(&self) -> SRAM1_ECC_R {
        SRAM1_ECC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - SRAM2 erase when system reset"]
    #[inline(always)]
    pub fn SRAM2_RST(&mut self) -> SRAM2_RST_W<'_, OPTSR2_PRG_SPEC> {
        SRAM2_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Backup RAM ECC detection and correction disable"]
    #[inline(always)]
    pub fn BKPRAM_ECC(&mut self) -> BKPRAM_ECC_W<'_, OPTSR2_PRG_SPEC> {
        BKPRAM_ECC_W::new(self, 4)
    }
    #[doc = "Bit 6 - SRAM2 ECC detection and correction disable"]
    #[inline(always)]
    pub fn SRAM2_ECC(&mut self) -> SRAM2_ECC_W<'_, OPTSR2_PRG_SPEC> {
        SRAM2_ECC_W::new(self, 6)
    }
    #[doc = "Bit 9 - SRAM1 erase upon system reset"]
    #[inline(always)]
    pub fn SRAM1_RST(&mut self) -> SRAM1_RST_W<'_, OPTSR2_PRG_SPEC> {
        SRAM1_RST_W::new(self, 9)
    }
    #[doc = "Bit 10 - SRAM1 ECC detection and correction disable"]
    #[inline(always)]
    pub fn SRAM1_ECC(&mut self) -> SRAM1_ECC_W<'_, OPTSR2_PRG_SPEC> {
        SRAM1_ECC_W::new(self, 10)
    }
}
#[doc = "FLASH option status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr2_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr2_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTSR2_PRG_SPEC;
impl crate::RegisterSpec for OPTSR2_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optsr2_prg::R`](R) reader structure"]
impl crate::Readable for OPTSR2_PRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`optsr2_prg::W`](W) writer structure"]
impl crate::Writable for OPTSR2_PRG_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OPTSR2_PRG to value 0"]
impl crate::Resettable for OPTSR2_PRG_SPEC {}
