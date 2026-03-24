#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod escrow {

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Estado {
        Esperando,
        Bloqueado,
        Completado,
        Reembolsado,
    }

    #[ink(storage)]
    pub struct Escrow {
        comprador: AccountId,
        vendedor: AccountId,
        arbitro: AccountId,
        precio_total: Balance,
        deposito_vendedor: Balance,
        estado: Estado,
        nft_id: ink::prelude::string::String,
    }

    impl Escrow {
        #[ink(constructor)]
        pub fn new(
            vendedor: AccountId,
            arbitro: AccountId,
            nft_id: ink::prelude::string::String,
        ) -> Self {
            Self {
                comprador: Self::env().caller(),
                vendedor,
                arbitro,
                precio_total: 0,
                deposito_vendedor: 0,
                estado: Estado::Esperando,
                nft_id,
            }
        }

        /// Comprador deposita el precio en KSM
        #[ink(message, payable)]
        pub fn depositar_comprador(&mut self) {
            assert!(self.estado == Estado::Esperando);
            assert!(self.env().caller() == self.comprador);
            self.precio_total += self.env().transferred_value();
            self.estado = Estado::Bloqueado;
        }

        /// Vendedor deposita garantia
        #[ink(message, payable)]
        pub fn depositar_vendedor(&mut self) {
            assert!(self.estado == Estado::Bloqueado);
            assert!(self.env().caller() == self.vendedor);
            self.deposito_vendedor += self.env().transferred_value();
        }

        /// Confirmar entrega - distribuye fondos 27/3/70
        #[ink(message)]
        pub fn confirmar_entrega(&mut self) {
            assert!(self.estado == Estado::Bloqueado);
            assert!(
                self.env().caller() == self.comprador
                    || self.env().caller() == self.arbitro
            );
            let total = self.precio_total;
            let pago_inmediato = total * 27 / 100;
            let comision = total * 3 / 100;
            let escrow_final = total - pago_inmediato - comision;

            self.env().transfer(self.vendedor, pago_inmediato).unwrap();
            self.env().transfer(self.arbitro, comision).unwrap();
            self.env().transfer(self.vendedor, escrow_final).unwrap();
            self.env().transfer(self.vendedor, self.deposito_vendedor).unwrap();

            self.estado = Estado::Completado;
        }

        /// Arbitro reembolsa al comprador en caso de disputa
        #[ink(message)]
        pub fn reembolsar(&mut self) {
            assert!(self.env().caller() == self.arbitro);
            assert!(self.estado == Estado::Bloqueado);
            self.env().transfer(self.comprador, self.precio_total).unwrap();
            self.estado = Estado::Reembolsado;
        }

        #[ink(message)]
        pub fn get_estado(&self) -> u8 {
            match self.estado {
                Estado::Esperando   => 0,
                Estado::Bloqueado   => 1,
                Estado::Completado  => 2,
                Estado::Reembolsado => 3,
            }
        }

        #[ink(message)]
        pub fn get_nft_id(&self) -> ink::prelude::string::String {
            self.nft_id.clone()
        }

        #[ink(message)]
        pub fn get_precio(&self) -> Balance {
            self.precio_total
        }
    }
}
