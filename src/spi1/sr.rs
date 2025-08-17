#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Rx-Packet available In I2S mode, it must be interpreted as follow: RxFIFO level is lower than FTHLV In I2S mode, it must be interpreted as follow: RxFIFO level is higher or equal to FTHLV RXP flag is changed by hardware. It monitors number of overall data currently available at RxFIFO if SPI is enabled. It has to be checked once a data packet is completely read out from RxFIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP_A {
    #[doc = "0: In SPI mode, it must be interpreted as follow: RxFIFO is empty or a not complete data packet is received"]
    B_0x0 = 0,
    #[doc = "1: In SPI mode, it must be interpreted as follow: RxFIFO contains at least 1 data packet"]
    B_0x1 = 1,
}
impl From<RXP_A> for bool {
    #[inline(always)]
    fn from(variant: RXP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXP` reader - Rx-Packet available In I2S mode, it must be interpreted as follow: RxFIFO level is lower than FTHLV In I2S mode, it must be interpreted as follow: RxFIFO level is higher or equal to FTHLV RXP flag is changed by hardware. It monitors number of overall data currently available at RxFIFO if SPI is enabled. It has to be checked once a data packet is completely read out from RxFIFO."]
pub type RXP_R = crate::BitReader<RXP_A>;
impl RXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXP_A {
        match self.bits {
            false => RXP_A::B_0x0,
            true => RXP_A::B_0x1,
        }
    }
    #[doc = "In SPI mode, it must be interpreted as follow: RxFIFO is empty or a not complete data packet is received"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXP_A::B_0x0
    }
    #[doc = "In SPI mode, it must be interpreted as follow: RxFIFO contains at least 1 data packet"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXP_A::B_0x1
    }
}
#[doc = "Tx-Packet space available In I2S mode, it must be interpreted as follow: there is less than FTHLV free locations in the TxFIFO In I2S mode, it must be interpreted as follow: there is FTHLV or more than FTHLV free locations in the TxFIFO TXP flag is changed by hardware. It monitors overall space currently available at TxFIFO no matter if SPI is enabled or not. It has to be checked once a complete data packet is stored at TxFIFO.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP_A {
    #[doc = "0: In SPI mode, it must be interpreted as follow: there is not enough space to locate next data packet at TxFIFO"]
    B_0x0 = 0,
    #[doc = "1: In SPI mode, it must be interpreted as follow: TxFIFO has enough free location to host 1 data packet"]
    B_0x1 = 1,
}
impl From<TXP_A> for bool {
    #[inline(always)]
    fn from(variant: TXP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXP` reader - Tx-Packet space available In I2S mode, it must be interpreted as follow: there is less than FTHLV free locations in the TxFIFO In I2S mode, it must be interpreted as follow: there is FTHLV or more than FTHLV free locations in the TxFIFO TXP flag is changed by hardware. It monitors overall space currently available at TxFIFO no matter if SPI is enabled or not. It has to be checked once a complete data packet is stored at TxFIFO."]
pub type TXP_R = crate::BitReader<TXP_A>;
impl TXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXP_A {
        match self.bits {
            false => TXP_A::B_0x0,
            true => TXP_A::B_0x1,
        }
    }
    #[doc = "In SPI mode, it must be interpreted as follow: there is not enough space to locate next data packet at TxFIFO"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXP_A::B_0x0
    }
    #[doc = "In SPI mode, it must be interpreted as follow: TxFIFO has enough free location to host 1 data packet"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXP_A::B_0x1
    }
}
#[doc = "duplex packet DXP flag is set whenever both TXP and RXP flags are set regardless SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXP_A {
    #[doc = "0: TxFIFO is Full and/or RxFIFO is Empty"]
    B_0x0 = 0,
    #[doc = "1: both TxFIFO has space for write and RxFIFO contains for read a single packet at least"]
    B_0x1 = 1,
}
impl From<DXP_A> for bool {
    #[inline(always)]
    fn from(variant: DXP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DXP` reader - duplex packet DXP flag is set whenever both TXP and RXP flags are set regardless SPI mode."]
pub type DXP_R = crate::BitReader<DXP_A>;
impl DXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DXP_A {
        match self.bits {
            false => DXP_A::B_0x0,
            true => DXP_A::B_0x1,
        }
    }
    #[doc = "TxFIFO is Full and/or RxFIFO is Empty"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DXP_A::B_0x0
    }
    #[doc = "both TxFIFO has space for write and RxFIFO contains for read a single packet at least"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DXP_A::B_0x1
    }
}
#[doc = "end of transfer EOT is set by hardware as soon as a full transfer is complete, that is when SPI is re-enabled or when TSIZE number of data have been transmitted and/or received on the SPI. EOT is cleared when SPI is re-enabled or by writing 1 to EOTC bit of SPI_IFCR optionally. EOT flag triggers an interrupt if EOTIE bit is set. If DXP flag is used until TXTF flag is set and DXPIE is cleared, EOT can be used to download the last packets contained into RxFIFO in one-shot. In master, EOT event terminates the data transaction and handles SS output optionally. When CRC is applied, the EOT event is extended over the CRC frame transaction. To restart the internal state machine properly, SPI is strongly suggested to be disabled and re-enabled before next transaction starts despite its setting is not changed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOT_A {
    #[doc = "0: transfer is on-going or not started"]
    B_0x0 = 0,
    #[doc = "1: transfer complete"]
    B_0x1 = 1,
}
impl From<EOT_A> for bool {
    #[inline(always)]
    fn from(variant: EOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` reader - end of transfer EOT is set by hardware as soon as a full transfer is complete, that is when SPI is re-enabled or when TSIZE number of data have been transmitted and/or received on the SPI. EOT is cleared when SPI is re-enabled or by writing 1 to EOTC bit of SPI_IFCR optionally. EOT flag triggers an interrupt if EOTIE bit is set. If DXP flag is used until TXTF flag is set and DXPIE is cleared, EOT can be used to download the last packets contained into RxFIFO in one-shot. In master, EOT event terminates the data transaction and handles SS output optionally. When CRC is applied, the EOT event is extended over the CRC frame transaction. To restart the internal state machine properly, SPI is strongly suggested to be disabled and re-enabled before next transaction starts despite its setting is not changed."]
pub type EOT_R = crate::BitReader<EOT_A>;
impl EOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOT_A {
        match self.bits {
            false => EOT_A::B_0x0,
            true => EOT_A::B_0x1,
        }
    }
    #[doc = "transfer is on-going or not started"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EOT_A::B_0x0
    }
    #[doc = "transfer complete"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EOT_A::B_0x1
    }
}
#[doc = "transmission transfer filled TXTF is set by hardware as soon as all of the data packets in a transfer have been submitted for transmission by application software or DMA, that is when TSIZE number of data have been pushed into the TxFIFO. This bit is cleared by software write 1 to TXTFC bit of SPI_IFCR exclusively. TXTF flag triggers an interrupt if TXTFIE bit is set. TXTF setting clears the TXPIE and DXPIE masks so to off-load application software from calculating when to disable TXP and DXP interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTF_A {
    #[doc = "0: upload of TxFIFO is on-going or not started"]
    B_0x0 = 0,
    #[doc = "1: TxFIFO upload is finished"]
    B_0x1 = 1,
}
impl From<TXTF_A> for bool {
    #[inline(always)]
    fn from(variant: TXTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXTF` reader - transmission transfer filled TXTF is set by hardware as soon as all of the data packets in a transfer have been submitted for transmission by application software or DMA, that is when TSIZE number of data have been pushed into the TxFIFO. This bit is cleared by software write 1 to TXTFC bit of SPI_IFCR exclusively. TXTF flag triggers an interrupt if TXTFIE bit is set. TXTF setting clears the TXPIE and DXPIE masks so to off-load application software from calculating when to disable TXP and DXP interrupts."]
pub type TXTF_R = crate::BitReader<TXTF_A>;
impl TXTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXTF_A {
        match self.bits {
            false => TXTF_A::B_0x0,
            true => TXTF_A::B_0x1,
        }
    }
    #[doc = "upload of TxFIFO is on-going or not started"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXTF_A::B_0x0
    }
    #[doc = "TxFIFO upload is finished"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXTF_A::B_0x1
    }
}
#[doc = "underrun This bit is cleared when SPI is re-enabled or by writing 1 to UDRC bit of SPI_IFCR optionally. Note: In SPI mode, the UDR flag applies to Slave mode only. In I2S/PCM mode, (when available) this flag applies to Master and Slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDR_A {
    #[doc = "0: no underrun"]
    B_0x0 = 0,
    #[doc = "1: underrun detected"]
    B_0x1 = 1,
}
impl From<UDR_A> for bool {
    #[inline(always)]
    fn from(variant: UDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDR` reader - underrun This bit is cleared when SPI is re-enabled or by writing 1 to UDRC bit of SPI_IFCR optionally. Note: In SPI mode, the UDR flag applies to Slave mode only. In I2S/PCM mode, (when available) this flag applies to Master and Slave mode"]
pub type UDR_R = crate::BitReader<UDR_A>;
impl UDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDR_A {
        match self.bits {
            false => UDR_A::B_0x0,
            true => UDR_A::B_0x1,
        }
    }
    #[doc = "no underrun"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UDR_A::B_0x0
    }
    #[doc = "underrun detected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UDR_A::B_0x1
    }
}
#[doc = "overrun This bit is cleared when SPI is re-enabled or by writing 1 to OVRC bit of SPI_IFCR optionally.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_A {
    #[doc = "0: no overrun"]
    B_0x0 = 0,
    #[doc = "1: overrun detected"]
    B_0x1 = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - overrun This bit is cleared when SPI is re-enabled or by writing 1 to OVRC bit of SPI_IFCR optionally."]
