#[doc = "Register `ICSR` reader"]
pub type R = crate::R<ICSR_SPEC>;
#[doc = "Register `ICSR` writer"]
pub type W = crate::W<ICSR_SPEC>;
#[doc = "Wakeup timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTWF_A {
    #[doc = "0: Wakeup timer configuration update not allowed except in initialization mode"]
    B_0x0 = 0,
    #[doc = "1: Wakeup timer configuration update allowed"]
    B_0x1 = 1,
}
impl From<WUTWF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTWF` reader - Wakeup timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
pub type WUTWF_R = crate::BitReader<WUTWF_A>;
impl WUTWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUTWF_A {
        match self.bits {
            false => WUTWF_A::B_0x0,
            true => WUTWF_A::B_0x1,
        }
    }
    #[doc = "Wakeup timer configuration update not allowed except in initialization mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUTWF_A::B_0x0
    }
    #[doc = "Wakeup timer configuration update allowed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUTWF_A::B_0x1
    }
}
#[doc = "Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHPF_A {
    #[doc = "0: No shift operation is pending"]
    B_0x0 = 0,
    #[doc = "1: A shift operation is pending"]
    B_0x1 = 1,
}
impl From<SHPF_A> for bool {
    #[inline(always)]
    fn from(variant: SHPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHPF` reader - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
pub type SHPF_R = crate::BitReader<SHPF_A>;
impl SHPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SHPF_A {
        match self.bits {
            false => SHPF_A::B_0x0,
            true => SHPF_A::B_0x1,
        }
    }
    #[doc = "No shift operation is pending"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SHPF_A::B_0x0
    }
    #[doc = "A shift operation is pending"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SHPF_A::B_0x1
    }
}
#[doc = "Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITS_A {
    #[doc = "0: Calendar has not been initialized"]
    B_0x0 = 0,
    #[doc = "1: Calendar has been initialized"]
    B_0x1 = 1,
}
impl From<INITS_A> for bool {
    #[inline(always)]
    fn from(variant: INITS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITS` reader - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state)."]
pub type INITS_R = crate::BitReader<INITS_A>;
impl INITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INITS_A {
        match self.bits {
            false => INITS_A::B_0x0,
            true => INITS_A::B_0x1,
        }
    }
    #[doc = "Calendar has not been initialized"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == INITS_A::B_0x0
    }
    #[doc = "Calendar has been initialized"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == INITS_A::B_0x1
    }
}
#[doc = "Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSF_A {
    #[doc = "0: Calendar shadow registers not yet synchronized"]
    B_0x0 = 0,
    #[doc = "1: Calendar shadow registers synchronized"]
    B_0x1 = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
pub type RSF_R = crate::BitReader<RSF_A>;
impl RSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::B_0x0,
            true => RSF_A::B_0x1,
        }
    }
    #[doc = "Calendar shadow registers not yet synchronized"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RSF_A::B_0x0
    }
    #[doc = "Calendar shadow registers synchronized"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RSF_A::B_0x1
    }
}
#[doc = "Field `RSF` writer - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG, RSF_A>;
impl<'a, REG> RSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calendar shadow registers not yet synchronized"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RSF_A::B_0x0)
    }
    #[doc = "Calendar shadow registers synchronized"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RSF_A::B_0x1)
    }
}
#[doc = "Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITF_A {
    #[doc = "0: Calendar registers update is not allowed"]
    B_0x0 = 0,
    #[doc = "1: Calendar registers update is allowed"]
    B_0x1 = 1,
}
impl From<INITF_A> for bool {
    #[inline(always)]
    fn from(variant: INITF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITF` reader - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated."]
pub type INITF_R = crate::BitReader<INITF_A>;
impl INITF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INITF_A {
        match self.bits {
            false => INITF_A::B_0x0,
            true => INITF_A::B_0x1,
        }
    }
    #[doc = "Calendar registers update is not allowed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == INITF_A::B_0x0
    }
    #[doc = "Calendar registers update is allowed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == INITF_A::B_0x1
    }
}
#[doc = "Initialization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT_A {
    #[doc = "0: Free running mode"]
    B_0x0 = 0,
    #[doc = "1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER), plus BIN and BCDU fields. Counters are stopped and start counting from the new value when INIT is reset."]
    B_0x1 = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization mode"]
