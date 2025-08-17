#[doc = "Register `TOCC` reader"]
pub type R = crate::R<TOCC_SPEC>;
#[doc = "Register `TOCC` writer"]
pub type W = crate::W<TOCC_SPEC>;
#[doc = "Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETOC_A {
    #[doc = "0: Timeout counter disabled"]
    B_0x0 = 0,
    #[doc = "1: Timeout counter enabled"]
    B_0x1 = 1,
}
impl From<ETOC_A> for bool {
    #[inline(always)]
    fn from(variant: ETOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETOC` reader - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type ETOC_R = crate::BitReader<ETOC_A>;
impl ETOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETOC_A {
        match self.bits {
            false => ETOC_A::B_0x0,
            true => ETOC_A::B_0x1,
        }
    }
    #[doc = "Timeout counter disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ETOC_A::B_0x0
    }
    #[doc = "Timeout counter enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ETOC_A::B_0x1
    }
}
#[doc = "Field `ETOC` writer - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type ETOC_W<'a, REG> = crate::BitWriter<'a, REG, ETOC_A>;
impl<'a, REG> ETOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout counter disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETOC_A::B_0x0)
    }
    #[doc = "Timeout counter enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETOC_A::B_0x1)
    }
}
#[doc = "Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOS_A {
    #[doc = "0: Continuous operation"]
    B_0x0 = 0,
    #[doc = "1: Timeout controlled by Tx event FIFO"]
    B_0x1 = 1,
    #[doc = "2: Timeout controlled by Rx FIFO 0"]
    B_0x2 = 2,
    #[doc = "3: Timeout controlled by Rx FIFO 1"]
    B_0x3 = 3,
}
impl From<TOS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TOS_A {
    type Ux = u8;
}
impl crate::IsEnum for TOS_A {}
#[doc = "Field `TOS` reader - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type TOS_R = crate::FieldReader<TOS_A>;
impl TOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOS_A {
        match self.bits {
            0 => TOS_A::B_0x0,
            1 => TOS_A::B_0x1,
            2 => TOS_A::B_0x2,
            3 => TOS_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TOS_A::B_0x0
    }
    #[doc = "Timeout controlled by Tx event FIFO"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TOS_A::B_0x1
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TOS_A::B_0x2
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TOS_A::B_0x3
    }
}
#[doc = "Field `TOS` writer - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type TOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TOS_A, crate::Safe>;
impl<'a, REG> TOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOS_A::B_0x0)
    }
    #[doc = "Timeout controlled by Tx event FIFO"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOS_A::B_0x1)
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TOS_A::B_0x2)
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TOS_A::B_0x3)
    }
}
#[doc = "Field `TOP` reader - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
pub type TOP_R = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn ETOC(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn TOS(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
    #[inline(always)]
    pub fn TOP(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn ETOC(&mut self) -> ETOC_W<'_, TOCC_SPEC> {
        ETOC_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn TOS(&mut self) -> TOS_W<'_, TOCC_SPEC> {
        TOS_W::new(self, 1)
    }
    #[doc = "Bits 16:31 - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
    #[inline(always)]
    pub fn TOP(&mut self) -> TOP_W<'_, TOCC_SPEC> {
        TOP_W::new(self, 16)
    }
}
#[doc = "FDCAN timeout counter configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tocc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCC_SPEC;
impl crate::RegisterSpec for TOCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocc::R`](R) reader structure"]
impl crate::Readable for TOCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tocc::W`](W) writer structure"]
impl crate::Writable for TOCC_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TOCC to value 0xffff_0000"]
impl crate::Resettable for TOCC_SPEC {
    const RESET_VALUE: u32 = 0xffff_0000;
}
