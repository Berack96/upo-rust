use super::{
    cell::Cell,
    config::Config,
    entities::{
        Direction::{self, Down, Left, Right, Up},
        Position,
    },
    floor::Floor,
};
use crate::entities::Entity;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    ops::Range,
};

/// Generatore del gioco che può creare dei piani del dungeon.
/// Idealmente questo generatore si comporta come il pattern Factory.
/// Per far si che funzioni ha bisongo di un seed per la generazione del piano
/// verrà utilizzato poi dal piano stesso per eventuali altri calcoli.
/// Inoltre ad esso viene passato una struttura di config che permette
/// di scegliere meglio come poter generare il piano.
pub struct Generator<'a> {
    pub rng: Pcg32,
    pub level: usize,
    config: &'a Config,
    size: usize,
}

impl<'a> Generator<'a> {
    /// Costruttore standard di un generatore, esso avrà tutte le caratteristiche indicate nella configurazione
    pub fn new(floor_seed: u64, floor_level: usize, config: &'a Config) -> Self {
        let min_floor = config.maze_generation.floor_size.clone().next().unwrap();
        let max_room = config.maze_generation.room_size.clone().last().unwrap();
        assert!(min_floor > max_room, "Floor size should be > than room");

        let mut rand_pcg = Pcg32::seed_from_u64(floor_seed);
        let mut floor_size = rand_pcg.gen_range(config.maze_generation.floor_size.clone());
        if floor_size % 2 == 0 {
            floor_size = floor_size.max(2) - 1
        }

        Self {
            rng: rand_pcg,
            level: floor_level,
            size: floor_size,
            config,
        }
    }
    /// Crea un nuovo labirinto a partire dalle configurazioni passate in input.\
    /// Questo metodo creerà un piano avente delle stanze collegate tra di loro tramite dei
    /// corridoi; inoltre in esse verranno inseriti degli effetti.
    pub fn build_floor(mut self) -> Floor {
        let maze_gen = &self.config.maze_generation;
        let room_size = self.config.maze_generation.room_size.clone();
        let mut gen = MazeGenerator::new(self.size, room_size, &mut self.rng);
        let mut grid = gen
            .generate_rooms(maze_gen.room_placing_attempts)
            .generate_labyrinth(maze_gen.straight_percentage)
            .connect_regions()
            .remove_dead_ends(maze_gen.dead_ends)
            .finalize(Cell::Wall, Cell::Empty);

        let index = gen.get_random_room_index();
        let entrance = gen.get_room_ranges(index);
        let index = gen.get_random_room_index();
        let exit = gen.get_room_ranges(index);

        let pos = self.rand_empty_cell_pos(&mut grid, entrance.0, entrance.1);
        grid[pos.0][pos.1] = Cell::Entrance;
        let pos = self.rand_empty_cell_pos(&mut grid, exit.0, exit.1);
        grid[pos.0][pos.1] = Cell::Exit;

        self.rand_place_effects(&mut grid);
        let entities = self.rand_place_entities(&mut grid);

        Floor::new(self.level, self.rng, entities, grid)
    }

    /// Permette di piazzare delle entità in modo casuale nell piano passato.\
    /// Le entità verranno messe solamente sopra celle Empty e non sopvrapposte fra di loro.\
    /// Alla fine verrà restituito un vettore contenente tutte le entità che dovrà poi essere associato
    /// al piano in fase di creazione.
    fn rand_place_entities(&mut self, grid: &mut Vec<Vec<Cell>>) -> Vec<Entity> {
        let entities = ProbVec::new(&self.config.entities, |e| {
            e.floors.contains(&self.level).then(|| (e.priority, e))
        });

        let mut result: Vec<Entity> = vec![];
        for _ in 0..self.config.entities_total {
            let config = entities.sample(&mut self.rng).clone();
            let mut entity = Entity::new(
                config.name.clone(),
                config.health,
                config.attack,
                config.behavior.clone(),
            );

            loop {
                let pos = self.rand_empty_cell_pos(grid, 0..self.size, 0..self.size);
                if !result.iter().any(|e| e.position == pos) {
                    entity.position = pos;
                    result.push(entity);
                    break;
                }
            }
        }
        result
    }
    /// piazza gli effetti della confgurazione in modo casuale su tutto il piano.\
    /// essi vengono piazzati solamente sulle celle Empty
    fn rand_place_effects(&mut self, grid: &mut Vec<Vec<Cell>>) {
        let effects = ProbVec::new(&self.config.effects, |e| {
            e.floors.contains(&self.level).then(|| (e.priority, e))
        });

        for _ in 0..self.config.effects_total {
            let effect = effects.sample(&mut self.rng).effect.clone();
            let cell = Cell::Special(effect);
            let pos = self.rand_empty_cell_pos(grid, 0..self.size, 0..self.size);
            grid[pos.0][pos.1] = cell;
        }
    }
    /// piazza una cella in un punto casuale tra i range inseriti.\
    /// il metodo continua a provare a piazzare la cella finche non trova una cella Empty.
    fn rand_empty_cell_pos(
        &mut self,
        grid: &mut Vec<Vec<Cell>>,
        range_x: Range<usize>,
        range_y: Range<usize>,
    ) -> Position {
        loop {
            let x = self.rng.gen_range(range_x.clone());
            let y = self.rng.gen_range(range_y.clone());
            if let Cell::Empty = grid[x][y] {
                return Position(x, y);
            }
        }
    }
}

