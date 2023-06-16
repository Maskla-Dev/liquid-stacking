#![no_std]
use gstd::{msg, prelude::*, ActorId, Debug, Decode, Default, Encode, TypeInfo};
use hashbrown::HashMap;
use StackingAction;

#[derive(Encode, Decode, TypeInfo, Debug, Default)]
struct StackingControl{
    //El dueño del contrato
    admin: ActorId;
    //El contrato para el stacking_control
    pool_contract: ActorId;
    //El contrato para los rendimientos
    rewards_contract: ActorId;
    //El contrato para stVara
    st_vara_contrat: ActorId;
    //Recompensas que ya han otorgadas
    rewards_produced: u128;
    //Recompensas por entregar
    rewards_total: ActorId;
    //VARAs en stacking
    in_stacking: u128;
    //Stackers
    stackers: HashMap<ActorId. Stacker>
    //Registro de transacciones
    transactions: HashMap<ActorId, Vec<Transaction<Action>>>
}

//Estado del contrato
static mut STACKING_CONTROL: Option<StackingControl> = None;

//Implementaciones especificas del contrato
impl StackingControl{
    
}

//Inicializacion del contrato
#[no_mangle]
extern "C" fn init(){

}

//Procesa los mensajes entrantes
#[no_mangle]
extern "C" fn handle(){
    let contract_state = unsafe { STACKING_CONTROL.as_ref().unwrap() }
    //Obtiene la accion a realizar
    let action: StackingAction = msg::load().expect("Unable to process action");
    //Obtiene el usuario que requiere realizar la accion
    let user = msg::source();
    //Verifica que el usuario se encuentre en el registro
    if let None = stackers {
        //Si no se encuentra lo registra
        contract_state.stackers.insert(user, Stacker::new());
    }
    let result = match action {
        Stake(amount) => {
            //Se comunica con el contrato de stacking pool para realizar el deposito
            let result: PoolResult = msg::send_for_reply_as(contract_state.pool_contract);
        },
        UnStake(amount) => {

        },
        Withdraw(amount) => {

        },
        GetReward => {

        },
        _ => {

        }
    }
    //Verifica que el usuario se encuentre dentro de los registros
    
}

//Controla las respuestas de mensajes
#[no_mangle]
extern "C" fn handle_reply(){

}