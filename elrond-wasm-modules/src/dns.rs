mod dns_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait Dns {
        #[payable("EGLD")]
        #[endpoint]
        fn register(&self, name: &ManagedBuffer);
    }
}

elrond_wasm::imports!();

/// Standard smart contract module that deals with registering usernames in a DNS contract.
///
/// Elrond usernames/herotags need to be requested by the beneficiary.
/// For a contract, this means that they need an endpoint via which to request a username from the DNS.
///
#[elrond_wasm::module]
pub trait DnsModule {
    #[proxy]
    fn dns_proxy(&self, to: ManagedAddress) -> dns_proxy::Proxy<Self::Api>;

    #[payable("EGLD")]
    #[only_owner]
    #[endpoint(dnsRegister)]
    fn dns_register(
        &self,
        dns_address: ManagedAddress,
        name: ManagedBuffer,
        #[payment] payment: BigUint,
    ) {
        self.dns_proxy(dns_address)
            .register(&name)
            .with_egld_transfer(payment)
            .async_call()
            .call_and_exit()
    }
}