pub type OVR_R = crate::BitReader<OVR_A>;
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::B_0x0,
            true => OVR_A::B_0x1,
        }
    }
    #[doc = "no overrun"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OVR_A::B_0x0
    }
    #[doc = "overrun detected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OVR_A::B_0x1
    }
}
#[doc = "CRC error This bit is cleared when SPI is re-enabled or by writing 1 to CRCEC bit of SPI_IFCR optionally.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCE_A {
    #[doc = "0: no CRC error"]
    B_0x0 = 0,
    #[doc = "1: CRC error detected"]
    B_0x1 = 1,
}
impl From<CRCE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCE` reader - CRC error This bit is cleared when SPI is re-enabled or by writing 1 to CRCEC bit of SPI_IFCR optionally."]
pub type CRCE_R = crate::BitReader<CRCE_A>;
impl CRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCE_A {
        match self.bits {
            false => CRCE_A::B_0x0,
            true => CRCE_A::B_0x1,
        }
    }
    #[doc = "no CRC error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRCE_A::B_0x0
    }
    #[doc = "CRC error detected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRCE_A::B_0x1
    }
}
#[doc = "TI frame format error This bit is cleared by writing 1 to TIFREC bit of SPI_IFCR exclusively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFRE_A {
    #[doc = "0: no TI Frame Error"]
    B_0x0 = 0,
    #[doc = "1: TI frame error detected"]
    B_0x1 = 1,
}
impl From<TIFRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIFRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIFRE` reader - TI frame format error This bit is cleared by writing 1 to TIFREC bit of SPI_IFCR exclusively."]
pub type TIFRE_R = crate::BitReader<TIFRE_A>;
impl TIFRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIFRE_A {
        match self.bits {
            false => TIFRE_A::B_0x0,
            true => TIFRE_A::B_0x1,
        }
    }
    #[doc = "no TI Frame Error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIFRE_A::B_0x0
    }
    #[doc = "TI frame error detected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIFRE_A::B_0x1
    }
}
#[doc = "mode fault This bit is cleared by writing 1 to MODFC bit of SPI_IFCR exclusively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODF_A {
    #[doc = "0: no mode fault"]
    B_0x0 = 0,
    #[doc = "1: mode fault detected. When MODF is set, SPE and IOLOCK bits of SPI_CR1 register are reset and their setting is blocked."]
    B_0x1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODF` reader - mode fault This bit is cleared by writing 1 to MODFC bit of SPI_IFCR exclusively."]
pub type MODF_R = crate::BitReader<MODF_A>;
impl MODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::B_0x0,
            true => MODF_A::B_0x1,
        }
    }
    #[doc = "no mode fault"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODF_A::B_0x0
    }
    #[doc = "mode fault detected. When MODF is set, SPE and IOLOCK bits of SPI_CR1 register are reset and their setting is blocked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODF_A::B_0x1
    }
}
#[doc = "suspension status In Master mode, SUSP is set by hardware either as soon as the current frame is completed after CSUSP request is done or at master automatic suspend receive mode (MASRX bit is set at SPI_CR1 register) on RxFIFO full condition. SUSP generates an interrupt when EOTIE is set. This bit has to be cleared prior SPI is disabled and this is done by writing 1 to SUSPC bit of SPI_IFCR exclusively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP_A {
    #[doc = "0: SPI not suspended (Master mode active or other mode)."]
    B_0x0 = 0,
    #[doc = "1: Master mode is suspended (current frame completed)."]
    B_0x1 = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` reader - suspension status In Master mode, SUSP is set by hardware either as soon as the current frame is completed after CSUSP request is done or at master automatic suspend receive mode (MASRX bit is set at SPI_CR1 register) on RxFIFO full condition. SUSP generates an interrupt when EOTIE is set. This bit has to be cleared prior SPI is disabled and this is done by writing 1 to SUSPC bit of SPI_IFCR exclusively."]
