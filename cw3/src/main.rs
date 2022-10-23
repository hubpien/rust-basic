extern crate numerals;
use numerals::roman::Roman;

fn main() {
/*
 Napisz skrypt zamieniający wartość zmiennej decimalNumber  na liczbę rzemską w zakresie od 1 do 10.
 Wynik zapisz do zmienej romanNumber.
 Jeśli decimalNumber nie mieści sie w zakresie to umieść w romanNumber komunikat: Decimal number is out of range!
 Jeśli decimalNumber jest równe undefined lub null to umieść w romanNumber komunikat: Decimal number is undefined or null!
*/
    let decimal_number = 5;
    use numerals::roman::Roman;
    use numerals::roman::Numeral::{I, V, X};
    let input    = Roman::from(decimal_number);
    let expected = Roman::from(vec![ X, X, V, I, I ]);
    assert_eq!(expected, input);
}

