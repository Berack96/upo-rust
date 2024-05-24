use rand::SeedableRng;
use rand_pcg::Pcg32;
use rogue_lib::{
    cell::{Cell, Effect, InstantDamage, TurnBasedDamage},
    entities::{Action, Direction, Entity, Immovable, Position},
    floor::Floor,
};

/*******************************************************/
/* Funzioni semplici per inizializzazione di strutture */
/*******************************************************/
fn get_basic_entity() -> Entity {
    Entity::new("name".to_string(), 100, 10, Box::new(Immovable))
}

fn get_basic_floor() -> Floor {
    let rng = Pcg32::seed_from_u64(0);
    let grid = vec![vec![Cell::Empty; 20]; 20];
    Floor::new(0, rng, vec![], grid)
}

/*******************************************************/
/* I tests iniziano da qui in poi                      */
/*******************************************************/

#[test]
fn test_cell_basic() {
    let mut entity = get_basic_entity();

    // random ones
    entity.direction = Direction::Up;
    entity.position = Position(10, 10);

    Cell::Empty.entity_over(&mut entity);
    assert_eq!(entity.position, Position(10, 10));
    assert_eq!(entity.direction, Direction::Up);

    Cell::Entance.entity_over(&mut entity);
    assert_eq!(entity.position, Position(10, 10));
    assert_eq!(entity.direction, Direction::Up);

    Cell::Exit.entity_over(&mut entity);
    assert_eq!(entity.position, Position(10, 10));
    assert_eq!(entity.direction, Direction::Up);

    Cell::Wall.entity_over(&mut entity);
    assert_eq!(entity.position, Position(10, 9));
    assert_eq!(entity.direction, Direction::Down);

    assert_eq!(Cell::Empty.as_char(), ' ');
    assert_eq!(Cell::Entance.as_char(), ' ');
    assert_eq!(Cell::Exit.as_char(), '¤');
    assert_eq!(Cell::Wall.as_char(), '█');
}

#[test]
fn test_cell_trait_effect_instant_damage() {
    let cell = InstantDamage(10);
    let mut floor = get_basic_floor();
    let mut entity = get_basic_entity();

    let health = entity.get_health();
    cell.apply_to(&mut entity, &mut floor);
    assert_eq!(entity.get_health(), health - 10);

    let cell = InstantDamage(-10);
    cell.apply_to(&mut entity, &mut floor);
    assert_eq!(entity.get_health(), health);
}

#[test]
fn test_directions() {
    let mut dir = Direction::Up;

    dir.invert();
    assert_eq!(dir, Direction::Down);
    dir.invert();
    assert_eq!(dir, Direction::Up);

    dir = Direction::Left;

    dir.invert();
    assert_eq!(dir, Direction::Right);
    dir.invert();
    assert_eq!(dir, Direction::Left);

    dir = Direction::None;
    assert_eq!(dir, Direction::None);

    let mut pos = Position(10, 10);
    assert_eq!(Direction::Up.move_from(&mut pos), &Position(10, 11));
    assert_eq!(Direction::Down.move_from(&mut pos), &Position(10, 10));
    assert_eq!(Direction::Left.move_from(&mut pos), &Position(9, 10));
    assert_eq!(Direction::Right.move_from(&mut pos), &Position(10, 10));
    assert_eq!(Direction::None.move_from(&mut pos), &Position(10, 10));

    assert_eq!(Direction::Up.as_char(), '▲');
    assert_eq!(Direction::Down.as_char(), '▼');
    assert_eq!(Direction::Left.as_char(), '◄');
    assert_eq!(Direction::Right.as_char(), '►');
    assert_eq!(Direction::None.as_char(), '■');
}

#[test]
fn test_entity_basic() {
    let entity = get_basic_entity();
    assert!(entity.is_alive());
    assert!(matches!(entity.buffer, Action::DoNothing));
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 100);
    assert_eq!(entity.get_name(), &"name".to_string());
    assert_eq!(entity.direction, Direction::None);
}

#[test]
fn test_entity_basic_damage() {
    let mut entity = get_basic_entity();

    entity.apply_damage(10);
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 90);
    assert!(entity.is_alive());

    entity.apply_damage(-10);
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 100);
    assert!(entity.is_alive());

    entity.apply_damage(90);
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 10);
    assert!(entity.is_alive());

    entity.apply_damage(-100);
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 100);
    assert!(entity.is_alive());

    entity.apply_damage(120);
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 0);
    assert!(!entity.is_alive());

    entity.apply_damage(10);
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 0);
    assert!(!entity.is_alive());

    entity.apply_damage(-500);
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 100);
    assert!(entity.is_alive());
}

