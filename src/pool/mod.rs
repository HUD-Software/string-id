mod slots;
mod slot;
use slots::Slots as Slots;
use slot::Slot as  Slot;

struct Pool {
    slots: [Slots; 256],
}