pub type SUSP_R = crate::BitReader<SUSP_A>;
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::B_0x0,
            true => SUSP_A::B_0x1,
        }
    }
    #[doc = "SPI not suspended (Master mode active or other mode)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SUSP_A::B_0x0
    }
    #[doc = "Master mode is suspended (current frame completed)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SUSP_A::B_0x1
    }
}
#[doc = "TxFIFO transmission complete The flag behavior depends on TSIZE setting. When TSIZE=0 the TXC is changed by hardware exclusively and it raises each time the TxFIFO becomes empty and there is no activity on the bus. If TSIZE 0 there is no specific reason to monitor TXC as it just copies the EOT flag value including its software clearing. The TXC generates an interrupt when EOTIE is set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXC_A {
    #[doc = "0: current data transaction is still ongoing, data is available in TxFIFO or last frame transmission is on going."]
    B_0x0 = 0,
    #[doc = "1: last TxFIFO frame transmission complete"]
    B_0x1 = 1,
}
impl From<TXC_A> for bool {
    #[inline(always)]
    fn from(variant: TXC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXC` reader - TxFIFO transmission complete The flag behavior depends on TSIZE setting. When TSIZE=0 the TXC is changed by hardware exclusively and it raises each time the TxFIFO becomes empty and there is no activity on the bus. If TSIZE 0 there is no specific reason to monitor TXC as it just copies the EOT flag value including its software clearing. The TXC generates an interrupt when EOTIE is set."]
pub type TXC_R = crate::BitReader<TXC_A>;
impl TXC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXC_A {
        match self.bits {
            false => TXC_A::B_0x0,
            true => TXC_A::B_0x1,
        }
    }
    #[doc = "current data transaction is still ongoing, data is available in TxFIFO or last frame transmission is on going."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXC_A::B_0x0
    }
    #[doc = "last TxFIFO frame transmission complete"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXC_A::B_0x1
    }
}
#[doc = "RxFIFO packing level When RXWNE=0 and data size is set up to 16-bit, the value gives number of remaining data frames persisting at RxFIFO. Note: (*): Optional value when data size is set up to 8-bit only. When data size is greater than 16-bit, these bits are always read as 00. In that consequence, the single data frame received at the FIFO cannot be detected neither by RWNE nor by RXPLVL bits if data size is set from 17 to 24 bits. The user then must apply other methods like TSIZE0 or FTHLV=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXPLVL_A {
    #[doc = "0: no next frame is available at RxFIFO"]
    B_0x0 = 0,
    #[doc = "1: 1 frame is available"]
    B_0x1 = 1,
    #[doc = "2: 2 frames are available*"]
    B_0x2 = 2,
    #[doc = "3: 3 frames are available*"]
    B_0x3 = 3,
}
impl From<RXPLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPLVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXPLVL_A {
    type Ux = u8;
}
impl crate::IsEnum for RXPLVL_A {}
#[doc = "Field `RXPLVL` reader - RxFIFO packing level When RXWNE=0 and data size is set up to 16-bit, the value gives number of remaining data frames persisting at RxFIFO. Note: (*): Optional value when data size is set up to 8-bit only. When data size is greater than 16-bit, these bits are always read as 00. In that consequence, the single data frame received at the FIFO cannot be detected neither by RWNE nor by RXPLVL bits if data size is set from 17 to 24 bits. The user then must apply other methods like TSIZE0 or FTHLV=0."]
pub type RXPLVL_R = crate::FieldReader<RXPLVL_A>;
impl RXPLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXPLVL_A {
        match self.bits {
            0 => RXPLVL_A::B_0x0,
            1 => RXPLVL_A::B_0x1,
            2 => RXPLVL_A::B_0x2,
            3 => RXPLVL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no next frame is available at RxFIFO"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXPLVL_A::B_0x0
    }
    #[doc = "1 frame is available"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXPLVL_A::B_0x1
    }
    #[doc = "2 frames are available*"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == RXPLVL_A::B_0x2
    }
    #[doc = "3 frames are available*"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == RXPLVL_A::B_0x3
    }
}
#[doc = "RxFIFO word not empty Note: This bit value does not depend on DSIZE setting and keeps together with RXPLVL\\[1:0\\] information about RxFIFO occupancy by residual data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXWNE_A {
    #[doc = "0: less than four bytes of RxFIFO space is occupied by data"]
    B_0x0 = 0,
    #[doc = "1: at least four bytes of RxFIFO space is occupied by data"]
    B_0x1 = 1,
}
impl From<RXWNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXWNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXWNE` reader - RxFIFO word not empty Note: This bit value does not depend on DSIZE setting and keeps together with RXPLVL\\[1:0\\] information about RxFIFO occupancy by residual data."]
pub type RXWNE_R = crate::BitReader<RXWNE_A>;
impl RXWNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXWNE_A {
        match self.bits {
            false => RXWNE_A::B_0x0,
            true => RXWNE_A::B_0x1,
        }
    }
    #[doc = "less than four bytes of RxFIFO space is occupied by data"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXWNE_A::B_0x0
    }
    #[doc = "at least four bytes of RxFIFO space is occupied by data"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXWNE_A::B_0x1
    }
}
#[doc = "Field `CTSIZE` reader - number of data frames remaining in current TSIZE session The value is not quite reliable when traffic is ongoing on bus or during autonomous operation in low-power mode. Note: CTSIZE\\[15:0\\] bits are not available in instances with limited set of features."]
pub type CTSIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Rx-Packet available In I2S mode, it must be interpreted as follow: RxFIFO level is lower than FTHLV In I2S mode, it must be interpreted as follow: RxFIFO level is higher or equal to FTHLV RXP flag is changed by hardware. It monitors number of overall data currently available at RxFIFO if SPI is enabled. It has to be checked once a data packet is completely read out from RxFIFO."]
    #[inline(always)]
    pub fn RXP(&self) -> RXP_R {
        RXP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx-Packet space available In I2S mode, it must be interpreted as follow: there is less than FTHLV free locations in the TxFIFO In I2S mode, it must be interpreted as follow: there is FTHLV or more than FTHLV free locations in the TxFIFO TXP flag is changed by hardware. It monitors overall space currently available at TxFIFO no matter if SPI is enabled or not. It has to be checked once a complete data packet is stored at TxFIFO."]
    #[inline(always)]
    pub fn TXP(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - duplex packet DXP flag is set whenever both TXP and RXP flags are set regardless SPI mode."]
    #[inline(always)]
    pub fn DXP(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - end of transfer EOT is set by hardware as soon as a full transfer is complete, that is when SPI is re-enabled or when TSIZE number of data have been transmitted and/or received on the SPI. EOT is cleared when SPI is re-enabled or by writing 1 to EOTC bit of SPI_IFCR optionally. EOT flag triggers an interrupt if EOTIE bit is set. If DXP flag is used until TXTF flag is set and DXPIE is cleared, EOT can be used to download the last packets contained into RxFIFO in one-shot. In master, EOT event terminates the data transaction and handles SS output optionally. When CRC is applied, the EOT event is extended over the CRC frame transaction. To restart the internal state machine properly, SPI is strongly suggested to be disabled and re-enabled before next transaction starts despite its setting is not changed."]
    #[inline(always)]
    pub fn EOT(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - transmission transfer filled TXTF is set by hardware as soon as all of the data packets in a transfer have been submitted for transmission by application software or DMA, that is when TSIZE number of data have been pushed into the TxFIFO. This bit is cleared by software write 1 to TXTFC bit of SPI_IFCR exclusively. TXTF flag triggers an interrupt if TXTFIE bit is set. TXTF setting clears the TXPIE and DXPIE masks so to off-load application software from calculating when to disable TXP and DXP interrupts."]
    #[inline(always)]
    pub fn TXTF(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - underrun This bit is cleared when SPI is re-enabled or by writing 1 to UDRC bit of SPI_IFCR optionally. Note: In SPI mode, the UDR flag applies to Slave mode only. In I2S/PCM mode, (when available) this flag applies to Master and Slave mode"]
    #[inline(always)]
    pub fn UDR(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - overrun This bit is cleared when SPI is re-enabled or by writing 1 to OVRC bit of SPI_IFCR optionally."]
    #[inline(always)]
    pub fn OVR(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC error This bit is cleared when SPI is re-enabled or by writing 1 to CRCEC bit of SPI_IFCR optionally."]
    #[inline(always)]
    pub fn CRCE(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TI frame format error This bit is cleared by writing 1 to TIFREC bit of SPI_IFCR exclusively."]
    #[inline(always)]
    pub fn TIFRE(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - mode fault This bit is cleared by writing 1 to MODFC bit of SPI_IFCR exclusively."]
    #[inline(always)]
    pub fn MODF(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - suspension status In Master mode, SUSP is set by hardware either as soon as the current frame is completed after CSUSP request is done or at master automatic suspend receive mode (MASRX bit is set at SPI_CR1 register) on RxFIFO full condition. SUSP generates an interrupt when EOTIE is set. This bit has to be cleared prior SPI is disabled and this is done by writing 1 to SUSPC bit of SPI_IFCR exclusively."]
    #[inline(always)]
    pub fn SUSP(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TxFIFO transmission complete The flag behavior depends on TSIZE setting. When TSIZE=0 the TXC is changed by hardware exclusively and it raises each time the TxFIFO becomes empty and there is no activity on the bus. If TSIZE 0 there is no specific reason to monitor TXC as it just copies the EOT flag value including its software clearing. The TXC generates an interrupt when EOTIE is set."]
    #[inline(always)]
    pub fn TXC(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - RxFIFO packing level When RXWNE=0 and data size is set up to 16-bit, the value gives number of remaining data frames persisting at RxFIFO. Note: (*): Optional value when data size is set up to 8-bit only. When data size is greater than 16-bit, these bits are always read as 00. In that consequence, the single data frame received at the FIFO cannot be detected neither by RWNE nor by RXPLVL bits if data size is set from 17 to 24 bits. The user then must apply other methods like TSIZE0 or FTHLV=0."]
    #[inline(always)]
    pub fn RXPLVL(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RxFIFO word not empty Note: This bit value does not depend on DSIZE setting and keeps together with RXPLVL\\[1:0\\] information about RxFIFO occupancy by residual data."]
    #[inline(always)]
    pub fn RXWNE(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - number of data frames remaining in current TSIZE session The value is not quite reliable when traffic is ongoing on bus or during autonomous operation in low-power mode. Note: CTSIZE\\[15:0\\] bits are not available in instances with limited set of features."]
    #[inline(always)]
    pub fn CTSIZE(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SPI/I2S status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0x1002"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0x1002;
}
