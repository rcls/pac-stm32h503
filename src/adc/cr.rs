#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADEN_A {
    #[doc = "0: ADC is disabled (OFF state)"]
    B_0x0 = 0,
    #[doc = "1: Write 1 to enable the ADC."]
    B_0x1 = 1,
}
impl From<ADEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` reader - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)"]
pub type ADEN_R = crate::BitReader<ADEN_A>;
impl ADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADEN_A {
        match self.bits {
            false => ADEN_A::B_0x0,
            true => ADEN_A::B_0x1,
        }
    }
    #[doc = "ADC is disabled (OFF state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADEN_A::B_0x0
    }
    #[doc = "Write 1 to enable the ADC."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADEN_A::B_0x1
    }
}
#[doc = "Field `ADEN` writer - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)"]
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG, ADEN_A>;
impl<'a, REG> ADEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC is disabled (OFF state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADEN_A::B_0x0)
    }
    #[doc = "Write 1 to enable the ADC."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADEN_A::B_0x1)
    }
}
#[doc = "ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDIS_A {
    #[doc = "0: no ADDIS command ongoing"]
    B_0x0 = 0,
    #[doc = "1: Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress."]
    B_0x1 = 1,
}
impl From<ADDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` reader - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
pub type ADDIS_R = crate::BitReader<ADDIS_A>;
impl ADDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDIS_A {
        match self.bits {
            false => ADDIS_A::B_0x0,
            true => ADDIS_A::B_0x1,
        }
    }
    #[doc = "no ADDIS command ongoing"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADDIS_A::B_0x0
    }
    #[doc = "Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADDIS_A::B_0x1
    }
}
#[doc = "Field `ADDIS` writer - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG, ADDIS_A>;
impl<'a, REG> ADDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no ADDIS command ongoing"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADDIS_A::B_0x0)
    }
    #[doc = "Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADDIS_A::B_0x1)
    }
}
#[doc = "ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTART_A {
    #[doc = "0: No ADC regular conversion is ongoing."]
    B_0x0 = 0,
    #[doc = "1: Write 1 to start regular conversions. Read 1 means that the ADC is operating and eventually converting a regular channel."]
    B_0x1 = 1,
}
impl From<ADSTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` reader - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
pub type ADSTART_R = crate::BitReader<ADSTART_A>;
impl ADSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADSTART_A {
        match self.bits {
            false => ADSTART_A::B_0x0,
            true => ADSTART_A::B_0x1,
        }
    }
    #[doc = "No ADC regular conversion is ongoing."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADSTART_A::B_0x0
    }
    #[doc = "Write 1 to start regular conversions. Read 1 means that the ADC is operating and eventually converting a regular channel."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADSTART_A::B_0x1
    }
}
#[doc = "Field `ADSTART` writer - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG, ADSTART_A>;
impl<'a, REG> ADSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ADC regular conversion is ongoing."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTART_A::B_0x0)
    }
    #[doc = "Write 1 to start regular conversions. Read 1 means that the ADC is operating and eventually converting a regular channel."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTART_A::B_0x1)
    }
}
#[doc = "ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JADSTART_A {
    #[doc = "0: No ADC injected conversion is ongoing."]
    B_0x0 = 0,
    #[doc = "1: Write 1 to start injected conversions. Read 1 means that the ADC is operating and eventually converting an injected channel."]
    B_0x1 = 1,
}
impl From<JADSTART_A> for bool {
    #[inline(always)]
    fn from(variant: JADSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JADSTART` reader - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
