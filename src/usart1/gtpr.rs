#[doc = "Register `GTPR` reader"]
pub type R = crate::R<GTPR_SPEC>;
#[doc = "Register `GTPR` writer"]
pub type W = crate::W<GTPR_SPEC>;
#[doc = "Prescaler value PSC\\[7:0\\] = IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\\[4:0\\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\] must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0' when the Smartcard and IrDA modes are not supported. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Reserved - do not program this value"]
    B_0x0_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE = 0,
    #[doc = "1: divides the source clock by 1"]
    B_0x1_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE = 1,
    #[doc = "2: divides the source clock by 2"]
    B_0x2_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE = 2,
    #[doc = "3: divides the source clock by 6"]
    B_0x3_SMARTCARD_MODE = 3,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSC_A {
    type Ux = u8;
}
impl crate::IsEnum for PSC_A {}
#[doc = "Field `PSC` reader - Prescaler value PSC\\[7:0\\] = IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\\[4:0\\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\] must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0' when the Smartcard and IrDA modes are not supported. Refer to ."]
pub type PSC_R = crate::FieldReader<PSC_A>;
impl PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PSC_A> {
        match self.bits {
            0 => Some(PSC_A::B_0x0_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE),
            1 => Some(PSC_A::B_0x1_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE),
            2 => Some(PSC_A::B_0x2_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE),
            3 => Some(PSC_A::B_0x3_SMARTCARD_MODE),
            _ => None,
        }
    }
    #[doc = "Reserved - do not program this value"]
    #[inline(always)]
    pub fn is_B_0x0_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE(&self) -> bool {
        *self == PSC_A::B_0x0_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE
    }
    #[doc = "divides the source clock by 1"]
    #[inline(always)]
    pub fn is_B_0x1_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE(&self) -> bool {
        *self == PSC_A::B_0x1_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE
    }
    #[doc = "divides the source clock by 2"]
    #[inline(always)]
    pub fn is_B_0x2_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE(&self) -> bool {
        *self == PSC_A::B_0x2_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE
    }
    #[doc = "divides the source clock by 6"]
    #[inline(always)]
    pub fn is_B_0x3_SMARTCARD_MODE(&self) -> bool {
        *self == PSC_A::B_0x3_SMARTCARD_MODE
    }
}
#[doc = "Field `PSC` writer - Prescaler value PSC\\[7:0\\] = IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\\[4:0\\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\] must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0' when the Smartcard and IrDA modes are not supported. Refer to ."]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8, PSC_A>;
impl<'a, REG> PSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved - do not program this value"]
    #[inline(always)]
    pub fn B_0x0_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::B_0x0_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE)
    }
    #[doc = "divides the source clock by 1"]
    #[inline(always)]
    pub fn B_0x1_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::B_0x1_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE)
    }
    #[doc = "divides the source clock by 2"]
    #[inline(always)]
    pub fn B_0x2_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::B_0x2_IRDA_LOW_POWER_AND_NORMAL_IRDA_MODE)
    }
    #[doc = "divides the source clock by 6"]
    #[inline(always)]
    pub fn B_0x3_SMARTCARD_MODE(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::B_0x3_SMARTCARD_MODE)
    }
}
#[doc = "Field `GT` reader - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type GT_R = crate::FieldReader;
#[doc = "Field `GT` writer - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type GT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Prescaler value PSC\\[7:0\\] = IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\\[4:0\\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\] must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0' when the Smartcard and IrDA modes are not supported. Refer to ."]
    #[inline(always)]
    pub fn PSC(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn GT(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value PSC\\[7:0\\] = IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\\[4:0\\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\] must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0' when the Smartcard and IrDA modes are not supported. Refer to ."]
    #[inline(always)]
    pub fn PSC(&mut self) -> PSC_W<'_, GTPR_SPEC> {
        PSC_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn GT(&mut self) -> GT_W<'_, GTPR_SPEC> {
        GT_W::new(self, 8)
    }
}
#[doc = "USART guard time and prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTPR_SPEC;
impl crate::RegisterSpec for GTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpr::R`](R) reader structure"]
impl crate::Readable for GTPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtpr::W`](W) writer structure"]
impl crate::Writable for GTPR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GTPR to value 0"]
impl crate::Resettable for GTPR_SPEC {}
