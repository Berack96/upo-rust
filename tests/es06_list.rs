use esercizi::es06_list::LinkedList;

#[test]
fn test_front() {
    let mut list = LinkedList::new();

    assert_eq!(0, list.len());
    assert_eq!(None, list.get_front());
    assert_eq!(None, list.pop_front());
    assert_eq!(None, list.get_front());
    assert_eq!(0, list.len());

    list.push_front(50);
    assert_eq!(1, list.len());
    assert_eq!(Some(50), list.get_front());
    assert_eq!(Some(50), list.pop_front());
    assert_eq!(None, list.get_front());
    assert_eq!(0, list.len());

    list.push_front(50);
    list.push_front(20);
    assert_eq!(2, list.len());
    assert_eq!(Some(20), list.get_front());
    assert_eq!(Some(20), list.pop_front());
    assert_eq!(Some(50), list.get_front());
    assert_eq!(Some(50), list.pop_front());
    assert_eq!(None, list.get_front());
    assert_eq!(0, list.len());

    list.push_front(50);
    list.push_front(20);
    list.push_front(150);
    assert_eq!(3, list.len());
    assert_eq!(Some(150), list.get_front());
    assert_eq!(Some(150), list.pop_front());
    assert_eq!(Some(20), list.get_front());
    assert_eq!(Some(20), list.pop_front());
    assert_eq!(Some(50), list.get_front());
    assert_eq!(Some(50), list.pop_front());
    assert_eq!(None, list.get_front());
    assert_eq!(0, list.len());
}

#[test]
fn test_back() {
    let mut list = LinkedList::new();

    assert_eq!(0, list.len());
    assert_eq!(None, list.get_back());
    assert_eq!(None, list.pop_back());
    assert_eq!(None, list.get_back());
    assert_eq!(0, list.len());

    list.push_back(50);
    assert_eq!(1, list.len());
    assert_eq!(Some(50), list.get_back());
    assert_eq!(Some(50), list.pop_back());
    assert_eq!(None, list.get_back());
    assert_eq!(0, list.len());

    list.push_back(50);
    list.push_back(20);
    assert_eq!(2, list.len());
    assert_eq!(Some(20), list.get_back());
    assert_eq!(Some(20), list.pop_back());
    assert_eq!(Some(50), list.get_back());
    assert_eq!(Some(50), list.pop_back());
    assert_eq!(None, list.get_back());
    assert_eq!(0, list.len());

    list.push_back(50);
    list.push_back(20);
    list.push_back(150);
    assert_eq!(3, list.len());
    assert_eq!(Some(150), list.get_back());
    assert_eq!(Some(150), list.pop_back());
    assert_eq!(Some(20), list.get_back());
    assert_eq!(Some(20), list.pop_back());
    assert_eq!(Some(50), list.get_back());
    assert_eq!(Some(50), list.pop_back());
    assert_eq!(None, list.get_back());
    assert_eq!(0, list.len());
}

#[test]
fn test_mixed() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert_eq!(0, list.len());

    list.push_back(50);
    assert_eq!(Some(50), list.get_back());
    assert_eq!(Some(50), list.get_front());
    assert_eq!(1, list.len());

    list.push_front(20);
    assert_eq!(Some(50), list.get_back());
    assert_eq!(Some(20), list.get_front());
    assert_eq!(2, list.len());

    list.push_front(10);
    assert_eq!(Some(50), list.get_back());
    assert_eq!(Some(10), list.get_front());
    assert_eq!(3, list.len());

    list.push_back(150);
    list.push_back(15);
    list.push_front(0);
    assert_eq!(Some(15), list.get_back());
    assert_eq!(Some(0), list.get_front());
    assert_eq!(6, list.len());

    assert_eq!(Some(15), list.pop_back());
    assert_eq!(5, list.len());
    assert_eq!(Some(0), list.pop_front());
    assert_eq!(4, list.len());
    assert_eq!(Some(150), list.pop_back());
    assert_eq!(3, list.len());
    assert_eq!(Some(50), list.pop_back());
    assert_eq!(2, list.len());
    assert_eq!(Some(10), list.pop_front());
    assert_eq!(1, list.len());
    assert_eq!(Some(20), list.pop_front());
    assert_eq!(0, list.len());

    assert_eq!(None, list.pop_back());
    assert_eq!(None, list.pop_front());

    // creo lista [50, 20, 10, 30, 40]
    list.push_back(10);
    list.push_front(20);
    list.push_back(30);
    list.push_back(40);
    list.push_front(50);

    assert_eq!(Some(50), list.pop_front());
    assert_eq!(Some(20), list.pop_front());
    assert_eq!(Some(10), list.pop_front());
    assert_eq!(Some(30), list.pop_front());
    assert_eq!(Some(40), list.pop_front());
    assert_eq!(None, list.pop_front());

    list.push_back(10);
    list.push_front(20);
    list.push_back(30);
    list.push_back(40);
    list.push_front(50);

    assert_eq!(Some(40), list.pop_back());
    assert_eq!(Some(30), list.pop_back());
    assert_eq!(Some(10), list.pop_back());
    assert_eq!(Some(20), list.pop_back());
    assert_eq!(Some(50), list.pop_back());
    assert_eq!(None, list.pop_back());
}

#[test]
fn test_get() {
    let mut list = LinkedList::new();
    // creo lista [50, 20, 10, 30, 40]
    list.push_back(10);
    list.push_front(20);
    list.push_back(30);
    list.push_back(40);
    list.push_front(50);

    assert_eq!(5, list.len());
    assert_eq!(Some(10), list.get(2));
    assert_eq!(Some(20), list.get(1));
    assert_eq!(Some(30), list.get(3));
    assert_eq!(Some(40), list.get(4));
    assert_eq!(Some(50), list.get(0));
    assert_eq!(None, list.get(5));

    assert_eq!(Some(40), list.get(-1));
    assert_eq!(Some(30), list.get(-2));
    assert_eq!(Some(10), list.get(-3));
    assert_eq!(Some(20), list.get(-4));
    assert_eq!(Some(50), list.get(-5));
    assert_eq!(None, list.get(-6));
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Test(u32);

#[test]
fn test_struct() {
    let mut list = LinkedList::new();

    list.push_back(Test(50));
    assert_eq!(Some(Test(50)), list.pop_front());
    assert_eq!(None, list.pop_front());
    assert_eq!(None, list.pop_back());

    list.push_back(Test(10));
    list.push_back(Test(20));
    list.push_back(Test(30));
    list.push_back(Test(40));
    assert_eq!(Some(Test(10)), list.pop_front());
    assert_eq!(Some(Test(40)), list.pop_back());
    assert_eq!(Some(Test(30)), list.pop_back());
    assert_eq!(Some(Test(20)), list.pop_front());
    assert_eq!(None, list.pop_front());
    assert_eq!(None, list.pop_back());
}