pub type INIT_R = crate::BitReader<INIT_A>;
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::B_0x0,
            true => INIT_A::B_0x1,
        }
    }
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == INIT_A::B_0x0
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER), plus BIN and BCDU fields. Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == INIT_A::B_0x1
    }
}
#[doc = "Field `INIT` writer - Initialization mode"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG, INIT_A>;
impl<'a, REG> INIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(INIT_A::B_0x0)
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER), plus BIN and BCDU fields. Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(INIT_A::B_0x1)
    }
}
#[doc = "Binary mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BIN_A {
    #[doc = "0: Free running BCD calendar mode (Binary mode disabled)."]
    B_0x0 = 0,
    #[doc = "1: Free running Binary mode (BCD mode disabled)"]
    B_0x1 = 1,
    #[doc = "2: Free running BCD calendar and Binary modes"]
    B_0x2 = 2,
    #[doc = "3: Free running BCD calendar and Binary modes"]
    B_0x3 = 3,
}
impl From<BIN_A> for u8 {
    #[inline(always)]
    fn from(variant: BIN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BIN_A {
    type Ux = u8;
}
impl crate::IsEnum for BIN_A {}
#[doc = "Field `BIN` reader - Binary mode"]
pub type BIN_R = crate::FieldReader<BIN_A>;
impl BIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIN_A {
        match self.bits {
            0 => BIN_A::B_0x0,
            1 => BIN_A::B_0x1,
            2 => BIN_A::B_0x2,
            3 => BIN_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Free running BCD calendar mode (Binary mode disabled)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BIN_A::B_0x0
    }
    #[doc = "Free running Binary mode (BCD mode disabled)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BIN_A::B_0x1
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == BIN_A::B_0x2
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == BIN_A::B_0x3
    }
}
#[doc = "Field `BIN` writer - Binary mode"]
pub type BIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BIN_A, crate::Safe>;
impl<'a, REG> BIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Free running BCD calendar mode (Binary mode disabled)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BIN_A::B_0x0)
    }
    #[doc = "Free running Binary mode (BCD mode disabled)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BIN_A::B_0x1)
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BIN_A::B_0x2)
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BIN_A::B_0x3)
    }
}
#[doc = "BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCDU_A {
    #[doc = "0: 1s calendar increment is generated each time SS\\[7:0\\] = 0"]
    B_0x0 = 0,
    #[doc = "1: 1s calendar increment is generated each time SS\\[8:0\\] = 0"]
    B_0x1 = 1,
    #[doc = "2: 1s calendar increment is generated each time SS\\[9:0\\] = 0"]
    B_0x2 = 2,
    #[doc = "3: 1s calendar increment is generated each time SS\\[10:0\\] = 0"]
    B_0x3 = 3,
    #[doc = "4: 1s calendar increment is generated each time SS\\[11:0\\] = 0"]
    B_0x4 = 4,
    #[doc = "5: 1s calendar increment is generated each time SS\\[12:0\\] = 0"]
    B_0x5 = 5,
    #[doc = "6: 1s calendar increment is generated each time SS\\[13:0\\] = 0"]
    B_0x6 = 6,
    #[doc = "7: 1s calendar increment is generated each time SS\\[14:0\\] = 0"]
    B_0x7 = 7,
}
impl From<BCDU_A> for u8 {
    #[inline(always)]
    fn from(variant: BCDU_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BCDU_A {
    type Ux = u8;
}
impl crate::IsEnum for BCDU_A {}
#[doc = "Field `BCDU` reader - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
pub type BCDU_R = crate::FieldReader<BCDU_A>;
impl BCDU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BCDU_A {
        match self.bits {
            0 => BCDU_A::B_0x0,
            1 => BCDU_A::B_0x1,
            2 => BCDU_A::B_0x2,
            3 => BCDU_A::B_0x3,
            4 => BCDU_A::B_0x4,
            5 => BCDU_A::B_0x5,
            6 => BCDU_A::B_0x6,
            7 => BCDU_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "1s calendar increment is generated each time SS\\[7:0\\] = 0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BCDU_A::B_0x0
    }
    #[doc = "1s calendar increment is generated each time SS\\[8:0\\] = 0"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BCDU_A::B_0x1
    }
    #[doc = "1s calendar increment is generated each time SS\\[9:0\\] = 0"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == BCDU_A::B_0x2
    }
    #[doc = "1s calendar increment is generated each time SS\\[10:0\\] = 0"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == BCDU_A::B_0x3
    }
    #[doc = "1s calendar increment is generated each time SS\\[11:0\\] = 0"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == BCDU_A::B_0x4
    }
    #[doc = "1s calendar increment is generated each time SS\\[12:0\\] = 0"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == BCDU_A::B_0x5
    }
    #[doc = "1s calendar increment is generated each time SS\\[13:0\\] = 0"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == BCDU_A::B_0x6
    }
    #[doc = "1s calendar increment is generated each time SS\\[14:0\\] = 0"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == BCDU_A::B_0x7
    }
}
#[doc = "Field `BCDU` writer - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
pub type BCDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BCDU_A, crate::Safe>;
impl<'a, REG> BCDU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1s calendar increment is generated each time SS\\[7:0\\] = 0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU_A::B_0x0)
    }
    #[doc = "1s calendar increment is generated each time SS\\[8:0\\] = 0"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU_A::B_0x1)
    }
    #[doc = "1s calendar increment is generated each time SS\\[9:0\\] = 0"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU_A::B_0x2)
    }
    #[doc = "1s calendar increment is generated each time SS\\[10:0\\] = 0"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU_A::B_0x3)
    }
    #[doc = "1s calendar increment is generated each time SS\\[11:0\\] = 0"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU_A::B_0x4)
    }
    #[doc = "1s calendar increment is generated each time SS\\[12:0\\] = 0"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU_A::B_0x5)
    }
    #[doc = "1s calendar increment is generated each time SS\\[13:0\\] = 0"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU_A::B_0x6)
    }
    #[doc = "1s calendar increment is generated each time SS\\[14:0\\] = 0"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU_A::B_0x7)
    }
}
#[doc = "Field `RECALPF` reader - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to Re-calibration on-the-fly."]
pub type RECALPF_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Wakeup timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
    #[inline(always)]
    pub fn WUTWF(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
    #[inline(always)]
    pub fn SHPF(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state)."]
    #[inline(always)]
    pub fn INITS(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
    #[inline(always)]
    pub fn RSF(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated."]
    #[inline(always)]
    pub fn INITF(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn INIT(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Binary mode"]
    #[inline(always)]
    pub fn BIN(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
    #[inline(always)]
    pub fn BCDU(&self) -> BCDU_R {
        BCDU_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 16 - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to Re-calibration on-the-fly."]
    #[inline(always)]
    pub fn RECALPF(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
    #[inline(always)]
    pub fn RSF(&mut self) -> RSF_W<'_, ICSR_SPEC> {
        RSF_W::new(self, 5)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn INIT(&mut self) -> INIT_W<'_, ICSR_SPEC> {
        INIT_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Binary mode"]
    #[inline(always)]
    pub fn BIN(&mut self) -> BIN_W<'_, ICSR_SPEC> {
        BIN_W::new(self, 8)
    }
    #[doc = "Bits 10:12 - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
    #[inline(always)]
    pub fn BCDU(&mut self) -> BCDU_W<'_, ICSR_SPEC> {
        BCDU_W::new(self, 10)
    }
}
#[doc = "RTC initialization control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsr::R`](R) reader structure"]
impl crate::Readable for ICSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icsr::W`](W) writer structure"]
impl crate::Writable for ICSR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICSR to value 0x07"]
impl crate::Resettable for ICSR_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
