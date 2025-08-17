#[doc = "Register `SHIFTR` writer"]
pub type W = crate::W<SHIFTR_SPEC>;
#[doc = "Field `SUBFS` writer - Subtract a fraction of a second These bits are write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in RTC_ICSR). The value which is written to SUBFS is added to the synchronous prescaler counter. Since this counter counts down, this operation effectively subtracts from (delays) the clock by: Delay (seconds) = SUBFS / (PREDIV_S + 1) A fraction of a second can effectively be added to the clock (advancing the clock) when the ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by: Advance (seconds) = (1 - (SUBFS / (PREDIV_S + 1))). In mixed BCD-binary mode (BIN=10 or 11), the SUBFS\\[14:BCDU+8\\] must be written with 0. Note: Writing to SUBFS causes RSF to be cleared. Software can then wait until RSF = 1 to be sure that the shadow registers have been updated with the shifted time."]
pub type SUBFS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Add one second This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in RTC_ICSR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1S_A {
    #[doc = "0: No effect"]
    B_0x0 = 0,
    #[doc = "1: Add one second to the clock/calendar"]
    B_0x1 = 1,
}
impl From<ADD1S_A> for bool {
    #[inline(always)]
    fn from(variant: ADD1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD1S` writer - Add one second This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in RTC_ICSR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation."]
pub type ADD1S_W<'a, REG> = crate::BitWriter<'a, REG, ADD1S_A>;
impl<'a, REG> ADD1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1S_A::B_0x0)
    }
    #[doc = "Add one second to the clock/calendar"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1S_A::B_0x1)
    }
}
impl W {
    #[doc = "Bits 0:14 - Subtract a fraction of a second These bits are write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in RTC_ICSR). The value which is written to SUBFS is added to the synchronous prescaler counter. Since this counter counts down, this operation effectively subtracts from (delays) the clock by: Delay (seconds) = SUBFS / (PREDIV_S + 1) A fraction of a second can effectively be added to the clock (advancing the clock) when the ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by: Advance (seconds) = (1 - (SUBFS / (PREDIV_S + 1))). In mixed BCD-binary mode (BIN=10 or 11), the SUBFS\\[14:BCDU+8\\] must be written with 0. Note: Writing to SUBFS causes RSF to be cleared. Software can then wait until RSF = 1 to be sure that the shadow registers have been updated with the shifted time."]
    #[inline(always)]
    pub fn SUBFS(&mut self) -> SUBFS_W<'_, SHIFTR_SPEC> {
        SUBFS_W::new(self, 0)
    }
    #[doc = "Bit 31 - Add one second This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF = 1, in RTC_ICSR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation."]
    #[inline(always)]
    pub fn ADD1S(&mut self) -> ADD1S_W<'_, SHIFTR_SPEC> {
        ADD1S_W::new(self, 31)
    }
}
#[doc = "RTC shift control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHIFTR_SPEC;
impl crate::RegisterSpec for SHIFTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`shiftr::W`](W) writer structure"]
impl crate::Writable for SHIFTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SHIFTR to value 0"]
impl crate::Resettable for SHIFTR_SPEC {}
