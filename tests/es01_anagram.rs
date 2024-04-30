use esercizi::es01_anagram::anagrammi;

#[test]
fn test_anagrammi() {
    assert_eq!(anagrammi("ciao", "ciaq"), false);
    assert_eq!(anagrammi("anna", "nana"), true);
    assert_eq!(
        anagrammi(
            "fhgsdlifgdsiulfsdkjhvldshvlidhfksdhlvuidxhljfkshlkseghlif",
            "fiisslfhkgjshsiflvdghdughdhddfiflljhksvklxhhusfildkevllds"
        ),
        true
    );
    assert_eq!(
        anagrammi(
            "fhgsdlifgdsiulfsdkjhvldsqsvlidhfksdhlvuidxhljfkshlkseghlif",
            "fiisslfhkgjshsiflvdghdughdhddfiflljhksvklxhhusfildkevllds"
        ),
        false
    );
    assert_eq!(anagrammi("bububububub", "fsdfgaiholka"), false);
    assert_eq!(anagrammi("baubau", "baubua"), true);
    assert_eq!(anagrammi("baubab", "baubb"), false);
}