#[test]
fn test_entity_basic_effects() {
    let mut floor = get_basic_floor();
    let mut entity = get_basic_entity();
    assert!(matches!(entity.get_effects().next(), None));

    entity.add_effect(Box::new(InstantDamage(10)));
    let mut iter = entity.get_effects();
    assert!(matches!(iter.next(), Some(_)));
    assert!(matches!(iter.next(), None));
    std::mem::drop(iter);

    let mut entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 90);
    assert!(matches!(entity.get_effects().next(), None));

    entity.add_effect(Box::new(InstantDamage(10)));
    entity.add_effect(Box::new(TurnBasedDamage::new(2, 10)));

    let mut iter = entity.get_effects();
    assert!(matches!(iter.next(), Some(_)));
    assert!(matches!(iter.next(), Some(_)));
    assert!(matches!(iter.next(), None));
    std::mem::drop(iter);

    let entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 70);
    let mut iter = entity.get_effects();
    assert!(matches!(iter.next(), Some(_)));
    assert!(matches!(iter.next(), None));
    std::mem::drop(iter);

    let mut entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.get_health_max(), 100);
    assert_eq!(entity.get_health(), 60);
    assert!(matches!(entity.get_effects().next(), None));

    entity.add_effect(Box::new(InstantDamage(100)));
    let entity = entity.update(&mut floor);
    assert!(matches!(entity, None));
}

#[test]
fn test_entity_basic_action() {
    let mut floor = get_basic_floor();
    let mut entity = get_basic_entity();
    entity.position = Position(10, 10);
    entity.buffer = Action::Move(Direction::Up);

    let mut entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.position, Position(10, 11));
    assert_eq!(entity.direction, Direction::Up);
    assert!(matches!(entity.buffer, Action::DoNothing));

    entity.buffer = Action::Move(Direction::Up);
    let mut entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.position, Position(10, 12));
    assert_eq!(entity.direction, Direction::Up);
    assert!(matches!(entity.buffer, Action::DoNothing));

    entity.buffer = Action::Move(Direction::Left);
    let mut entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.position, Position(9, 12));
    assert_eq!(entity.direction, Direction::Left);
    assert!(matches!(entity.buffer, Action::DoNothing));

    entity.buffer = Action::Move(Direction::Left);
    let mut entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.position, Position(8, 12));
    assert_eq!(entity.direction, Direction::Left);
    assert!(matches!(entity.buffer, Action::DoNothing));

    entity.buffer = Action::Move(Direction::Down);
    let mut entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.position, Position(8, 11));
    assert_eq!(entity.direction, Direction::Down);
    assert!(matches!(entity.buffer, Action::DoNothing));

    entity.buffer = Action::Move(Direction::Right);
    let mut entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.position, Position(9, 11));
    assert_eq!(entity.direction, Direction::Right);
    assert!(matches!(entity.buffer, Action::DoNothing));

    entity.buffer = Action::Move(Direction::None);
    let mut entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.position, Position(9, 11));
    assert_eq!(entity.direction, Direction::None);
    assert!(matches!(entity.buffer, Action::DoNothing));

    entity.buffer = Action::DoNothing;
    let entity = entity.update(&mut floor).unwrap();
    assert_eq!(entity.position, Position(9, 11));
    assert_eq!(entity.direction, Direction::None);
    assert!(matches!(entity.buffer, Action::DoNothing));
}

#[test]
fn test_generator_priority() {
    let mut vec = vec![(1_u32, &"a"), (3, &"b"), (2, &"c")].into_iter();
    let vec1 = vec!["", "", ""];
    let vec = rogue_lib::generator::vec_filter(&vec1, |_| vec.next());
    let mut sum: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();
    let mut rng = <rand_pcg::Pcg32 as rand::SeedableRng>::seed_from_u64(0);
    let tot = 600000;
    for _ in 0..tot {
        let sample = rogue_lib::generator::vec_get_sample(&vec, &mut rng);
        let val = sum.entry(*sample).or_default();
        *val += 1;
    }

    // deve essere ~circa a questo valore (per questo il round)
    assert_eq!(
        (*sum.get("a").unwrap() as f32 / (tot as f32 / 6.0)).round(),
        3.0
    );
    assert_eq!(
        (*sum.get("b").unwrap() as f32 / (tot as f32 / 6.0)).round(),
        1.0
    );
    assert_eq!(
        (*sum.get("c").unwrap() as f32 / (tot as f32 / 6.0)).round(),
        2.0
    );
}
