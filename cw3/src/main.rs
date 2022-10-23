

fn main() {
/*
 Napisz skrypt zamieniający wartość zmiennej decimalNumber  na liczbę rzemską w zakresie od 1 do 10.
 Wynik zapisz do zmienej romanNumber.
 Jeśli decimalNumber nie mieści sie w zakresie to umieść w romanNumber komunikat: Decimal number is out of range!
 Jeśli decimalNumber jest równe undefined lub null to umieść w romanNumber komunikat: Decimal number is undefined or null!
*/use numerals::roman::Roman;
    Roman::new(1).to_string();
    let decimal_number = 5;
    let roman_number = Roman::from(decimal_number);
    println!("{} = {}", decimal_number, roman_number);
}



