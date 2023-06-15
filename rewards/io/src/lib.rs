use gmeta::{Metadata};

pub struct Rewards;

impl Metadata for Rewards{
    type Init = ();
    type Handle = ();
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}