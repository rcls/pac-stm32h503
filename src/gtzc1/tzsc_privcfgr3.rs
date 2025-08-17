#[doc = "Register `TZSC_PRIVCFGR3` reader"]
pub type R = crate::R<TZSC_PRIVCFGR3_SPEC>;
#[doc = "Register `TZSC_PRIVCFGR3` writer"]
pub type W = crate::W<TZSC_PRIVCFGR3_SPEC>;
#[doc = "privileged access mode for I3C2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C2PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<I3C2PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: I3C2PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C2PRIV` reader - privileged access mode for I3C2"]
pub type I3C2PRIV_R = crate::BitReader<I3C2PRIV_A>;
impl I3C2PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3C2PRIV_A {
        match self.bits {
            false => I3C2PRIV_A::B_0x0,
            true => I3C2PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I3C2PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I3C2PRIV_A::B_0x1
    }
}
#[doc = "Field `I3C2PRIV` writer - privileged access mode for I3C2"]
pub type I3C2PRIV_W<'a, REG> = crate::BitWriter<'a, REG, I3C2PRIV_A>;
impl<'a, REG> I3C2PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<CRCPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: CRCPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCPRIV` reader - privileged access mode for CRC"]
pub type CRCPRIV_R = crate::BitReader<CRCPRIV_A>;
impl CRCPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCPRIV_A {
        match self.bits {
            false => CRCPRIV_A::B_0x0,
            true => CRCPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRCPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRCPRIV_A::B_0x1
    }
}
#[doc = "Field `CRCPRIV` writer - privileged access mode for CRC"]
pub type CRCPRIV_W<'a, REG> = crate::BitWriter<'a, REG, CRCPRIV_A>;
impl<'a, REG> CRCPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for ICACHE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICACHEPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<ICACHEPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHEPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICACHEPRIV` reader - privileged access mode for ICACHE"]
pub type ICACHEPRIV_R = crate::BitReader<ICACHEPRIV_A>;
impl ICACHEPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICACHEPRIV_A {
        match self.bits {
            false => ICACHEPRIV_A::B_0x0,
            true => ICACHEPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ICACHEPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ICACHEPRIV_A::B_0x1
    }
}
#[doc = "Field `ICACHEPRIV` writer - privileged access mode for ICACHE"]
pub type ICACHEPRIV_W<'a, REG> = crate::BitWriter<'a, REG, ICACHEPRIV_A>;
impl<'a, REG> ICACHEPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ICACHEPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ICACHEPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for ADC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<ADC1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC1PRIV` reader - privileged access mode for ADC1"]
pub type ADC1PRIV_R = crate::BitReader<ADC1PRIV_A>;
impl ADC1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC1PRIV_A {
        match self.bits {
            false => ADC1PRIV_A::B_0x0,
            true => ADC1PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADC1PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADC1PRIV_A::B_0x1
    }
}
#[doc = "Field `ADC1PRIV` writer - privileged access mode for ADC1"]
pub type ADC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, ADC1PRIV_A>;
impl<'a, REG> ADC1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for HASH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<HASHPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: HASHPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHPRIV` reader - privileged access mode for HASH"]
pub type HASHPRIV_R = crate::BitReader<HASHPRIV_A>;
impl HASHPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HASHPRIV_A {
        match self.bits {
            false => HASHPRIV_A::B_0x0,
            true => HASHPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HASHPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HASHPRIV_A::B_0x1
    }
}
#[doc = "Field `HASHPRIV` writer - privileged access mode for HASH"]
pub type HASHPRIV_W<'a, REG> = crate::BitWriter<'a, REG, HASHPRIV_A>;
impl<'a, REG> HASHPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HASHPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HASHPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for RNG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<RNGPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: RNGPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGPRIV` reader - privileged access mode for RNG"]
pub type RNGPRIV_R = crate::BitReader<RNGPRIV_A>;
impl RNGPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGPRIV_A {
        match self.bits {
            false => RNGPRIV_A::B_0x0,
            true => RNGPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RNGPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RNGPRIV_A::B_0x1
    }
}
#[doc = "Field `RNGPRIV` writer - privileged access mode for RNG"]
pub type RNGPRIV_W<'a, REG> = crate::BitWriter<'a, REG, RNGPRIV_A>;
impl<'a, REG> RNGPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RNGPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RNGPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for RAMSCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMCFGPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<RAMCFGPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: RAMCFGPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMCFGPRIV` reader - privileged access mode for RAMSCFG"]
pub type RAMCFGPRIV_R = crate::BitReader<RAMCFGPRIV_A>;
impl RAMCFGPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMCFGPRIV_A {
        match self.bits {
            false => RAMCFGPRIV_A::B_0x0,
            true => RAMCFGPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RAMCFGPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RAMCFGPRIV_A::B_0x1
    }
}
#[doc = "Field `RAMCFGPRIV` writer - privileged access mode for RAMSCFG"]
pub type RAMCFGPRIV_W<'a, REG> = crate::BitWriter<'a, REG, RAMCFGPRIV_A>;
impl<'a, REG> RAMCFGPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGPRIV_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 2 - privileged access mode for I3C2"]
    #[inline(always)]
    pub fn I3C2PRIV(&self) -> I3C2PRIV_R {
        I3C2PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for CRC"]
    #[inline(always)]
    pub fn CRCPRIV(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for ICACHE"]
    #[inline(always)]
    pub fn ICACHEPRIV(&self) -> ICACHEPRIV_R {
        ICACHEPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for ADC1"]
    #[inline(always)]
    pub fn ADC1PRIV(&self) -> ADC1PRIV_R {
        ADC1PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - privileged access mode for HASH"]
    #[inline(always)]
    pub fn HASHPRIV(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - privileged access mode for RNG"]
    #[inline(always)]
    pub fn RNGPRIV(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 26 - privileged access mode for RAMSCFG"]
    #[inline(always)]
    pub fn RAMCFGPRIV(&self) -> RAMCFGPRIV_R {
        RAMCFGPRIV_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - privileged access mode for I3C2"]
    #[inline(always)]
    pub fn I3C2PRIV(&mut self) -> I3C2PRIV_W<'_, TZSC_PRIVCFGR3_SPEC> {
        I3C2PRIV_W::new(self, 2)
    }
    #[doc = "Bit 8 - privileged access mode for CRC"]
    #[inline(always)]
    pub fn CRCPRIV(&mut self) -> CRCPRIV_W<'_, TZSC_PRIVCFGR3_SPEC> {
        CRCPRIV_W::new(self, 8)
    }
    #[doc = "Bit 12 - privileged access mode for ICACHE"]
    #[inline(always)]
    pub fn ICACHEPRIV(&mut self) -> ICACHEPRIV_W<'_, TZSC_PRIVCFGR3_SPEC> {
        ICACHEPRIV_W::new(self, 12)
    }
    #[doc = "Bit 14 - privileged access mode for ADC1"]
    #[inline(always)]
    pub fn ADC1PRIV(&mut self) -> ADC1PRIV_W<'_, TZSC_PRIVCFGR3_SPEC> {
        ADC1PRIV_W::new(self, 14)
    }
    #[doc = "Bit 17 - privileged access mode for HASH"]
    #[inline(always)]
    pub fn HASHPRIV(&mut self) -> HASHPRIV_W<'_, TZSC_PRIVCFGR3_SPEC> {
        HASHPRIV_W::new(self, 17)
    }
    #[doc = "Bit 18 - privileged access mode for RNG"]
    #[inline(always)]
    pub fn RNGPRIV(&mut self) -> RNGPRIV_W<'_, TZSC_PRIVCFGR3_SPEC> {
        RNGPRIV_W::new(self, 18)
    }
    #[doc = "Bit 26 - privileged access mode for RAMSCFG"]
    #[inline(always)]
    pub fn RAMCFGPRIV(&mut self) -> RAMCFGPRIV_W<'_, TZSC_PRIVCFGR3_SPEC> {
        RAMCFGPRIV_W::new(self, 26)
    }
}
#[doc = "GTZC1 TZSC privilege configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_PRIVCFGR3_SPEC;
impl crate::RegisterSpec for TZSC_PRIVCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_privcfgr3::R`](R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzsc_privcfgr3::W`](W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR3_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TZSC_PRIVCFGR3 to value 0"]
impl crate::Resettable for TZSC_PRIVCFGR3_SPEC {}