pub struct ProbVec<'a, T> {
    prob: Vec<(f32, &'a T)>,
}

impl<'a, T> ProbVec<'a, T> {
    /// Crea una vista del vettore passato in input dopo aver applicato la funzione di filtro.\
    /// Il vettore risultante avrà una tupla contenente l'elemento T e la sua probabilità
    /// di essere scelto fra tutti gli elementi del vettore.\
    /// Quindi la somma di tutte le probabilità sarà 1.0 (floating arithmetic permettendo).\
    /// La funzione passata in input deve restituire un valore che più vicino a 0 è, maggiore la priorità
    /// dell'elemento di essere selezionato.\
    /// L'algoritmo poi penserà a trasformare le priorità in probabilità secondo questa logica:\
    /// A, priorità 1 e B, priorità 2 => A, 0.66 e B, 0.33\
    /// Ciò significa che A ha probabilità doppia rispetto a B di essere scelta.
    pub fn new<F>(original: &'a Vec<T>, filter: F) -> Self
    where
        F: FnMut(&T) -> Option<(u32, &T)>,
    {
        let temp = original.iter().filter_map(filter).collect::<Vec<_>>();
        let max = temp.iter().fold(0, |a, b| a.max(b.0)) + 1;
        let total = temp.iter().map(|(p, _)| (max - *p) as f32).sum::<f32>();
        let mut accum = 0.0;
        let prob = temp
            .into_iter()
            .map(|(p, item)| {
                accum += (max - p) as f32 / total;
                (accum, item)
            })
            .collect();
        Self { prob }
    }

    /// Dato un vettore generato secondo la funzione vec_filter, essa ne prende un valore casuale
    /// utilizzando le probabilità interne del vettore.\
    pub fn sample(&self, rng: &mut impl Rng) -> &'a T {
        let sample = rng.gen_range(0.0..1.0);
        self.prob
            .iter()
            .filter(|(p, _)| *p >= sample)
            .next()
            .unwrap()
            .1
    }
}

/// Utile per la generazione del labirinto.\
/// L'algoritmo per la generazione del labirinto si può trovare ovunque online, ma in generale è:\
/// - Piazza delle stanze a caso nella zona.\
/// - Riempi tutto il resto con un labirinto.\
/// - Fai dei fori nei vari muri per connettere le stanze e il labirinto.\
/// - Rimuovi alcuni dead-end del labirinto e fai dei fori in esso.\
/// \
/// La fonte degli algoritmi la si può trovare all'articolo:
/// https://journal.stuffwithstuff.com/2014/12/21/rooms-and-mazes/
/// E la sua implementazione la si può trovare al link di github:
/// https://github.com/munificent/hauberk/blob/db360d9efa714efb6d937c31953ef849c7394a39/lib/src/content/dungeon.dart#L74
pub struct MazeGenerator<'a> {
    size: usize,
    rooms_size: Range<usize>,
    rng: &'a mut Pcg32,
    rooms: Vec<Room>,
    regions: Vec<Vec<Option<usize>>>,
    current_region: usize,
}

impl Display for MazeGenerator<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = (0..self.size)
            .into_iter()
            .flat_map(|y| {
                (0..self.size)
                    .into_iter()
                    .map(move |x| {
                        if let Some(num) = self.regions[x][y] {
                            format!("{num:2} ")
                        } else {
                            "███".to_string()
                        }
                    })
                    .chain(std::iter::once("\n".to_string()))
            })
            .collect::<String>();
        write!(f, "{}", str)
    }
}

