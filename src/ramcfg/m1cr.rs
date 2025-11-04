#[doc = "Register `M1CR` reader"]
pub type R = crate::R<M1CR_SPEC>;
#[doc = "Register `M1CR` writer"]
pub type W = crate::W<M1CR_SPEC>;
#[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCE_A {
    #[doc = "0: ECC disabled"]
    B_0x0 = 0,
    #[doc = "1: ECC enabled"]
    B_0x1 = 1,
}
impl From<ECCE_A> for bool {
    #[inline(always)]
    fn from(variant: ECCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCE` reader - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
pub type ECCE_R = crate::BitReader<ECCE_A>;
impl ECCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCE_A {
        match self.bits {
            false => ECCE_A::B_0x0,
            true => ECCE_A::B_0x1,
        }
    }
    #[doc = "ECC disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ECCE_A::B_0x0
    }
    #[doc = "ECC enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ECCE_A::B_0x1
    }
}
#[doc = "Field `ECCE` writer - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
pub type ECCE_W<'a, REG> = crate::BitWriter<'a, REG, ECCE_A>;
impl<'a, REG> ECCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCE_A::B_0x0)
    }
    #[doc = "ECC enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCE_A::B_0x1)
    }
}
#[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALE_A {
    #[doc = "0: Failing address not stored in the SRAMx ECC single/double error address registers"]
    B_0x0 = 0,
    #[doc = "1: Failing address stored in the SRAMx ECC single/double error address registers"]
    B_0x1 = 1,
}
impl From<ALE_A> for bool {
    #[inline(always)]
    fn from(variant: ALE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALE` reader - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
pub type ALE_R = crate::BitReader<ALE_A>;
impl ALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALE_A {
        match self.bits {
            false => ALE_A::B_0x0,
            true => ALE_A::B_0x1,
        }
    }
    #[doc = "Failing address not stored in the SRAMx ECC single/double error address registers"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ALE_A::B_0x0
    }
    #[doc = "Failing address stored in the SRAMx ECC single/double error address registers"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ALE_A::B_0x1
    }
}
#[doc = "Field `ALE` writer - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
pub type ALE_W<'a, REG> = crate::BitWriter<'a, REG, ALE_A>;
impl<'a, REG> ALE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Failing address not stored in the SRAMx ECC single/double error address registers"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALE_A::B_0x0)
    }
    #[doc = "Failing address stored in the SRAMx ECC single/double error address registers"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALE_A::B_0x1)
    }
}
#[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMER_A {
    #[doc = "0: No erase operation on going"]
    B_0x0 = 0,
    #[doc = "1: Erase operation on going"]
    B_0x1 = 1,
}
impl From<SRAMER_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMER` reader - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
pub type SRAMER_R = crate::BitReader<SRAMER_A>;
impl SRAMER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAMER_A {
        match self.bits {
            false => SRAMER_A::B_0x0,
            true => SRAMER_A::B_0x1,
        }
    }
    #[doc = "No erase operation on going"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAMER_A::B_0x0
    }
    #[doc = "Erase operation on going"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAMER_A::B_0x1
    }
}
#[doc = "Field `SRAMER` writer - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG, SRAMER_A>;
impl<'a, REG> SRAMER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No erase operation on going"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMER_A::B_0x0)
    }
    #[doc = "Erase operation on going"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMER_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
    #[inline(always)]
    pub fn ECCE(&self) -> ECCE_R {
        ECCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
    #[inline(always)]
    pub fn ALE(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
    #[inline(always)]
    pub fn SRAMER(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
    #[inline(always)]
    pub fn ECCE(&mut self) -> ECCE_W<'_, M1CR_SPEC> {
        ECCE_W::new(self, 0)
    }
    #[doc = "Bit 4 - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
    #[inline(always)]
    pub fn ALE(&mut self) -> ALE_W<'_, M1CR_SPEC> {
        ALE_W::new(self, 4)
    }
    #[doc = "Bit 8 - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
    #[inline(always)]
    pub fn SRAMER(&mut self) -> SRAMER_W<'_, M1CR_SPEC> {
        SRAMER_W::new(self, 8)
    }
}
#[doc = "RAMCFG memory 1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1CR_SPEC;
impl crate::RegisterSpec for M1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1cr::R`](R) reader structure"]
impl crate::Readable for M1CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m1cr::W`](W) writer structure"]
impl crate::Writable for M1CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets M1CR to value 0"]
impl crate::Resettable for M1CR_SPEC {}
