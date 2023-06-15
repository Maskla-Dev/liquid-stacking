#![no_std]
use gstd::{prelude::*, ActorId, msg, Encode, Decode, TypeInfo, Debug, Default};
use hashbrown::HashMap;
use Account;

#[derive(Encode, Decode, TypeInfo, Debug, Default)]
pub struct LiquidityStackingPool{
    admin: ActorId,
    accounts: HashMap<ActorId, Account>,
    current_varas: u128,
    current_st_varas: u128,
    min_deposit: u128,
}

static mut LIQUIDITY_STACKING_POOL: Option<LiquidityStackingPool> = None;

impl LiquidityStackingPool{
    //Create a new LiquidityStackingPool
    fn new(admin: ActorId) -> Self{
        Self{
            admin,
            accounts: HashMap::new(),
            min_deposit: 10,
            current_varas: 0,
            current_st_varas: 0,
        }
    }
    //Withdraw VARAS from the pool
    pub fn withdraw_unstaked(&mut self, amount: u128){
        
    }
}

#[no_mangle]

extern "C" fn init(){

}

extern "C" fn handle(){

}

extern "C" fn State(){

}