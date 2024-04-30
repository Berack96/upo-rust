use esercizi::es02_rational::Rational;

#[test]
fn test_rational_traits() {
    assert_eq!(
        Rational::new(2, 3) * Rational::new(3, 2),
        Rational::new(1, 1)
    );
    assert_eq!(
        Rational::new(4, 3) * Rational::new(5, 7),
        Rational::new(20, 21)
    );
    assert_eq!(
        Rational::new(-3, 2) * Rational::new(7, 4),
        Rational::new(-21, 8)
    );
    assert_eq!(
        Rational::new(-3, 2) * Rational::new(14, 14),
        Rational::new(-3, 2)
    );
    assert_eq!(Rational::new(2, 3) * 1, Rational::new(2, 3));
    assert_eq!(Rational::new(2, 3) * (-5), Rational::new(-10, 3));
    assert_eq!(Rational::new(2, 3) * 10, Rational::new(20, 3));

    assert_eq!(
        Rational::new(2, 3) + Rational::new(3, 2),
        Rational::new(13, 6)
    );
    assert_eq!(
        Rational::new(5, 3) + Rational::new(5, 2),
        Rational::new(25, 6)
    );
    assert_eq!(
        Rational::new(-3, 16) + Rational::new(5, -4),
        Rational::new(-23, 16)
    );
    assert_eq!(
        Rational::from(100) + Rational::from(47),
        Rational::from(147)
    );
    assert_eq!(
        Rational::new(23, 12)
            + Rational::new(24, 15)
            + Rational::new(23, 24)
            + Rational::new(-437, 120),
        Rational::new(5, 6)
    );
    assert_eq!(Rational::new(-3, 2) + 0, Rational::new(-3, 2));
    assert_eq!(Rational::new(-3, 2) + 1, Rational::new(-1, 2));
    assert_eq!(Rational::new(-3, 2) + 5, Rational::new(7, 2));
    assert_eq!(Rational::new(-3, 2) + (-1), Rational::new(-5, 2));
}
