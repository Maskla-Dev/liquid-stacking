#![no_std]
use gstd::{msg, prelude::*, ActorId, Debug, Decode, Default, Encode, TypeInfo};
use hashbrown::HashMap;
use st_vara_master_io::Account;
use st_vara_master_io::TransactionStatus;
use st_vara_master_io::TransactionOperation;

#[derive(Encode, Decode, TypeInfo, Debug, Default)]
pub struct StVaraMaster {
    //A quien le pertenece el contrato
    admin: ActorId,
    //Quien pueden hacer operaciones
    master_account: ActorId,
    //Registro de transacciones
    transactions: HashMap<u128, Transaction>,
    //Registro de cuentas
    accounts: HashMap<ActorId, Account>,
    //Total de stVARA tokens
    total_st_varas: u128,
    //Valor de un stVara con respecto a VARA - VARA:stVARA - 1:st_vara_value
    st_vara_value: u128,
    //Total de VARA tokens
    total_varas: u128,
    //Total de VARA tokens unstacked
    total_varas_unstacked: u128,
    //Total de VARA tokens stacked
    total_varas_stacked: u128,
}

pub struct Transaction {
    TransactionOperation: TransactionOperation,
    TransactionStatus: TransactionStatus,
}
