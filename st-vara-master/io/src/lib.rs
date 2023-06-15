use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, Debug, Decode, Default, Encode, TypeInfo};
use hashbrown::HashMap;
use Account;

pub mod Transaction;
use crate::Transaction::*;
/// El contrato inteligente de stVARA se encarga de cambiar VARAs por stVaras y viceversa.
/// La proporcion de cambio es 1:stvara_value.
/// 
/// 
///

pub struct StVaraMaster;

impl Metadata for StVaraMaster {
    type Init = In<InitStVaraMaster>;
    type Handle = InOut<TransactionOperation, TransactionEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = StVaraMasterState;
}

pub struct StVaraMasterState {
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

#[derive(Encode, Decode, TypeInfo, Debug, Default)]
pub struct InitStVaraMaster {
    admin: ActorId,
    master_account: ActorId,
    st_vara_value: u128,
}
