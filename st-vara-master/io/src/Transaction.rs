#![no_std]
use gstd::{prelude::*, ActorId, Debug, Decode, Default, Encode, TypeInfo};

pub mod Transaction {
    #[derive(Encode, Decode, TypeInfo, Debug)]
    pub enum TransactionOperation {
        //Agrega stVaras a la cuenta, aumenta el total de stVaras
        Mint {
            receipt: ActorId,
            amount: u128,
        },
        //Retira stVaras de la cuenta, disminuye el total de stVaras
        Burn {
            receipt: ActorId,
            amount: u128,
        },
        //Retira stVaras de la cuenta send y aumenta el total de stVaras de la cuenta receipt
        Transfer {
            send: ActorId,
            receipt: ActorId,
            amount: u128,
        },
        //Aumenta stVARAs y disminuye VARAs de la cuenta
        Stack {
            receipt: ActorId,
            amount: u128,
        },
        //Aumenta VARAs y disminuye stVARAs de la cuenta
        Unstack {
            receipt: ActorId,
            amount: u128,
        },
        //
    }

    #[derive(Encode, Decode, TypeInfo, Debug)]
    pub enum TransactionEvent {
        Ok,
        Error,
        Balance(u128),
    }

    pub enum TransactionStatus {
        SUCCESS,
        FAILED,
    }
}