pub type JADSTART_R = crate::BitReader<JADSTART_A>;
impl JADSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JADSTART_A {
        match self.bits {
            false => JADSTART_A::B_0x0,
            true => JADSTART_A::B_0x1,
        }
    }
    #[doc = "No ADC injected conversion is ongoing."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JADSTART_A::B_0x0
    }
    #[doc = "Write 1 to start injected conversions. Read 1 means that the ADC is operating and eventually converting an injected channel."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JADSTART_A::B_0x1
    }
}
#[doc = "Field `JADSTART` writer - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
pub type JADSTART_W<'a, REG> = crate::BitWriter<'a, REG, JADSTART_A>;
impl<'a, REG> JADSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ADC injected conversion is ongoing."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JADSTART_A::B_0x0)
    }
    #[doc = "Write 1 to start injected conversions. Read 1 means that the ADC is operating and eventually converting an injected channel."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JADSTART_A::B_0x1)
    }
}
#[doc = "ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTP_A {
    #[doc = "0: No ADC stop regular conversion command ongoing"]
    B_0x0 = 0,
    #[doc = "1: Write 1 to stop regular conversions ongoing. Read 1 means that an ADSTP command is in progress."]
    B_0x1 = 1,
}
impl From<ADSTP_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` reader - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)."]
pub type ADSTP_R = crate::BitReader<ADSTP_A>;
impl ADSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADSTP_A {
        match self.bits {
            false => ADSTP_A::B_0x0,
            true => ADSTP_A::B_0x1,
        }
    }
    #[doc = "No ADC stop regular conversion command ongoing"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADSTP_A::B_0x0
    }
    #[doc = "Write 1 to stop regular conversions ongoing. Read 1 means that an ADSTP command is in progress."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADSTP_A::B_0x1
    }
}
#[doc = "Field `ADSTP` writer - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)."]
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG, ADSTP_A>;
impl<'a, REG> ADSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ADC stop regular conversion command ongoing"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTP_A::B_0x0)
    }
    #[doc = "Write 1 to stop regular conversions ongoing. Read 1 means that an ADSTP command is in progress."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTP_A::B_0x1)
    }
}
#[doc = "ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JADSTP_A {
    #[doc = "0: No ADC stop injected conversion command ongoing"]
    B_0x0 = 0,
    #[doc = "1: Write 1 to stop injected conversions ongoing. Read 1 means that an ADSTP command is in progress."]
    B_0x1 = 1,
}
impl From<JADSTP_A> for bool {
    #[inline(always)]
    fn from(variant: JADSTP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JADSTP` reader - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)"]
pub type JADSTP_R = crate::BitReader<JADSTP_A>;
impl JADSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JADSTP_A {
        match self.bits {
            false => JADSTP_A::B_0x0,
            true => JADSTP_A::B_0x1,
        }
    }
    #[doc = "No ADC stop injected conversion command ongoing"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JADSTP_A::B_0x0
    }
    #[doc = "Write 1 to stop injected conversions ongoing. Read 1 means that an ADSTP command is in progress."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JADSTP_A::B_0x1
    }
}
#[doc = "Field `JADSTP` writer - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)"]
pub type JADSTP_W<'a, REG> = crate::BitWriter<'a, REG, JADSTP_A>;
impl<'a, REG> JADSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ADC stop injected conversion command ongoing"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JADSTP_A::B_0x0)
    }
    #[doc = "Write 1 to stop injected conversions ongoing. Read 1 means that an ADSTP command is in progress."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JADSTP_A::B_0x1)
    }
}
#[doc = "ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADVREGEN_A {
    #[doc = "0: ADC Voltage regulator disabled"]
    B_0x0 = 0,
    #[doc = "1: ADC Voltage regulator enabled."]
    B_0x1 = 1,
}
impl From<ADVREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADVREGEN` reader - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type ADVREGEN_R = crate::BitReader<ADVREGEN_A>;
impl ADVREGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADVREGEN_A {
        match self.bits {
            false => ADVREGEN_A::B_0x0,
            true => ADVREGEN_A::B_0x1,
        }
    }
    #[doc = "ADC Voltage regulator disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADVREGEN_A::B_0x0
    }
    #[doc = "ADC Voltage regulator enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADVREGEN_A::B_0x1
    }
}
#[doc = "Field `ADVREGEN` writer - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG, ADVREGEN_A>;
impl<'a, REG> ADVREGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC Voltage regulator disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN_A::B_0x0)
    }
    #[doc = "ADC Voltage regulator enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN_A::B_0x1)
    }
}
#[doc = "Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEPPWD_A {
    #[doc = "0: ADC not in Deep-power down"]
    B_0x0 = 0,
    #[doc = "1: ADC in Deep-power-down (default reset state)"]
    B_0x1 = 1,
}
impl From<DEEPPWD_A> for bool {
    #[inline(always)]
    fn from(variant: DEEPPWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEEPPWD` reader - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DEEPPWD_R = crate::BitReader<DEEPPWD_A>;
