#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACR_SPEC>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACR_SPEC>;
#[doc = "Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY_A {
    #[doc = "0: zero wait state used to read a word from non-volatile memory"]
    B_0x0 = 0,
    #[doc = "1: one wait state used to read a word from non-volatile memory"]
    B_0x1 = 1,
    #[doc = "2: two wait states used to read a word from non-volatile memory"]
    B_0x2 = 2,
    #[doc = "7: seven wait states used to read a word from non-volatile memory"]
    B_0x7 = 7,
    #[doc = "15: 15 wait states used to read from non-volatile memory"]
    B_0xF = 15,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LATENCY_A {
    type Ux = u8;
}
impl crate::IsEnum for LATENCY_A {}
#[doc = "Field `LATENCY` reader - Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
pub type LATENCY_R = crate::FieldReader<LATENCY_A>;
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LATENCY_A> {
        match self.bits {
            0 => Some(LATENCY_A::B_0x0),
            1 => Some(LATENCY_A::B_0x1),
            2 => Some(LATENCY_A::B_0x2),
            7 => Some(LATENCY_A::B_0x7),
            15 => Some(LATENCY_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "zero wait state used to read a word from non-volatile memory"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LATENCY_A::B_0x0
    }
    #[doc = "one wait state used to read a word from non-volatile memory"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LATENCY_A::B_0x1
    }
    #[doc = "two wait states used to read a word from non-volatile memory"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == LATENCY_A::B_0x2
    }
    #[doc = "seven wait states used to read a word from non-volatile memory"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == LATENCY_A::B_0x7
    }
    #[doc = "15 wait states used to read from non-volatile memory"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == LATENCY_A::B_0xF
    }
}
#[doc = "Field `LATENCY` writer - Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, LATENCY_A>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "zero wait state used to read a word from non-volatile memory"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY_A::B_0x0)
    }
    #[doc = "one wait state used to read a word from non-volatile memory"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY_A::B_0x1)
    }
    #[doc = "two wait states used to read a word from non-volatile memory"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY_A::B_0x2)
    }
    #[doc = "seven wait states used to read a word from non-volatile memory"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY_A::B_0x7)
    }
    #[doc = "15 wait states used to read from non-volatile memory"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY_A::B_0xF)
    }
}
#[doc = "Field `WRHIGHFREQ` reader - Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to for details. Note: No check is performed to verify that the configuration is correct. Two WRHIGHFREQ values can be selected for some frequencies."]
pub type WRHIGHFREQ_R = crate::FieldReader;
#[doc = "Field `WRHIGHFREQ` writer - Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to for details. Note: No check is performed to verify that the configuration is correct. Two WRHIGHFREQ values can be selected for some frequencies."]
pub type WRHIGHFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN_A {
    #[doc = "0: prefetch disabled."]
    B_0x0 = 0,
    #[doc = "1: prefetch enabled when latency is at least one wait state."]
    B_0x1 = 1,
}
impl From<PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
pub type PRFTEN_R = crate::BitReader<PRFTEN_A>;
impl PRFTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN_A {
        match self.bits {
            false => PRFTEN_A::B_0x0,
            true => PRFTEN_A::B_0x1,
        }
    }
    #[doc = "prefetch disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRFTEN_A::B_0x0
    }
    #[doc = "prefetch enabled when latency is at least one wait state."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRFTEN_A::B_0x1
    }
}
#[doc = "Field `PRFTEN` writer - Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN_A>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "prefetch disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN_A::B_0x0)
    }
    #[doc = "prefetch enabled when latency is at least one wait state."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN_A::B_0x1)
    }
}
#[doc = "Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S_PRFTEN_A {
    #[doc = "0: prefetch, if enabled fetches each instruction."]
    B_0x0 = 0,
    #[doc = "1: prefetch, if enabled avoids fetch past branch to improve efficiency."]
    B_0x1 = 1,
}
impl From<S_PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: S_PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_PRFTEN` reader - Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality."]
pub type S_PRFTEN_R = crate::BitReader<S_PRFTEN_A>;
impl S_PRFTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S_PRFTEN_A {
        match self.bits {
            false => S_PRFTEN_A::B_0x0,
            true => S_PRFTEN_A::B_0x1,
        }
    }
    #[doc = "prefetch, if enabled fetches each instruction."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == S_PRFTEN_A::B_0x0
    }
    #[doc = "prefetch, if enabled avoids fetch past branch to improve efficiency."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == S_PRFTEN_A::B_0x1
    }
}
#[doc = "Field `S_PRFTEN` writer - Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality."]
pub type S_PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, S_PRFTEN_A>;
impl<'a, REG> S_PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "prefetch, if enabled fetches each instruction."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(S_PRFTEN_A::B_0x0)
    }
    #[doc = "prefetch, if enabled avoids fetch past branch to improve efficiency."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(S_PRFTEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
    #[inline(always)]
    pub fn LATENCY(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to for details. Note: No check is performed to verify that the configuration is correct. Two WRHIGHFREQ values can be selected for some frequencies."]
    #[inline(always)]
    pub fn WRHIGHFREQ(&self) -> WRHIGHFREQ_R {
        WRHIGHFREQ_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
    #[inline(always)]
    pub fn PRFTEN(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality."]
    #[inline(always)]
    pub fn S_PRFTEN(&self) -> S_PRFTEN_R {
        S_PRFTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
    #[inline(always)]
    pub fn LATENCY(&mut self) -> LATENCY_W<'_, ACR_SPEC> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to for details. Note: No check is performed to verify that the configuration is correct. Two WRHIGHFREQ values can be selected for some frequencies."]
    #[inline(always)]
    pub fn WRHIGHFREQ(&mut self) -> WRHIGHFREQ_W<'_, ACR_SPEC> {
        WRHIGHFREQ_W::new(self, 4)
    }
    #[doc = "Bit 8 - Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
    #[inline(always)]
    pub fn PRFTEN(&mut self) -> PRFTEN_W<'_, ACR_SPEC> {
        PRFTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality."]
    #[inline(always)]
    pub fn S_PRFTEN(&mut self) -> S_PRFTEN_W<'_, ACR_SPEC> {
        S_PRFTEN_W::new(self, 9)
    }
}
#[doc = "FLASH access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ACR to value 0x13"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: u32 = 0x13;
}
