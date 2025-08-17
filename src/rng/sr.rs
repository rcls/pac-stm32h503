#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN = 0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRDY_A {
    #[doc = "0: The RNG_DR register is not yet valid, no random data is available."]
    B_0x0 = 0,
    #[doc = "1: The RNG_DR register contains valid random data."]
    B_0x1 = 1,
}
impl From<DRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRDY` reader - Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN = 0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY = 1."]
pub type DRDY_R = crate::BitReader<DRDY_A>;
impl DRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRDY_A {
        match self.bits {
            false => DRDY_A::B_0x0,
            true => DRDY_A::B_0x1,
        }
    }
    #[doc = "The RNG_DR register is not yet valid, no random data is available."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DRDY_A::B_0x0
    }
    #[doc = "The RNG_DR register contains valid random data."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DRDY_A::B_0x1
    }
}
#[doc = "Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CECS_A {
    #[doc = "0: The RNG clock is correct (fRNGCLK fHCLK/32). If the CEIS bit is set, this means that a slow clock was detected and the situation has been recovered."]
    B_0x0 = 0,
    #[doc = "1: The RNG clock is too slow (fRNGCLK fHCLK/32)."]
    B_0x1 = 1,
}
impl From<CECS_A> for bool {
    #[inline(always)]
    fn from(variant: CECS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CECS` reader - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0."]
pub type CECS_R = crate::BitReader<CECS_A>;
impl CECS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CECS_A {
        match self.bits {
            false => CECS_A::B_0x0,
            true => CECS_A::B_0x1,
        }
    }
    #[doc = "The RNG clock is correct (fRNGCLK fHCLK/32). If the CEIS bit is set, this means that a slow clock was detected and the situation has been recovered."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CECS_A::B_0x0
    }
    #[doc = "The RNG clock is too slow (fRNGCLK fHCLK/32)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CECS_A::B_0x1
    }
}
#[doc = "Seed error current status Run-time repetition count test failed (noise source has provided more than 24 consecutive bits at a constant value 0 or 1, or more than 32 consecutive occurrence of two bits patterns 01 or 10) Start-up or continuous adaptive proportion test on noise source failed. Start-up post-processing/conditioning sanity check failed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECS_A {
    #[doc = "0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered."]
    B_0x0 = 0,
    #[doc = "1: At least one of the following faulty sequence has been detected:"]
    B_0x1 = 1,
}
impl From<SECS_A> for bool {
    #[inline(always)]
    fn from(variant: SECS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECS` reader - Seed error current status Run-time repetition count test failed (noise source has provided more than 24 consecutive bits at a constant value 0 or 1, or more than 32 consecutive occurrence of two bits patterns 01 or 10) Start-up or continuous adaptive proportion test on noise source failed. Start-up post-processing/conditioning sanity check failed."]
pub type SECS_R = crate::BitReader<SECS_A>;
impl SECS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECS_A {
        match self.bits {
            false => SECS_A::B_0x0,
            true => SECS_A::B_0x1,
        }
    }
    #[doc = "No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SECS_A::B_0x0
    }
    #[doc = "At least one of the following faulty sequence has been detected:"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SECS_A::B_0x1
    }
}
#[doc = "Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEIS_A {
    #[doc = "0: The RNG clock is correct (fRNGCLK fHCLK/32)"]
    B_0x0 = 0,
    #[doc = "1: The RNG clock before internal divider is detected too slow (fRNGCLK fHCLK/32)"]
    B_0x1 = 1,
}
impl From<CEIS_A> for bool {
    #[inline(always)]
    fn from(variant: CEIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIS` reader - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type CEIS_R = crate::BitReader<CEIS_A>;
impl CEIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEIS_A {
        match self.bits {
            false => CEIS_A::B_0x0,
            true => CEIS_A::B_0x1,
        }
    }
    #[doc = "The RNG clock is correct (fRNGCLK fHCLK/32)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CEIS_A::B_0x0
    }
    #[doc = "The RNG clock before internal divider is detected too slow (fRNGCLK fHCLK/32)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CEIS_A::B_0x1
    }
}
#[doc = "Field `CEIS` writer - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type CEIS_W<'a, REG> = crate::BitWriter<'a, REG, CEIS_A>;
impl<'a, REG> CEIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RNG clock is correct (fRNGCLK fHCLK/32)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CEIS_A::B_0x0)
    }
    #[doc = "The RNG clock before internal divider is detected too slow (fRNGCLK fHCLK/32)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CEIS_A::B_0x1)
    }
}
#[doc = "Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEIS_A {
    #[doc = "0: No faulty sequence detected"]
    B_0x0 = 0,
    #[doc = "1: At least one faulty sequence is detected. See SECS bit description for details."]
    B_0x1 = 1,
}
impl From<SEIS_A> for bool {
    #[inline(always)]
    fn from(variant: SEIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEIS` reader - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type SEIS_R = crate::BitReader<SEIS_A>;
impl SEIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEIS_A {
        match self.bits {
            false => SEIS_A::B_0x0,
            true => SEIS_A::B_0x1,
        }
    }
    #[doc = "No faulty sequence detected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SEIS_A::B_0x0
    }
    #[doc = "At least one faulty sequence is detected. See SECS bit description for details."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SEIS_A::B_0x1
    }
}
#[doc = "Field `SEIS` writer - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type SEIS_W<'a, REG> = crate::BitWriter<'a, REG, SEIS_A>;
impl<'a, REG> SEIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No faulty sequence detected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SEIS_A::B_0x0)
    }
    #[doc = "At least one faulty sequence is detected. See SECS bit description for details."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SEIS_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN = 0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY = 1."]
    #[inline(always)]
    pub fn DRDY(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0."]
    #[inline(always)]
    pub fn CECS(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed error current status Run-time repetition count test failed (noise source has provided more than 24 consecutive bits at a constant value 0 or 1, or more than 32 consecutive occurrence of two bits patterns 01 or 10) Start-up or continuous adaptive proportion test on noise source failed. Start-up post-processing/conditioning sanity check failed."]
    #[inline(always)]
    pub fn SECS(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn CEIS(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn SEIS(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn CEIS(&mut self) -> CEIS_W<'_, SR_SPEC> {
        CEIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn SEIS(&mut self) -> SEIS_W<'_, SR_SPEC> {
        SEIS_W::new(self, 6)
    }
}
#[doc = "RNG status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {}