impl DEEPPWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEEPPWD_A {
        match self.bits {
            false => DEEPPWD_A::B_0x0,
            true => DEEPPWD_A::B_0x1,
        }
    }
    #[doc = "ADC not in Deep-power down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DEEPPWD_A::B_0x0
    }
    #[doc = "ADC in Deep-power-down (default reset state)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DEEPPWD_A::B_0x1
    }
}
#[doc = "Field `DEEPPWD` writer - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG, DEEPPWD_A>;
impl<'a, REG> DEEPPWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC not in Deep-power down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPPWD_A::B_0x0)
    }
    #[doc = "ADC in Deep-power-down (default reset state)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPPWD_A::B_0x1)
    }
}
#[doc = "Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALDIF_A {
    #[doc = "0: Writing ADCAL launches a calibration in Single-ended inputs mode."]
    B_0x0 = 0,
    #[doc = "1: Writing ADCAL launches a calibration in Differential inputs mode."]
    B_0x1 = 1,
}
impl From<ADCALDIF_A> for bool {
    #[inline(always)]
    fn from(variant: ADCALDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCALDIF` reader - Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type ADCALDIF_R = crate::BitReader<ADCALDIF_A>;
impl ADCALDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCALDIF_A {
        match self.bits {
            false => ADCALDIF_A::B_0x0,
            true => ADCALDIF_A::B_0x1,
        }
    }
    #[doc = "Writing ADCAL launches a calibration in Single-ended inputs mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADCALDIF_A::B_0x0
    }
    #[doc = "Writing ADCAL launches a calibration in Differential inputs mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADCALDIF_A::B_0x1
    }
}
#[doc = "Field `ADCALDIF` writer - Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type ADCALDIF_W<'a, REG> = crate::BitWriter<'a, REG, ADCALDIF_A>;
impl<'a, REG> ADCALDIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing ADCAL launches a calibration in Single-ended inputs mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALDIF_A::B_0x0)
    }
    #[doc = "Writing ADCAL launches a calibration in Differential inputs mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALDIF_A::B_0x1)
    }
}
#[doc = "ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCAL_A {
    #[doc = "0: Calibration complete"]
    B_0x0 = 0,
    #[doc = "1: Write 1 to calibrate the ADC. Read at 1 means that a calibration in progress."]
    B_0x1 = 1,
}
impl From<ADCAL_A> for bool {
    #[inline(always)]
    fn from(variant: ADCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` reader - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)"]
pub type ADCAL_R = crate::BitReader<ADCAL_A>;
impl ADCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCAL_A {
        match self.bits {
            false => ADCAL_A::B_0x0,
            true => ADCAL_A::B_0x1,
        }
    }
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADCAL_A::B_0x0
    }
    #[doc = "Write 1 to calibrate the ADC. Read at 1 means that a calibration in progress."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADCAL_A::B_0x1
    }
}
#[doc = "Field `ADCAL` writer - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)"]
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG, ADCAL_A>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCAL_A::B_0x0)
    }
    #[doc = "Write 1 to calibrate the ADC. Read at 1 means that a calibration in progress."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCAL_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)"]
    #[inline(always)]
    pub fn ADEN(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn ADDIS(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
    #[inline(always)]
    pub fn ADSTART(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
    #[inline(always)]
    pub fn JADSTART(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)."]
    #[inline(always)]
    pub fn ADSTP(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)"]
    #[inline(always)]
    pub fn JADSTP(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn ADVREGEN(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn DEEPPWD(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn ADCALDIF(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)"]
    #[inline(always)]
    pub fn ADCAL(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)"]
    #[inline(always)]
    pub fn ADEN(&mut self) -> ADEN_W<'_, CR_SPEC> {
        ADEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn ADDIS(&mut self) -> ADDIS_W<'_, CR_SPEC> {
        ADDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
    #[inline(always)]
    pub fn ADSTART(&mut self) -> ADSTART_W<'_, CR_SPEC> {
        ADSTART_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
    #[inline(always)]
    pub fn JADSTART(&mut self) -> JADSTART_W<'_, CR_SPEC> {
        JADSTART_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)."]
    #[inline(always)]
    pub fn ADSTP(&mut self) -> ADSTP_W<'_, CR_SPEC> {
        ADSTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)"]
    #[inline(always)]
    pub fn JADSTP(&mut self) -> JADSTP_W<'_, CR_SPEC> {
        JADSTP_W::new(self, 5)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn ADVREGEN(&mut self) -> ADVREGEN_W<'_, CR_SPEC> {
        ADVREGEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn DEEPPWD(&mut self) -> DEEPPWD_W<'_, CR_SPEC> {
        DEEPPWD_W::new(self, 29)
    }
    #[doc = "Bit 30 - Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn ADCALDIF(&mut self) -> ADCALDIF_W<'_, CR_SPEC> {
        ADCALDIF_W::new(self, 30)
    }
    #[doc = "Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)"]
    #[inline(always)]
    pub fn ADCAL(&mut self) -> ADCAL_W<'_, CR_SPEC> {
        ADCAL_W::new(self, 31)
    }
}
#[doc = "ADC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0x2000_0000"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
