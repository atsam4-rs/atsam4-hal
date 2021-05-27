use super::{
    Controller,
    EthernetAddress,
    rx,
    tx,
};

use crate::{
    clock::{Enabled, GmacClock},
    gpio::{Pd0, Pd1, Pd2, Pd3, Pd4, Pd5, Pd6, Pd7, Pd8, Pd9, PfA},
    pac::GMAC,
};

pub struct Builder {
    ethernet_address: EthernetAddress,
    alternate_addresses: [Option<EthernetAddress>; 3],
    alternate_address_count: usize,
    disable_broadcast: bool,
    phy_address: u8,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            ethernet_address: EthernetAddress::default(),
            alternate_addresses: [None; 3],
            alternate_address_count: 0,
            disable_broadcast: false,
            phy_address: 0,
        }
    }

    pub fn set_ethernet_address(mut self, ethernet_address: EthernetAddress) -> Self {
        self.ethernet_address = ethernet_address;
        self
    }

    pub fn ethernet_address(&self) -> EthernetAddress {
        self.ethernet_address
    }

    pub fn add_alternate_ethernet_address(mut self, ethernet_address: EthernetAddress) -> Self {
        if self.alternate_address_count == 3 {
            panic!("Attempted to add more than three alternate addresses");
        }

        self.alternate_addresses[self.alternate_address_count] = Some(ethernet_address);
        self.alternate_address_count += 1;
        self
    }

    pub fn alternate_ethernet_address_count(&self) -> usize {
        self.alternate_address_count
    }

    pub fn alternate_ethernet_address(&self, index: usize) -> EthernetAddress {
        if index >= self.alternate_address_count {
            panic!("Attempted to access invalid alternate address");
        }

        self.alternate_addresses[index].unwrap()
    }

    pub fn disable_broadcast(mut self) -> Self {
        self.disable_broadcast = true;
        self
    }

    pub fn has_disable_broadcast(&self) -> bool {
        self.disable_broadcast
    }

    pub fn set_phy_address(mut self, phy_address: u8) -> Self {
        self.phy_address = phy_address;
        self
    }

    pub fn phy_address(&self) -> u8 {
        self.phy_address
    }

    pub fn build<'rxtx>(
        self,
        gmac: GMAC,
        clock: GmacClock<Enabled>,
        grefck: Pd0<PfA>,
        gtxen:  Pd1<PfA>,
        gtx0:   Pd2<PfA>,
        gtx1:   Pd3<PfA>,
        gcrsdv: Pd4<PfA>,
        grx0:   Pd5<PfA>,
        grx1:   Pd6<PfA>,
        grxer:  Pd7<PfA>,
        gmdc:   Pd8<PfA>,
        gmdio:  Pd9<PfA>,
        rx: &'rxtx mut dyn rx::DescriptorTable,
        tx: &'rxtx mut dyn tx::DescriptorTable,
    ) -> Controller<'rxtx> {
        Controller::new(gmac, clock, grefck, gtxen, gtx0, gtx1, gcrsdv, grx0, grx1, grxer, gmdc, gmdio, rx, tx, self)
    }
}
