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
        liberado_vendedor: Balance,
        estado: Estado,
        nft_id: ink::prelude::string::String,
    }

    impl Escrow {
        /// Crea un nuevo escrow para RWAgallery
        /// arbitro = wallet RWAgallery Treasury
        /// nft_id = referencia de la obra en Chaotic
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
                liberado_vendedor: 0,
                estado: Estado::Esperando,
                nft_id,
            }
        }

        /// Comprador deposita el precio de la obra en KSM
        #[ink(message, payable)]
        pub fn depositar_comprador(&mut self) {
            assert!(self.estado == Estado::Esperando, "Estado incorrecto");
            assert!(self.env().caller() == self.comprador, "Solo el comprador");
            self.precio_total += self.env().transferred_value();
            self.estado = Estado::Bloqueado;
        }

        /// Vendedor deposita garantia (10% del precio)
        #[ink(message, payable)]
        pub fn depositar_vendedor(&mut self) {
            assert!(self.estado == Estado::Bloqueado, "Estado incorrecto");
            assert!(self.env().caller() == self.vendedor, "Solo el vendedor");
            self.deposito_vendedor += self.env().transferred_value();
        }

        /// Comprador confirma recepcion de la obra fisica
        /// Se libera: 27% vendedor inmediato, 3% arbitro, 70% queda hasta confirmacion final
        #[ink(message)]
        pub fn confirmar_entrega(&mut self) {
            assert!(self.estado == Estado::Bloqueado, "Estado incorrecto");
            assert!(
                self.env().caller() == self.comprador
                    || self.env().caller() == self.arbitro,
                "No autorizado"
            );

            let total = self.precio_total;
            let pago_inmediato = total * 27 / 100;   // 27% al vendedor
            let comision = total * 3 / 100;           // 3% a RWAgallery
            let escrow_final = total - pago_inmediato - comision; // 70% restante

            // Pago inmediato al vendedor
            self.env().transfer(self.vendedor, pago_inmediato).unwrap();
            // Comision a RWAgallery Treasury
            self.env().transfer(self.arbitro, comision).unwrap();
            // Resto al vendedor (70% una vez confirmada entrega)
            self.env().transfer(self.vendedor, escrow_final).unwrap();
            // Devolver garantia al vendedor
            self.env().transfer(self.vendedor, self.deposito_vendedor).unwrap();

            self.liberado_vendedor = total;
            self.estado = Estado::Completado;
        }

        /// Arbitro puede reembolsar al comprador en caso de disputa
        #[ink(message)]
        pub fn reembolsar(&mut self) {
            assert!(self.env().caller() == self.arbitro, "Solo el arbitro");
            assert!(self.estado == Estado::Bloqueado, "Estado incorrecto");
            self.env().transfer(self.comprador, self.precio_total).unwrap();
            self.estado = Estado::Reembolsado;
        }

        /// Consultar estado del escrow
        #[ink(message)]
        pub fn get_estado(&self) -> u8 {
            match self.estado {
                Estado::Esperando => 0,
                Estado::Bloqueado => 1,
                Estado::Completado => 2,
                Estado::Reembolsado => 3,
            }
        }

        /// Consultar NFT asociado
        #[ink(message)]
        pub fn get_nft_id(&self) -> ink::prelude::string::String {
            self.nft_id.clone()
        }

        /// Consultar precio total depositado
        #[ink(message)]
        pub fn get_precio(&self) -> Balance {
            self.precio_total
        }
    }
}