impl<'a> MazeGenerator<'a> {
    /// Crea un nuovo generatore di stanze a partire dai parametri passati.\
    /// *size* è consigliato che sia un numero dispari, altrimenti alcune zone avranno doppi muri.\
    /// *rooms_size* è consigliato un range come numero maggiore al massimo la metà di size.\
    /// Nota che le stanze generate avranno sempre dimensione dispari per poter generare il labirinto correttamente.\
    /// *rng* indica un generatore di numeri casuali ripetibili, in modo da avere risultati consistenti.
    pub fn new(size: usize, rooms_size: Range<usize>, rng: &'a mut Pcg32) -> Self {
        Self {
            size,
            rooms_size,
            rng,
            rooms: vec![],
            regions: vec![vec![None; size]; size],
            current_region: 0,
        }
    }
    /// Crea il labirinto formato da muri e spazi vuoti passati in input.\
    /// I due parametri passati devono implementare il trait Clone, dato che
    /// quando viene creata la matrice, essi verranno messi all'interno di essa.
    pub fn finalize<T: Clone>(&self, wall: T, empty: T) -> Vec<Vec<T>> {
        self.regions
            .iter()
            .map(|col| {
                col.iter()
                    .map(|cell| match cell {
                        Some(_) => empty.clone(),
                        None => wall.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    /// Rimuove tutti i pezzi di labirinto che non vanno da nessuna parte, o che non sono collegati.\
    /// Per fare ciò cerca tutte le celle vuote che hanno una sola cella vuota collegata.
    /// Dopodichè le rimuove e prende quelle rimenenti finchè non ce rimangono più.\
    /// Nel caso si può decidere di lasciare qualche zona che non va a collegarsi da nessuna parte
    /// mettendo un numero > 0 nel cutoff.\
    /// Questo indicherà che nel labirinto ci saranno al massimo N corridioi senza uscita.
    pub fn remove_dead_ends(&mut self, cutoff: u32) -> &mut Self {
        let mut dead_ends = (0..self.size)
            .into_iter()
            .flat_map(|x| {
                (0..self.size)
                    .into_iter()
                    .map(move |y| Position(x, y))
                    .filter(|pos| self.get(pos).is_some())
                    .filter(|pos| self.has_near_none(pos, 3))
            })
            .collect::<VecDeque<_>>();

        while let Some(pos) = dead_ends.pop_front() {
            if dead_ends.len() < cutoff as usize {
                break;
            }

            self.set(&pos, None);
            dead_ends.extend(
                self.get_near(&pos)
                    .filter(|pos| self.get(pos).is_some())
                    .filter(|pos| self.has_near_none(pos, 3)),
            );
        }

        self
    }
    /// Permette di connettere tutte le zone in modo da avere un grafo collegato invece che sparso.\
    /// Il labirinto, quando vengono create le stanze, non avrà i corridoi e le stanze collegate.\
    /// Questa funzione serve per fare proprio quello, ovvero il collegamento fra di essi.\
    /// Il labirinto si può vedere come un grafo nel quale ci sono delle regioni (stanze e corridoi) scollegate
    /// fra di loro, e l'unico modo per metterle assieme è quello di preare degli archi (rompere i muri).\
    pub fn connect_regions(&mut self) -> &mut Self {
        let mut connectors = self.get_regions_connectors();
        let mut merged = MergeSets::new(1, self.current_region);
        let mut keys = connectors.keys().map(|pos| pos.clone()).collect::<Vec<_>>();
        keys.sort(); // for repeatability

        while !merged.has_only_one() {
            let rand_index = self.rng.gen_range(0..keys.len());
            let pos = &keys[rand_index];
            if let Some(regions) = connectors.remove(pos) {
                self.set(pos, Some(0));
                self.current_region += 1;

                merged.merge(regions.into_iter());

                connectors.remove(&Position(pos.0 + 1, pos.1));
                connectors.remove(&Position(pos.0, pos.1 + 1));
                connectors.remove(&Position(pos.0.saturating_sub(1), pos.1));
                connectors.remove(&Position(pos.0, pos.1.saturating_sub(1)));
            }
        }
        self
    }
    /// Permette di ricevere una mappa che contiene tutte le posizioni None del labirinto che hanno
    /// due o più regioni tra le celle vicine.\
    /// Ciò indica che, nel caso in cui vengano messe a Some(_) le regioni adiacenti ora sono collegate formando
    /// una sola grande regione, e quindi collegando parti separate del grafo.\
    /// La lista di punti in cui ciò è possibile contiene tutti i punti esterni delle stanze interne del labirinto.\
    /// Questa operazione è l'equivalente dell'operazione dei grafi al link: https://en.wikipedia.org/wiki/Spanning_tree
    fn get_regions_connectors(&self) -> HashMap<Position, HashSet<usize>> {
        self.rooms
            .iter()
            .flat_map(|room| room.get_bounding_points())
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|pos| self.get(pos).is_none())
            .filter_map(|pos| {
                let regions = self
                    .get_near(&pos)
                    .filter_map(|pos| self.get(&pos))
                    .collect::<HashSet<_>>();

                if regions.len() >= 2 {
                    Some((pos, regions))
                } else {
                    None
                }
            })
            .collect()
    }
    /// Riempie tutti gli spazi vuoti della griglia con un labirinto.\
    /// Questo algoritmo lascerà spazi con muri tra i vari cammini e non cercherà
    /// di connettersi con altri corridoi (per fare ciò esiste il metodo connect_regions).\
    /// Il parametro da passare indica la percentuale (0..=100) di quanto deve continuare ad
    /// andare dritto quando crea il labirinto.\
    /// Con percentuali alte si avranno molti corridoi lunghi, con percentuali basse si avranno
    /// molte svolte.
    pub fn generate_labyrinth(&mut self, mut straight_percentage: u32) -> &mut Self {
        straight_percentage = straight_percentage.min(100); // cap at 100

        for x in (1..self.size).step_by(2) {
            for y in (1..self.size).step_by(2) {
                let pos = Position(x, y);
                if self.get(&pos).is_none() && self.has_near_none(&pos, 4) {
                    self.grow_maze(pos, straight_percentage);
                }
            }
        }
        self
    }
    /// Crea il labirinto nelle zone vuote a partire dalla posizione indicata.\
    /// Questo metodo è messo privato dato che la posizione di partenza deve essere dispari e
    /// deve avere tutti e quattro le celle vicine settate a None.\
    /// L'algoritmo utilizzato è un backtracking iterativo modificato in modo da generare corridioi
    /// un pochino più lunghi e lo si può trovare sulla pagina:\
    /// https://en.wikipedia.org/wiki/Maze_generation_algorithm#Iterative_implementation_(with_stack)\
    /// Il parametro straight_percentage indica quanto "scava" i corridoi del labirinto
    /// senza girare, e quindi creando lunghi segmenti.
    fn grow_maze(&mut self, start: Position, straight_percentage: u32) {
        self.current_region += 1;
        self.set(&start, Some(self.current_region));

        let mut prev_direction = Direction::None;
        let mut cells = vec![];
        cells.push(start);

        while let Some(mut pos) = cells.pop() {
            let directions = self.get_empty_cells_directions(&pos);
            prev_direction = if !directions.is_empty() {
                // Based on how "windy" passages are, try to prefer carving in the same direction.
                let same_direction = self.rng.gen_range(0..=100) < straight_percentage;
                let current = if directions.contains(&prev_direction) && same_direction {
                    prev_direction
                } else {
                    let rand = self.rng.gen_range(0..directions.len());
                    directions[rand]
                };

                // save for back-tracking
                let prev = pos.clone();
                cells.push(prev);

                // move two times
                self.set(current.move_from(&mut pos), Some(self.current_region));
                self.set(current.move_from(&mut pos), Some(self.current_region));
                cells.push(pos);
                current
            } else {
                Direction::None
            }
        }
    }
    /// Ritorna tutte le direzioni da cui ci si può spostare da una cella.\
    /// Questo metodo controlla che dalla posizione *pos* si possa andare in una direzione
    /// almeno per due passi. In caso positivo, la direzione viene inserita nel risultato.\
    /// Questo metodo viene usato esclusivamente da grow_maze
    fn get_empty_cells_directions(&self, pos: &Position) -> Vec<Direction> {
        [Up, Left, Down, Right]
            .into_iter()
            .filter(|dir| {
                let mut pos = pos.clone();
                dir.move_from(&mut pos);
                dir.move_from(&mut pos);
                pos.0 < self.size && pos.1 < self.size && self.has_near_none(&pos, 4)
            })
            .collect()
    }
    /// Aggiunge delle stanze in modo casuale all'interno della rappresentazione del labirinto.\
    /// Questo metodo non controlla altro che le stanze già inserite per evitare di avere collisioni fra di esse.\
    /// Nel caso in cui questo metodo venga chiamato dopo la generazione del labirinto, e le stanze venissero
    /// inserite senza collisioni con quelle precedenti, il labirinto sottostante sarebbe sovrascritto.\
    /// Il parametro attempts indica dopo quanti inserimenti falliti si deve fermare.
    pub fn generate_rooms(&mut self, mut attempts: u32) -> &mut Self {
        while attempts > 0 {
            let room = Room::rand(self.rng, self.size, self.rooms_size.clone());
            if self.rooms.iter().any(|other| room.collide(other)) {
                attempts -= 1;
            } else {
                self.current_region += 1;
                room.get_area_points()
                    .for_each(|p| self.set(&p, Some(self.current_region)));
                self.rooms.push(room);
            }
        }
        self
    }
    /// Ritorna un iteratore di posizioni vicine alla posizione indicata.\
    /// Viene ritornato un iteratore in modo che si possa decidere cosa farlo diventare.\
    /// Nel caso una posizione sia fuori dal campo, essa viene scartata e non
    /// sarà compresa al'interno dell'iterazione.
    fn get_near(&'a self, pos: &'a Position) -> impl Iterator<Item = Position> + 'a {
        [Up, Left, Down, Right]
            .into_iter()
            .map(|dir| *dir.move_from(&mut pos.clone()))
            .filter(|pos| pos.0 < self.size && pos.1 < self.size)
    }
    /// Indica se alla posizione passata la cella ha un tot dei vicini None.\
    /// Se infatti si passasse a total 2, significa che questo metodo restituirà
    /// true solamente se la cella alla posizione pos ha esattamente 2 vicini None.
    fn has_near_none(&self, pos: &Position, total: usize) -> bool {
        let total = total.min(4);
        self.get_near(pos)
            .filter(|pos| self.get(pos).is_none())
            .fold(0, |count, _| count + 1)
            == total
    }
    /// Metodo per l'assegnamento di un valore alla posizione indicata.\
    /// Nel caso si voglia mettere un muro, assegnare None, altrimenti inserire Some(region) per
    /// indicare a quale regione quella cella appartiene.
    fn set(&mut self, pos: &Position, val: Option<usize>) {
        self.regions[pos.0][pos.1] = val;
    }
    /// Permette di prendere il valore contenuto nella cella.\
    /// Nel caso None si indica un muro, mentre in Some(region) si indica la regione quella cella appartiene.
    fn get(&self, pos: &Position) -> Option<usize> {
        self.regions[pos.0][pos.1]
    }
    /// Ritorna un indice a caso fra quelli possibili riguardo le stanze create.
    pub fn get_random_room_index(&mut self) -> usize {
        self.rng.gen_range(0..self.rooms.len())
    }
    /// Ritorna una coppia di ranges che indicano la zona in cui si trova la stanza indicata fra quelle generate.
    pub fn get_room_ranges(&self, index: usize) -> (Range<usize>, Range<usize>) {
        let room = &self.rooms[index.min(self.rooms.len())];
        let x = room.lo.0..(room.hi.0 + 1);
        let y = room.lo.1..(room.hi.1 + 1);
        (x, y)
    }
}

/// Struttura ausiliaria usata per contenere le posizioni.\
/// Vengono implementate alcuni metodi comodi per essi, quali la collisione
/// o la generazione dei punti dei lati.\
/// La stanza viene rappresentata come un rettangolo, la quale area indica l'interno,
/// mentre i lati non hanno dimensione.\
/// I punti quindi salvati sono il minimo e il massimo di un rettangolo ed indicano il
/// punto più in basso da dove inizia l'area e quello più in alto.
#[derive(Clone, Copy, Debug)]
struct Room {
    lo: Position,
    hi: Position,
}
impl Room {
    /// Crea una stanza random a partire da un massimo valore dei punti raggiungibile
    /// e un range che indica il minimo e il massimo della grandezza di una stanza.
    pub fn rand(rng: &mut impl Rng, max: usize, range: Range<usize>) -> Self {
        let x = Self::rand_odd(rng, 0..max);
        let y = Self::rand_odd(rng, 0..max);

        // removing one since the odd + odd = even => odd-1 + odd = odd
        let x_size = Self::rand_odd(rng, range.clone()) - 1;
        let y_size = Self::rand_odd(rng, range) - 1;

        let x_bottom = if x < x_size { 1 } else { x - x_size };
        let y_bottom = if y < y_size { 1 } else { y - y_size };

        let x_top = (x_bottom + x_size).min(max - 2);
        let y_top = (y_bottom + y_size).min(max - 2);

        Self {
            lo: Position(x_bottom, y_bottom),
            hi: Position(x_top, y_top),
        }
    }
    /// Genera tutti i punti di tutti i lati all'esterno della stanza, insomma i punti dei muri.\
    /// Gli unici punti non generati dall'iteratore ritornato sono quelli degli angoli.\
    /// Es. dato lo(1,1) e hi(2,2) => (1,0), (1,3), (2,0), (2,3), (0,1), (3,1), (0,2), (3,2)\
    /// -XXXX-\
    /// -X██X-\
    /// -X██X-\
    /// -XXXX-\
    pub fn get_bounding_points<'a>(&'a self) -> impl Iterator<Item = Position> + 'a {
        let x_range = self.lo.0..=self.hi.0;
        let y_range = self.lo.1..=self.hi.1;

        let lo_x = self.lo.0 - 1;
        let lo_y = self.lo.1 - 1;
        let hi_x = self.hi.0 + 1;
        let hi_y = self.hi.1 + 1;

        let x_range = x_range.flat_map(move |x| vec![Position(x, lo_y), Position(x, hi_y)]);
        let y_range = y_range.flat_map(move |y| vec![Position(lo_x, y), Position(hi_x, y)]);
        x_range.chain(y_range)
    }
    /// Genera tutti i punti all'interno del rettangolo indicato dalla stanza.\
    /// I lati si possono vedere come i muri e l'area come l'interno.\
    /// Cosí facendo, i punti sui lati non verranno generati.
    pub fn get_area_points<'a>(&'a self) -> impl Iterator<Item = Position> + 'a {
        (self.lo.0..=self.hi.0).into_iter().flat_map(|x| {
            (self.lo.1..=self.hi.1)
                .into_iter()
                .map(move |y| Position(x, y))
        })
    }
    /// Indica se la stanza creata è in collisione con un'altra passata in input.\
    /// Più precisamente una collisione avviene se l'area di una stanza si sovrappone con l'altra.
    /// Il codice risultante deriva dal seguente link:\
    /// https://stackoverflow.com/questions/306316/determine-if-two-rectangles-overlap-each-other
    pub fn collide(&self, other: &Self) -> bool {
        self.lo.0 <= other.hi.0
            && self.hi.0 >= other.lo.0
            && self.lo.1 <= other.hi.1
            && self.hi.1 >= other.lo.1
    }
    /// Genera un numero dispari a partire dal range inserito.\
    /// Questo metodo è utile per il piazzamento della stanza in punti dispari in modo che
    /// il labirinto si possa mettere tra i vari muri.
    fn rand_odd(rng: &mut impl Rng, range: Range<usize>) -> usize {
        let mut rand = rng.gen_range(range);
        if rand % 2 == 0 {
            rand = rand.saturating_sub(1).max(1);
        }
        rand
    }
}

