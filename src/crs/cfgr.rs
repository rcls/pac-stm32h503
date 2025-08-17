#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "Field `RELOAD` reader - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section 10.5.2 for more details about counter behavior."]
pub type RELOAD_R = crate::FieldReader<u16>;
#[doc = "Field `RELOAD` writer - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section 10.5.2 for more details about counter behavior."]
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FELIM` reader - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\] bits of the CRS_ISR register. Refer to Section 10.5.3 for more details about FECAP evaluation."]
pub type FELIM_R = crate::FieldReader;
#[doc = "Field `FELIM` writer - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\] bits of the CRS_ISR register. Refer to Section 10.5.3 for more details about FECAP evaluation."]
pub type FELIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCDIV_A {
    #[doc = "0: SYNC not divided (default)"]
    B_0x0 = 0,
    #[doc = "1: SYNC divided by 2"]
    B_0x1 = 1,
    #[doc = "2: SYNC divided by 4"]
    B_0x2 = 2,
    #[doc = "3: SYNC divided by 8"]
    B_0x3 = 3,
    #[doc = "4: SYNC divided by 16"]
    B_0x4 = 4,
    #[doc = "5: SYNC divided by 32"]
    B_0x5 = 5,
    #[doc = "6: SYNC divided by 64"]
    B_0x6 = 6,
    #[doc = "7: SYNC divided by 128"]
    B_0x7 = 7,
}
impl From<SYNCDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for SYNCDIV_A {}
#[doc = "Field `SYNCDIV` reader - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
pub type SYNCDIV_R = crate::FieldReader<SYNCDIV_A>;
impl SYNCDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCDIV_A {
        match self.bits {
            0 => SYNCDIV_A::B_0x0,
            1 => SYNCDIV_A::B_0x1,
            2 => SYNCDIV_A::B_0x2,
            3 => SYNCDIV_A::B_0x3,
            4 => SYNCDIV_A::B_0x4,
            5 => SYNCDIV_A::B_0x5,
            6 => SYNCDIV_A::B_0x6,
            7 => SYNCDIV_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "SYNC not divided (default)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SYNCDIV_A::B_0x0
    }
    #[doc = "SYNC divided by 2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SYNCDIV_A::B_0x1
    }
    #[doc = "SYNC divided by 4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SYNCDIV_A::B_0x2
    }
    #[doc = "SYNC divided by 8"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SYNCDIV_A::B_0x3
    }
    #[doc = "SYNC divided by 16"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SYNCDIV_A::B_0x4
    }
    #[doc = "SYNC divided by 32"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SYNCDIV_A::B_0x5
    }
    #[doc = "SYNC divided by 64"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SYNCDIV_A::B_0x6
    }
    #[doc = "SYNC divided by 128"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SYNCDIV_A::B_0x7
    }
}
#[doc = "Field `SYNCDIV` writer - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
pub type SYNCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SYNCDIV_A, crate::Safe>;
impl<'a, REG> SYNCDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYNC not divided (default)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV_A::B_0x0)
    }
    #[doc = "SYNC divided by 2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV_A::B_0x1)
    }
    #[doc = "SYNC divided by 4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV_A::B_0x2)
    }
    #[doc = "SYNC divided by 8"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV_A::B_0x3)
    }
    #[doc = "SYNC divided by 16"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV_A::B_0x4)
    }
    #[doc = "SYNC divided by 32"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV_A::B_0x5)
    }
    #[doc = "SYNC divided by 64"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV_A::B_0x6)
    }
    #[doc = "SYNC divided by 128"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV_A::B_0x7)
    }
}
#[doc = "SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table 68: CRS internal input/output signals for STM32U575/585): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCSRC_A {
    #[doc = "0: crs_sync_in_1 selected as SYNC signal source"]
    B_0x0 = 0,
    #[doc = "1: crs_sync_in_2 selected as SYNC signal source"]
    B_0x1 = 1,
    #[doc = "2: crs_sync_in_3 selected as SYNC signal source"]
    B_0x2 = 2,
    #[doc = "3: crs_sync_in_4 selected as SYNC signal source"]
    B_0x3 = 3,
}
impl From<SYNCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCSRC_A {
    type Ux = u8;
}
impl crate::IsEnum for SYNCSRC_A {}
#[doc = "Field `SYNCSRC` reader - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table 68: CRS internal input/output signals for STM32U575/585): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
pub type SYNCSRC_R = crate::FieldReader<SYNCSRC_A>;
impl SYNCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCSRC_A {
        match self.bits {
            0 => SYNCSRC_A::B_0x0,
            1 => SYNCSRC_A::B_0x1,
            2 => SYNCSRC_A::B_0x2,
            3 => SYNCSRC_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "crs_sync_in_1 selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SYNCSRC_A::B_0x0
    }
    #[doc = "crs_sync_in_2 selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SYNCSRC_A::B_0x1
    }
    #[doc = "crs_sync_in_3 selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SYNCSRC_A::B_0x2
    }
    #[doc = "crs_sync_in_4 selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SYNCSRC_A::B_0x3
    }
}
#[doc = "Field `SYNCSRC` writer - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table 68: CRS internal input/output signals for STM32U575/585): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
pub type SYNCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCSRC_A, crate::Safe>;
impl<'a, REG> SYNCSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "crs_sync_in_1 selected as SYNC signal source"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC_A::B_0x0)
    }
    #[doc = "crs_sync_in_2 selected as SYNC signal source"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC_A::B_0x1)
    }
    #[doc = "crs_sync_in_3 selected as SYNC signal source"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC_A::B_0x2)
    }
    #[doc = "crs_sync_in_4 selected as SYNC signal source"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC_A::B_0x3)
    }
}
#[doc = "SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCPOL_A {
    #[doc = "0: SYNC active on rising edge (default)"]
    B_0x0 = 0,
    #[doc = "1: SYNC active on falling edge"]
    B_0x1 = 1,
}
impl From<SYNCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCPOL` reader - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
pub type SYNCPOL_R = crate::BitReader<SYNCPOL_A>;
impl SYNCPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCPOL_A {
        match self.bits {
            false => SYNCPOL_A::B_0x0,
            true => SYNCPOL_A::B_0x1,
        }
    }
    #[doc = "SYNC active on rising edge (default)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SYNCPOL_A::B_0x0
    }
    #[doc = "SYNC active on falling edge"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SYNCPOL_A::B_0x1
    }
}
#[doc = "Field `SYNCPOL` writer - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
pub type SYNCPOL_W<'a, REG> = crate::BitWriter<'a, REG, SYNCPOL_A>;
impl<'a, REG> SYNCPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYNC active on rising edge (default)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPOL_A::B_0x0)
    }
    #[doc = "SYNC active on falling edge"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPOL_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section 10.5.2 for more details about counter behavior."]
    #[inline(always)]
    pub fn RELOAD(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\] bits of the CRS_ISR register. Refer to Section 10.5.3 for more details about FECAP evaluation."]
    #[inline(always)]
    pub fn FELIM(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
    #[inline(always)]
    pub fn SYNCDIV(&self) -> SYNCDIV_R {
        SYNCDIV_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table 68: CRS internal input/output signals for STM32U575/585): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
    #[inline(always)]
    pub fn SYNCSRC(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
    #[inline(always)]
    pub fn SYNCPOL(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section 10.5.2 for more details about counter behavior."]
    #[inline(always)]
    pub fn RELOAD(&mut self) -> RELOAD_W<'_, CFGR_SPEC> {
        RELOAD_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\] bits of the CRS_ISR register. Refer to Section 10.5.3 for more details about FECAP evaluation."]
    #[inline(always)]
    pub fn FELIM(&mut self) -> FELIM_W<'_, CFGR_SPEC> {
        FELIM_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
    #[inline(always)]
    pub fn SYNCDIV(&mut self) -> SYNCDIV_W<'_, CFGR_SPEC> {
        SYNCDIV_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table 68: CRS internal input/output signals for STM32U575/585): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
    #[inline(always)]
    pub fn SYNCSRC(&mut self) -> SYNCSRC_W<'_, CFGR_SPEC> {
        SYNCSRC_W::new(self, 28)
    }
    #[doc = "Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
    #[inline(always)]
    pub fn SYNCPOL(&mut self) -> SYNCPOL_W<'_, CFGR_SPEC> {
        SYNCPOL_W::new(self, 31)
    }
}
#[doc = "CRS configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR to value 0x2022_bb7f"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: u32 = 0x2022_bb7f;
}
