use super::{entities::{Action, Direction, Entity}, floor::Floor};
use dyn_clone::{clone_trait_object, DynClone};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Rappresentazione di una cella di spazio.\
/// Essa ha diversi valori in base a cosa si può fare o meno su di essa.
/// Nel caso in cui passi sopra una entià esiste un metodo entity_over che
/// gestisce le varie casistiche.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Cell {
    Entance,
    Exit,
    Special(Box<dyn Effect>),
    Wall,
    Empty,
}

impl Cell {
    /// Data una entità che passa sopra questa cella di spazio
    /// modifica la posizione e la fa tornare indietro nel caso sia un muro,
    /// nel caso di una cella speciale, applica l'effetto all'entità,
    /// e i tutti gli altri casi non fa nulla.\
    /// Il movimento tra piani tramite Exit e Entrance non è gestito in questa funzione
    /// data la complessità di muovere l'entità.
    pub fn entity_over(&mut self, entity: &mut Entity) {
        match self {
            Cell::Special(effect) => {
                entity.add_effect(effect.clone());
                if !effect.is_persistent() {
                    *self = Cell::Empty
                }
            }
            Cell::Wall => {
                entity.direction.invert();
                entity.direction.move_from(&mut entity.position);
            }
            _ => (),
        }
    }
    /// Restituisce la rappresentazione della cella in formato char, in questo modo
    /// può essere utilizzata per vedere il valore e mostrarlo a terminale.
    pub fn as_char(&self) -> char {
        match self {
            Cell::Entance => ' ',
            Cell::Exit => '¤',
            Cell::Special(_) => '§',
            Cell::Wall => '█',
            Cell::Empty => ' ',
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

/// Trait che permette di implementare un effetto speciale di una
/// cella di spazio.\
/// Il trait è taggato con typetag in modo che possa essere utilizzato
/// nella serializzazione e deserializzazione di serde.
/// Esso permette di trasformare le implementazioni di Effect in una
/// spiecie di Enum senza il bisogno di farlo manualmente.\
/// Quello che viene richiesto è che, nell'implementazione di una
/// struttura concreta di questo trait, venga messo sopra impl X for Effect:\
/// #\[typetag::serde\]\
/// \
/// In questo modo si possono creare molteplici effetti che implementano
/// questo trait senza il bisogno di avere un Enum con essi
#[typetag::serde(tag = "type")]
pub trait Effect: DynClone + core::fmt::Debug {
    /// Indica se l'effetto rimane nel terreno dopo la sua applicazione ad una entità.\
    /// Nel caso di true, l'effetto non verrà rimosso dal terreno,
    /// eltrimenti la cella dove si trova questo effetto diventerà Empty
    fn is_persistent(&self) -> bool;
    /// Applica l'effetto ad una entità.\
    /// L'effetto può essere di tutto a partire da un danno a qualcosa di più
    /// elaborato come una trappola di nemici.\
    /// Tramite l'entità si può anche accedere al piano dove si trova per
    /// poter modificare eventualmente qualcosa.
    fn apply_to(&self, entity: &mut Entity, floor: &mut Floor);
}
clone_trait_object!(Effect);

/// Permette di dare un danno istantaneo a qualunque entità ci passi sopra.\
/// Una volta utilizzato verrà rimosso dal piano.\
/// Nel caso in cui il danno sia negativo, l'entità verrà curata
/// (sempre che la sua vita sia un valore positivo e non negativo)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstantDamage(pub i32);
#[typetag::serde]
impl Effect for InstantDamage {
    fn is_persistent(&self) -> bool {
        false
    }
    fn apply_to(&self, entity: &mut Entity, _floor: &mut Floor) {
        entity.apply_damage(self.0);
    }
}

/// Permettere di infliggere lo stato di confuzione ad una entità.\
/// Esso ignora il successivo comando che verrà impartito all'entità
/// con una probabilità del 50% e inserirà un movimento in una direzione casuale.\
/// Come parametro si può passare per quanti turni l'effetto dura
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Confusion(pub u8);
#[typetag::serde]
impl Effect for Confusion {
    fn is_persistent(&self) -> bool {
        true
    }
    fn apply_to(&self, entity: &mut Entity, floor: &mut Floor) {
        if self.0 > 0 {
            let rng = floor.get_rng();
            if rng.gen_bool(0.5) {
                let random_direction = Direction::random(rng);
                entity.buffer = Action::Move(random_direction);
            }
            entity.add_effect(Box::new(Self(self.0 - 1)));
        }
    }
}

/// Permette di infliggere un danno nel tempo.\
/// Similmente a InstantDamage, se il danno è negativo allora il personaggio verrà curato,
/// sempre a patto che la sua vita sia un valore positivo.\
/// L'effetto dura un determinato numero di turni.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TurnBasedDamage {
    time: u8,
    damage: i32,
}
#[typetag::serde]
impl Effect for TurnBasedDamage {
    fn is_persistent(&self) -> bool {
        false
    }
    fn apply_to(&self, entity: &mut Entity, _floor: &mut Floor) {
        if self.time > 0 {
            entity.apply_damage(self.damage);
            entity.add_effect(Box::new(Self {
                time: self.time - 1,
                damage: self.damage,
            }));
        }
    }
}
