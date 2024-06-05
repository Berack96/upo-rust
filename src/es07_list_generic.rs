#![allow(unused)]

/**
 * Fare una implementazione delle liste doppiamenti linkate come la precedente, ma i metodo get
 * rimuove l’elemento nella posizione n e lo ritorna. Per questa implementazione il tipo generico
 * T non deve implementare Copy.

#[derive(Default)]
struct Node<T> {
    item: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}
impl<T> Node<T> {
    fn new(item: T) -> Self
}

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Default)]
pub struct DoublyPointedList<T> {
    head: Pointer<T>,
    tail: Pointer<T>,
    size: usize,
}
impl<T> DoublyPointedList<T> {
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
use std::{cell::RefCell, mem, rc::Rc};

type Pointer<T> = Option<Rc<RefCell<T>>>;

pub struct LinkedList<T> {
    size: usize,
    head: Pointer<Node<T>>,
    tail: Pointer<Node<T>>,
}

struct Node<T> {
    element: T,
    next: Pointer<Self>,
    prev: Pointer<Self>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }
    pub fn as_memref(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

impl<T> LinkedList<T> {
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
            head.borrow_mut().prev = Some(element.clone());
            element.borrow_mut().next = Some(head);
        } else {
            self.tail = Some(element.clone());
        }

        self.head = Some(element);
        self.size += 1;
    }
    pub fn push_back(&mut self, element: T) {
        let element = Node::new(element).as_memref();
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(element.clone());
            element.borrow_mut().prev = Some(tail);
        } else {
            self.head = Some(element.clone());
        }

        self.tail = Some(element);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.get(0)
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.get(-1)
    }

    pub fn get(&mut self, n: i32) -> Option<T> {
        let index = n + if n < 0 { self.size as i32 } else { 0 };
        if let Some(node) = self.find(index) {
            let temp = node.borrow_mut();
            let prev = temp.prev.clone();
            let next = temp.next.clone();
            mem::drop(temp); // drop the borrow

            match &prev {
                Some(val) => val.borrow_mut().next = next.clone(),
                None => self.head = next.clone(),
            }
            match &next {
                Some(val) => val.borrow_mut().prev = prev.clone(),
                None => self.tail = prev.clone(),
            }

            self.size -= 1;
            Some(Rc::try_unwrap(node).ok().unwrap().into_inner().element)
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
