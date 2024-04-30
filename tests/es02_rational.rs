use esercizi::es02_rational::Rational;

#[test]
fn test_razionali() {
    assert_eq!(
        Rational::new(2, 3).multiplication(&Rational::new(3, 2)),
        &Rational::new(1, 1)
    );
    assert_eq!(
        Rational::new(4, 3).multiplication(&Rational::new(5, 7)),
        &Rational::new(20, 21)
    );
    assert_eq!(
        Rational::new(-3, 2).multiplication(&Rational::new(7, 4)),
        &Rational::new(-21, 8)
    );
    assert_eq!(
        Rational::new(-3, 2).multiplication(&Rational::new(14, 14)),
        &Rational::new(-3, 2)
    );
    assert_eq!(
        Rational::new(2, 3).addition(&Rational::new(3, 2)),
        &Rational::new(13, 6)
    );
    assert_eq!(
        Rational::new(5, 3).addition(&Rational::new(5, 2)),
        &Rational::new(25, 6)
    );
    assert_eq!(
        Rational::new(-3, 16).addition(&Rational::new(5, -4)),
        &Rational::new(-23, 16)
    );
    assert_eq!(
        Rational::from(100).addition(&Rational::from(47)),
        &Rational::from(147)
    );
    assert_eq!(
        Rational::new(23, 12)
            .addition(&Rational::new(24, 15))
            .addition(&Rational::new(23, 24))
            .addition(&Rational::new(-437, 120)),
        &Rational::new(5, 6)
    );
    assert_eq!(Rational::new(-2, 4), Rational::new(-1, 2));
    assert_eq!(Rational::new(4, -2), Rational::new(-2, 1))
}
