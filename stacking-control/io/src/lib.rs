#![no_std]
use gmeta::{Metadata};
use gstd::{msg, prelude::*, ActorId, Debug, Decode, Default, Encode, TypeInfo};

pub struct StackingControl;

//Metadata del smart contract
impl Metadata for StackingControl{
    type Init = ();
    type Handle = InOut<Action, ActionResult>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}

//Las acciones permitidas en el contrato
pub enum Action{
    //Stacking con monto
    Stake(u128),
    //Unstacking con monto
    UnStake(u128),
    //Retiro con monto
    Withdraw(u128),
    //Obtener recompensa
    GetReward,
}

//Errores que pueden surgir en la interaccion con los contratos o en el proceso
pub enum Error{
    //El usuario no cuenta con VARAs suficientes para realizar Stake o Withdraw
    InsufficientVARAs,
    //El usuario no cuenta con VARAs para realizar accion Stake o Withdraw
    ZeroVARAs,
    //El usuario no cuenta con stVARAs suficientes para realizar UnStake
    InsufficientStVARAs,
    //El usuario no cuenta con stVARAs para realizar UnStake
    ZeroStVaras,
    //El usurio no cuenta con recompensas para reclamar
    ZeroRewards,
    //El usuario no es reconocible
    StakerNotFound,
    //Ha ocurrido un error al interactuar con algun contrato
    ContractError(String),
    //No es posible realizar la transferencia de VARAs durante Stake o Withdraw
    TransferenceNotAllowed,
    //La transferencia sde VARAs durante Stake o Withdraw esta siendo procesada
    BlockedTransfer,
    //No es una accion valida
    ActionNotAllowed
}

//Resultados de las acciones
pub enum ActionResult{
    //Stackin satisfactorio, contiene el monto con el que se realizo el stacking
    SuccessStake(u128),
    //UnStacking satisfactorio, contiene el monto con el que se realizo el unstacking
    SuccessUnStake(u128),
    //Retiro satisfactorio, contiene el monto con el que se realizo el retiro
    SuccessWithdraw(u128),
    //Recompensa reclamada satisfactoriamente, contiene el monto con el que se realizo el reclamo
    RewardClaimed(u128),
    //Ha ocurrido un error, contiene el error que ocurrio
    Unsuccessful(Error)
}

//Guarda las transacciones que se han realizado
pub struct Transaction<T>{
    //Id de la transaccion
    pub id: u128;
    //Tipo de transaccion realizada
    pub transaction_kind: T;
}