/// Struttura usata per unire due o più regioni in modo veloce.\
/// Questo codice è un'implementazione grezza e non ottimizzata di algoritmi UnionFind.\
#[derive(Debug)]
struct MergeSets {
    sets: Vec<usize>,
    current: usize,
    start: usize,
    len: usize,
}
impl MergeSets {
    /// Crea la struttura UnionFind in modo da avere dei sets numerati da start a total.\
    /// In questo modo possono esistere 4 insiemi, ma a partire dal numero 3 => 3,4,5,6
    pub fn new(start: usize, total: usize) -> Self {
        Self {
            sets: (start..=total).into_iter().collect(),
            current: total + 1,
            start,
            len: total - start,
        }
    }
    /// Indica se tutti i sets sono stati uniti oppure no.\
    /// Infatti se sono tutti uniti allora ritorna true, altrimenti false.
    pub fn has_only_one(&self) -> bool {
        self.len == self.sets.len()
    }
    /// Unisce uno o più regioni indicate dall'iteratore.\
    /// Questo metodo ha complessità pari ad O(n).
    pub fn merge(&mut self, regions: impl Iterator<Item = usize>) {
        let regions = regions
            .map(|reg| self.sets[reg - self.start])
            .collect::<HashSet<_>>();
        self.len = self
            .sets
            .iter_mut()
            .filter(|set| regions.contains(set))
            .fold(0, |count, set| {
                *set = self.current;
                count + 1
            });
        self.current += 1;
    }
}
