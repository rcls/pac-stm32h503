#[doc = "Register `PMCR` reader"]
pub type R = crate::R<PMCR_SPEC>;
#[doc = "Register `PMCR` writer"]
pub type W = crate::W<PMCR_SPEC>;
#[doc = "booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTEN_A {
    #[doc = "0: Booster disabled"]
    B_0x0 = 0,
    #[doc = "1: Booster enabled"]
    B_0x1 = 1,
}
impl From<BOOSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: BOOSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOSTEN` reader - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
pub type BOOSTEN_R = crate::BitReader<BOOSTEN_A>;
impl BOOSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOOSTEN_A {
        match self.bits {
            false => BOOSTEN_A::B_0x0,
            true => BOOSTEN_A::B_0x1,
        }
    }
    #[doc = "Booster disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BOOSTEN_A::B_0x0
    }
    #[doc = "Booster enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BOOSTEN_A::B_0x1
    }
}
#[doc = "Field `BOOSTEN` writer - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG, BOOSTEN_A>;
impl<'a, REG> BOOSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Booster disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN_A::B_0x0)
    }
    #[doc = "Booster enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN_A::B_0x1)
    }
}
#[doc = "booster V DD selection Note: Booster must not be used when V DDA 2.7 V, but V DD 2.7 V (add current consumption). Note: When both V DD 2.7 V and V DDA 2.7 V, booster is needed to get full AC performances from I/O analog switches.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTVDDSEL_A {
    #[doc = "0: V DDA is selected as V BOOSTER supply (BOOSTE = 0)"]
    B_0x0 = 0,
    #[doc = "1: V DD is selected as V BOOSTER supply (regardless BOOSTEN that must be cleared to avoid unwanted power consumption)"]
    B_0x1 = 1,
}
impl From<BOOSTVDDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: BOOSTVDDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOSTVDDSEL` reader - booster V DD selection Note: Booster must not be used when V DDA 2.7 V, but V DD 2.7 V (add current consumption). Note: When both V DD 2.7 V and V DDA 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
pub type BOOSTVDDSEL_R = crate::BitReader<BOOSTVDDSEL_A>;
impl BOOSTVDDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOOSTVDDSEL_A {
        match self.bits {
            false => BOOSTVDDSEL_A::B_0x0,
            true => BOOSTVDDSEL_A::B_0x1,
        }
    }
    #[doc = "V DDA is selected as V BOOSTER supply (BOOSTE = 0)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BOOSTVDDSEL_A::B_0x0
    }
    #[doc = "V DD is selected as V BOOSTER supply (regardless BOOSTEN that must be cleared to avoid unwanted power consumption)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BOOSTVDDSEL_A::B_0x1
    }
}
#[doc = "Field `BOOSTVDDSEL` writer - booster V DD selection Note: Booster must not be used when V DDA 2.7 V, but V DD 2.7 V (add current consumption). Note: When both V DD 2.7 V and V DDA 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
pub type BOOSTVDDSEL_W<'a, REG> = crate::BitWriter<'a, REG, BOOSTVDDSEL_A>;
impl<'a, REG> BOOSTVDDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V DDA is selected as V BOOSTER supply (BOOSTE = 0)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTVDDSEL_A::B_0x0)
    }
    #[doc = "V DD is selected as V BOOSTER supply (regardless BOOSTEN that must be cleared to avoid unwanted power consumption)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTVDDSEL_A::B_0x1)
    }
}
#[doc = "Fast-mode Plus command on PB(6)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB6_FMPLUS_A {
    #[doc = "0: Fast-mode Plus mode on PB(6) is disabled"]
    B_0x0 = 0,
    #[doc = "1: Fast-mode Plus mode on PB(6) is enabled"]
    B_0x1 = 1,
}
impl From<PB6_FMPLUS_A> for bool {
    #[inline(always)]
    fn from(variant: PB6_FMPLUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PB6_FMPLUS` reader - Fast-mode Plus command on PB(6)"]
pub type PB6_FMPLUS_R = crate::BitReader<PB6_FMPLUS_A>;
impl PB6_FMPLUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PB6_FMPLUS_A {
        match self.bits {
            false => PB6_FMPLUS_A::B_0x0,
            true => PB6_FMPLUS_A::B_0x1,
        }
    }
    #[doc = "Fast-mode Plus mode on PB(6) is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PB6_FMPLUS_A::B_0x0
    }
    #[doc = "Fast-mode Plus mode on PB(6) is enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PB6_FMPLUS_A::B_0x1
    }
}
#[doc = "Field `PB6_FMPLUS` writer - Fast-mode Plus command on PB(6)"]
pub type PB6_FMPLUS_W<'a, REG> = crate::BitWriter<'a, REG, PB6_FMPLUS_A>;
impl<'a, REG> PB6_FMPLUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast-mode Plus mode on PB(6) is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_FMPLUS_A::B_0x0)
    }
    #[doc = "Fast-mode Plus mode on PB(6) is enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PB6_FMPLUS_A::B_0x1)
    }
}
#[doc = "Fast-mode Plus command on PB(7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB7_FMPLUS_A {
    #[doc = "0: Fast-mode Plus mode on PB(7) is disabled"]
    B_0x0 = 0,
    #[doc = "1: Fast mode plus mode on PB(7) is enabled"]
    B_0x1 = 1,
}
impl From<PB7_FMPLUS_A> for bool {
    #[inline(always)]
    fn from(variant: PB7_FMPLUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PB7_FMPLUS` reader - Fast-mode Plus command on PB(7)"]
pub type PB7_FMPLUS_R = crate::BitReader<PB7_FMPLUS_A>;
impl PB7_FMPLUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PB7_FMPLUS_A {
        match self.bits {
            false => PB7_FMPLUS_A::B_0x0,
            true => PB7_FMPLUS_A::B_0x1,
        }
    }
    #[doc = "Fast-mode Plus mode on PB(7) is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PB7_FMPLUS_A::B_0x0
    }
    #[doc = "Fast mode plus mode on PB(7) is enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PB7_FMPLUS_A::B_0x1
    }
}
#[doc = "Field `PB7_FMPLUS` writer - Fast-mode Plus command on PB(7)"]
pub type PB7_FMPLUS_W<'a, REG> = crate::BitWriter<'a, REG, PB7_FMPLUS_A>;
impl<'a, REG> PB7_FMPLUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast-mode Plus mode on PB(7) is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_FMPLUS_A::B_0x0)
    }
    #[doc = "Fast mode plus mode on PB(7) is enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PB7_FMPLUS_A::B_0x1)
    }
}
#[doc = "Fast-mode Plus command on PB(8)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PB8_FMPLUS_A {
    #[doc = "0: Fast-mode Plus mode on PB(8) is disabled"]
    B_0x0 = 0,
    #[doc = "1: Fast-mode Plus mode on PB(8) is enabled"]
    B_0x1 = 1,
}
impl From<PB8_FMPLUS_A> for bool {
    #[inline(always)]
    fn from(variant: PB8_FMPLUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PB8_FMPLUS` reader - Fast-mode Plus command on PB(8)"]
pub type PB8_FMPLUS_R = crate::BitReader<PB8_FMPLUS_A>;
impl PB8_FMPLUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PB8_FMPLUS_A {
        match self.bits {
            false => PB8_FMPLUS_A::B_0x0,
            true => PB8_FMPLUS_A::B_0x1,
        }
    }
    #[doc = "Fast-mode Plus mode on PB(8) is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PB8_FMPLUS_A::B_0x0
    }
    #[doc = "Fast-mode Plus mode on PB(8) is enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PB8_FMPLUS_A::B_0x1
    }
}
#[doc = "Field `PB8_FMPLUS` writer - Fast-mode Plus command on PB(8)"]
pub type PB8_FMPLUS_W<'a, REG> = crate::BitWriter<'a, REG, PB8_FMPLUS_A>;
impl<'a, REG> PB8_FMPLUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast-mode Plus mode on PB(8) is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_FMPLUS_A::B_0x0)
    }
    #[doc = "Fast-mode Plus mode on PB(8) is enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PB8_FMPLUS_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
    #[inline(always)]
    pub fn BOOSTEN(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - booster V DD selection Note: Booster must not be used when V DDA 2.7 V, but V DD 2.7 V (add current consumption). Note: When both V DD 2.7 V and V DDA 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
    #[inline(always)]
    pub fn BOOSTVDDSEL(&self) -> BOOSTVDDSEL_R {
        BOOSTVDDSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast-mode Plus command on PB(6)"]
    #[inline(always)]
    pub fn PB6_FMPLUS(&self) -> PB6_FMPLUS_R {
        PB6_FMPLUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast-mode Plus command on PB(7)"]
    #[inline(always)]
    pub fn PB7_FMPLUS(&self) -> PB7_FMPLUS_R {
        PB7_FMPLUS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast-mode Plus command on PB(8)"]
    #[inline(always)]
    pub fn PB8_FMPLUS(&self) -> PB8_FMPLUS_R {
        PB8_FMPLUS_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
    #[inline(always)]
    pub fn BOOSTEN(&mut self) -> BOOSTEN_W<'_, PMCR_SPEC> {
        BOOSTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - booster V DD selection Note: Booster must not be used when V DDA 2.7 V, but V DD 2.7 V (add current consumption). Note: When both V DD 2.7 V and V DDA 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
    #[inline(always)]
    pub fn BOOSTVDDSEL(&mut self) -> BOOSTVDDSEL_W<'_, PMCR_SPEC> {
        BOOSTVDDSEL_W::new(self, 9)
    }
    #[doc = "Bit 16 - Fast-mode Plus command on PB(6)"]
    #[inline(always)]
    pub fn PB6_FMPLUS(&mut self) -> PB6_FMPLUS_W<'_, PMCR_SPEC> {
        PB6_FMPLUS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Fast-mode Plus command on PB(7)"]
    #[inline(always)]
    pub fn PB7_FMPLUS(&mut self) -> PB7_FMPLUS_W<'_, PMCR_SPEC> {
        PB7_FMPLUS_W::new(self, 17)
    }
    #[doc = "Bit 18 - Fast-mode Plus command on PB(8)"]
    #[inline(always)]
    pub fn PB8_FMPLUS(&mut self) -> PB8_FMPLUS_W<'_, PMCR_SPEC> {
        PB8_FMPLUS_W::new(self, 18)
    }
}
#[doc = "SBS product mode and configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMCR_SPEC;
impl crate::RegisterSpec for PMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmcr::R`](R) reader structure"]
impl crate::Readable for PMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmcr::W`](W) writer structure"]
impl crate::Writable for PMCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PMCR to value 0"]
impl crate::Resettable for PMCR_SPEC {}
