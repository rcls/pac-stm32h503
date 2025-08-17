#[doc = "Register `TSCC` reader"]
pub type R = crate::R<TSCC_SPEC>;
#[doc = "Register `TSCC` writer"]
pub type W = crate::W<TSCC_SPEC>;
#[doc = "Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSS_A {
    #[doc = "0: Timestamp counter value always 0x0000"]
    B_0x0 = 0,
    #[doc = "1: Timestamp counter value incremented according to TCP"]
    B_0x1 = 1,
    #[doc = "2: External timestamp counter from TIM3 value (tim3_cnt\\[0:15\\])"]
    B_0x2 = 2,
    #[doc = "3: Same as 00."]
    B_0x3 = 3,
}
impl From<TSS_A> for u8 {
    #[inline(always)]
    fn from(variant: TSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSS_A {
    type Ux = u8;
}
impl crate::IsEnum for TSS_A {}
#[doc = "Field `TSS` reader - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type TSS_R = crate::FieldReader<TSS_A>;
impl TSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSS_A {
        match self.bits {
            0 => TSS_A::B_0x0,
            1 => TSS_A::B_0x1,
            2 => TSS_A::B_0x2,
            3 => TSS_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TSS_A::B_0x0
    }
    #[doc = "Timestamp counter value incremented according to TCP"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TSS_A::B_0x1
    }
    #[doc = "External timestamp counter from TIM3 value (tim3_cnt\\[0:15\\])"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TSS_A::B_0x2
    }
    #[doc = "Same as 00."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TSS_A::B_0x3
    }
}
#[doc = "Field `TSS` writer - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TSS_A, crate::Safe>;
impl<'a, REG> TSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSS_A::B_0x0)
    }
    #[doc = "Timestamp counter value incremented according to TCP"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSS_A::B_0x1)
    }
    #[doc = "External timestamp counter from TIM3 value (tim3_cnt\\[0:15\\])"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TSS_A::B_0x2)
    }
    #[doc = "Same as 00."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TSS_A::B_0x3)
    }
}
#[doc = "Field `TCP` reader - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times ."]
pub type TCP_R = crate::FieldReader;
#[doc = "Field `TCP` writer - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times ."]
pub type TCP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn TSS(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times ."]
    #[inline(always)]
    pub fn TCP(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn TSS(&mut self) -> TSS_W<'_, TSCC_SPEC> {
        TSS_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times ."]
    #[inline(always)]
    pub fn TCP(&mut self) -> TCP_W<'_, TSCC_SPEC> {
        TCP_W::new(self, 16)
    }
}
#[doc = "FDCAN timestamp counter configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCC_SPEC;
impl crate::RegisterSpec for TSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscc::R`](R) reader structure"]
impl crate::Readable for TSCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tscc::W`](W) writer structure"]
impl crate::Writable for TSCC_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TSCC to value 0"]
impl crate::Resettable for TSCC_SPEC {}
