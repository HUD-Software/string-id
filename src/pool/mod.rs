mod slots;
mod slot;
use slots::Slots;

struct Pool {
    slots: [Slots; 256],
}