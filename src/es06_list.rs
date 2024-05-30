#![allow(unused)]

/**
 * implementerete una struttura di lista doppiamente linkata.
 * Di seguito ho messo  le dichiarazioni delle strutture dati e le funzioni e i metodi che dovete implementare.
 * Dovete implementare anche dei test usando come tipo di T qualcosa di piu’ complesso di un tipo primitivo,, ad esempio una struttura punto con 2 componenti intere.

#[derive(Default)]
struct Node<T:Copy> {
    item: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}
impl<T:Copy> Node<T> {
    fn new(item: T) -> Self
}

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Default)]
pub struct DoublyPointedList<T:Copy> {
    head: Pointer<T>,
    tail: Pointer<T>,
    size: usize,
}
impl<T:Copy> DoublyPointedList<T> {
    pub fn new() -> Self
    pub fn is_empty(&self) -> bool
    pub fn len(&self) -> usize
    pub fn push_back(&mut self, item: T)
    pub fn push_front(&mut self, item: T)
    pub fn pop_back(&mut self) -> Option<T>
    pub fn pop_front(&mut self) -> Option<T>
    // Se n e' positivo ritornate l'ennesimo elemento dall'inizio
    //della lista mentre se e' negativo lo ritornate dalla coda
    //(-1 e' il primo elemento dalla coda)
    pub fn get(& self, n:i32) -> Option<T>
}
*/
use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};

type Pointer<T> = Option<Rc<RefCell<T>>>;

pub struct LinkedList<T: Copy> {
    size: usize,
    head: Pointer<Node<T>>,
    tail: Pointer<Node<T>>,
}

struct Node<T: Copy> {
    element: T,
    next: Pointer<Node<T>>,
    prev: Pointer<Node<T>>,
}

impl<T: Copy> Node<T> {
    pub fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }
    pub fn get(node: &Rc<RefCell<Node<T>>>) -> RefMut<Node<T>> {
        node.as_ref().borrow_mut()
    }
    pub fn as_memref(self) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(self))
    }
}

impl<T: Copy> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push_front(&mut self, element: T) {
        let element = Node::new(element).as_memref();
        if let Some(head) = self.head.take() {
            Node::get(&head).prev = Some(element.clone());
            Node::get(&element).next = Some(head);
        } else {
            self.tail = Some(element.clone());
        }

        self.head = Some(element);
        self.size += 1;
    }
    pub fn push_back(&mut self, element: T) {
        let element = Node::new(element).as_memref();
        if let Some(tail) = self.tail.take() {
            Node::get(&tail).next = Some(element.clone());
            Node::get(&element).prev = Some(tail);
        } else {
            self.head = Some(element.clone());
        }

        self.tail = Some(element);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            let node = Node::get(&node);
            self.head = node.next.clone();

            match self.head.as_ref() {
                Some(other) => Node::get(other).prev = None,
                _ => self.tail = None,
            }

            self.size -= 1;
            return Some(node.element);
        }
        None
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(node) = self.tail.take() {
            let node = Node::get(&node);
            self.tail = node.prev.clone();

            match self.tail.as_ref() {
                Some(other) => Node::get(other).next = None,
                _ => self.head = None,
            }

            self.size -= 1;
            return Some(node.element);
        }
        None
    }

    pub fn get_front(&self) -> Option<T> {
        self.head.clone().and_then(|h| Some(Node::get(&h).element))
    }
    pub fn get_back(&self) -> Option<T> {
        self.tail.clone().and_then(|t| Some(Node::get(&t).element))
    }

    pub fn get(&self, n: i32) -> Option<T> {
        let index = n + if n < 0 { self.size as i32 } else { 0 };
        if let Some(node) = self.find(index) {
            Some(Node::get(&node).element)
        } else {
            None
        }
    }

    fn find(&self, index: i32) -> Pointer<Node<T>> {
        if index < 0 || index as usize >= self.size {
            return None;
        }

        let index = index as usize;
        let delta_back = (self.size - index) - 1_usize;
        let from_head = index <= delta_back;

        let node = if from_head { &self.head } else { &self.tail };
        let mut node = node.clone();
        let mut index = if from_head { index } else { delta_back };

        while let Some(curr) = node {
            if index == 0 {
                return Some(curr.clone());
            }

            let curr = curr.as_ref().borrow();
            let curr = if from_head { &curr.next } else { &curr.prev };
            node = curr.clone();
            index -= 1;
        }
        None
    }
}
