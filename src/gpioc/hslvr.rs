#[doc = "Register `HSLVR` reader"]
pub type R = crate::R<HSLVR_SPEC>;
#[doc = "Register `HSLVR` writer"]
pub type W = crate::W<HSLVR_SPEC>;
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV0_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV0_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV0` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV0_R = crate::BitReader<HSLV0_A>;
impl HSLV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV0_A {
        match self.bits {
            false => HSLV0_A::B_0x0,
            true => HSLV0_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV0_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV0_A::B_0x1
    }
}
#[doc = "Field `HSLV0` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV0_W<'a, REG> = crate::BitWriter<'a, REG, HSLV0_A>;
impl<'a, REG> HSLV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV0_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV0_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV1_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV1_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV1` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV1_R = crate::BitReader<HSLV1_A>;
impl HSLV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV1_A {
        match self.bits {
            false => HSLV1_A::B_0x0,
            true => HSLV1_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV1_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV1_A::B_0x1
    }
}
#[doc = "Field `HSLV1` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV1_W<'a, REG> = crate::BitWriter<'a, REG, HSLV1_A>;
impl<'a, REG> HSLV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV1_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV1_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV2_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV2_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV2` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV2_R = crate::BitReader<HSLV2_A>;
impl HSLV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV2_A {
        match self.bits {
            false => HSLV2_A::B_0x0,
            true => HSLV2_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV2_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV2_A::B_0x1
    }
}
#[doc = "Field `HSLV2` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV2_W<'a, REG> = crate::BitWriter<'a, REG, HSLV2_A>;
impl<'a, REG> HSLV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV2_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV2_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV3_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV3_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV3` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV3_R = crate::BitReader<HSLV3_A>;
impl HSLV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV3_A {
        match self.bits {
            false => HSLV3_A::B_0x0,
            true => HSLV3_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV3_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV3_A::B_0x1
    }
}
#[doc = "Field `HSLV3` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV3_W<'a, REG> = crate::BitWriter<'a, REG, HSLV3_A>;
impl<'a, REG> HSLV3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV3_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV3_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV4_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV4_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV4` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV4_R = crate::BitReader<HSLV4_A>;
impl HSLV4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV4_A {
        match self.bits {
            false => HSLV4_A::B_0x0,
            true => HSLV4_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV4_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV4_A::B_0x1
    }
}
#[doc = "Field `HSLV4` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV4_W<'a, REG> = crate::BitWriter<'a, REG, HSLV4_A>;
impl<'a, REG> HSLV4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV4_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV4_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV5_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV5_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV5` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV5_R = crate::BitReader<HSLV5_A>;
impl HSLV5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV5_A {
        match self.bits {
            false => HSLV5_A::B_0x0,
            true => HSLV5_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV5_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV5_A::B_0x1
    }
}
#[doc = "Field `HSLV5` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV5_W<'a, REG> = crate::BitWriter<'a, REG, HSLV5_A>;
impl<'a, REG> HSLV5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV5_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV5_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV6_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV6_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV6` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV6_R = crate::BitReader<HSLV6_A>;
impl HSLV6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV6_A {
        match self.bits {
            false => HSLV6_A::B_0x0,
            true => HSLV6_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV6_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV6_A::B_0x1
    }
}
#[doc = "Field `HSLV6` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV6_W<'a, REG> = crate::BitWriter<'a, REG, HSLV6_A>;
impl<'a, REG> HSLV6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV6_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV6_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV7_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV7_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV7` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV7_R = crate::BitReader<HSLV7_A>;
impl HSLV7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV7_A {
        match self.bits {
            false => HSLV7_A::B_0x0,
            true => HSLV7_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV7_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV7_A::B_0x1
    }
}
#[doc = "Field `HSLV7` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV7_W<'a, REG> = crate::BitWriter<'a, REG, HSLV7_A>;
impl<'a, REG> HSLV7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV7_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV7_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV8_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV8_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV8` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV8_R = crate::BitReader<HSLV8_A>;
impl HSLV8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV8_A {
        match self.bits {
            false => HSLV8_A::B_0x0,
            true => HSLV8_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV8_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV8_A::B_0x1
    }
}
#[doc = "Field `HSLV8` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV8_W<'a, REG> = crate::BitWriter<'a, REG, HSLV8_A>;
impl<'a, REG> HSLV8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV8_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV8_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV9_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV9_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV9` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV9_R = crate::BitReader<HSLV9_A>;
impl HSLV9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV9_A {
        match self.bits {
            false => HSLV9_A::B_0x0,
            true => HSLV9_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV9_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV9_A::B_0x1
    }
}
#[doc = "Field `HSLV9` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV9_W<'a, REG> = crate::BitWriter<'a, REG, HSLV9_A>;
impl<'a, REG> HSLV9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV9_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV9_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV10_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV10_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV10` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV10_R = crate::BitReader<HSLV10_A>;
impl HSLV10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV10_A {
        match self.bits {
            false => HSLV10_A::B_0x0,
            true => HSLV10_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV10_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV10_A::B_0x1
    }
}
#[doc = "Field `HSLV10` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV10_W<'a, REG> = crate::BitWriter<'a, REG, HSLV10_A>;
impl<'a, REG> HSLV10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV10_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV10_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV11_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV11_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV11` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV11_R = crate::BitReader<HSLV11_A>;
impl HSLV11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV11_A {
        match self.bits {
            false => HSLV11_A::B_0x0,
            true => HSLV11_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV11_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV11_A::B_0x1
    }
}
#[doc = "Field `HSLV11` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV11_W<'a, REG> = crate::BitWriter<'a, REG, HSLV11_A>;
impl<'a, REG> HSLV11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV11_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV11_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV12_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV12_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV12` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV12_R = crate::BitReader<HSLV12_A>;
impl HSLV12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV12_A {
        match self.bits {
            false => HSLV12_A::B_0x0,
            true => HSLV12_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV12_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV12_A::B_0x1
    }
}
#[doc = "Field `HSLV12` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV12_W<'a, REG> = crate::BitWriter<'a, REG, HSLV12_A>;
impl<'a, REG> HSLV12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV12_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV12_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV13_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV13_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV13` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV13_R = crate::BitReader<HSLV13_A>;
impl HSLV13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV13_A {
        match self.bits {
            false => HSLV13_A::B_0x0,
            true => HSLV13_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV13_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV13_A::B_0x1
    }
}
#[doc = "Field `HSLV13` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV13_W<'a, REG> = crate::BitWriter<'a, REG, HSLV13_A>;
impl<'a, REG> HSLV13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV13_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV13_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV14_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV14_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV14` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV14_R = crate::BitReader<HSLV14_A>;
impl HSLV14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV14_A {
        match self.bits {
            false => HSLV14_A::B_0x0,
            true => HSLV14_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV14_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV14_A::B_0x1
    }
}
#[doc = "Field `HSLV14` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV14_W<'a, REG> = crate::BitWriter<'a, REG, HSLV14_A>;
impl<'a, REG> HSLV14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV14_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV14_A::B_0x1)
    }
}
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV15_A {
    #[doc = "0: I/O speed optimization disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O speed optimization enabled"]
    B_0x1 = 1,
}
impl From<HSLV15_A> for bool {
    #[inline(always)]
    fn from(variant: HSLV15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV15` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV15_R = crate::BitReader<HSLV15_A>;
impl HSLV15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV15_A {
        match self.bits {
            false => HSLV15_A::B_0x0,
            true => HSLV15_A::B_0x1,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSLV15_A::B_0x0
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSLV15_A::B_0x1
    }
}
#[doc = "Field `HSLV15` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV15_W<'a, REG> = crate::BitWriter<'a, REG, HSLV15_A>;
impl<'a, REG> HSLV15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV15_A::B_0x0)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV15_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV0(&self) -> HSLV0_R {
        HSLV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV1(&self) -> HSLV1_R {
        HSLV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV2(&self) -> HSLV2_R {
        HSLV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV3(&self) -> HSLV3_R {
        HSLV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV4(&self) -> HSLV4_R {
        HSLV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV5(&self) -> HSLV5_R {
        HSLV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV6(&self) -> HSLV6_R {
        HSLV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV7(&self) -> HSLV7_R {
        HSLV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV8(&self) -> HSLV8_R {
        HSLV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV9(&self) -> HSLV9_R {
        HSLV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV10(&self) -> HSLV10_R {
        HSLV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV11(&self) -> HSLV11_R {
        HSLV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV12(&self) -> HSLV12_R {
        HSLV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV13(&self) -> HSLV13_R {
        HSLV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV14(&self) -> HSLV14_R {
        HSLV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV15(&self) -> HSLV15_R {
        HSLV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV0(&mut self) -> HSLV0_W<'_, HSLVR_SPEC> {
        HSLV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV1(&mut self) -> HSLV1_W<'_, HSLVR_SPEC> {
        HSLV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV2(&mut self) -> HSLV2_W<'_, HSLVR_SPEC> {
        HSLV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV3(&mut self) -> HSLV3_W<'_, HSLVR_SPEC> {
        HSLV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV4(&mut self) -> HSLV4_W<'_, HSLVR_SPEC> {
        HSLV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV5(&mut self) -> HSLV5_W<'_, HSLVR_SPEC> {
        HSLV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV6(&mut self) -> HSLV6_W<'_, HSLVR_SPEC> {
        HSLV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV7(&mut self) -> HSLV7_W<'_, HSLVR_SPEC> {
        HSLV7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV8(&mut self) -> HSLV8_W<'_, HSLVR_SPEC> {
        HSLV8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV9(&mut self) -> HSLV9_W<'_, HSLVR_SPEC> {
        HSLV9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV10(&mut self) -> HSLV10_W<'_, HSLVR_SPEC> {
        HSLV10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV11(&mut self) -> HSLV11_W<'_, HSLVR_SPEC> {
        HSLV11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV12(&mut self) -> HSLV12_W<'_, HSLVR_SPEC> {
        HSLV12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV13(&mut self) -> HSLV13_W<'_, HSLVR_SPEC> {
        HSLV13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV14(&mut self) -> HSLV14_W<'_, HSLVR_SPEC> {
        HSLV14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn HSLV15(&mut self) -> HSLV15_W<'_, HSLVR_SPEC> {
        HSLV15_W::new(self, 15)
    }
}
#[doc = "GPIO high-speed low-voltage register\n\nYou can [`read`](crate::Reg::read) this register and get [`hslvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hslvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSLVR_SPEC;
impl crate::RegisterSpec for HSLVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hslvr::R`](R) reader structure"]
impl crate::Readable for HSLVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hslvr::W`](W) writer structure"]
impl crate::Writable for HSLVR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HSLVR to value 0"]
impl crate::Resettable for HSLVR_SPEC {}
