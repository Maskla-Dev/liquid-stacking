use gmeta::{Metadata};

pub struct StackingControl;

impl Metadata for StackingControl{
    type Init = ();
    type Handle = ();
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}
