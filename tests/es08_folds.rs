use esercizi::es08_folds::{num_vocali_struct, num_vocali_tuple, NumVocali, TuplaVocali};

#[test]
fn test_folds() {
    let a = String::from(" Ciao Paola come stai ? Ok . Tu John come stai ? Ok ");
    assert_eq!(TuplaVocali::new(5, 2, 3, 7, 1), num_vocali_tuple(&a));
    assert_eq!(NumVocali::new(5, 2, 3, 7, 1), num_vocali_struct(&a));
}
