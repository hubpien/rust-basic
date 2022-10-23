/**
* Napisz skrypt, który oblicza promień koła na podstawie pola powierzchni w zmiennej area
* Wynik zapisz do zmiennej łańuchowej radius z dwoma miejscami po przecinku.
*/
fn main() {
    let area: f32 = 1.0;
    let radius: f32;
    let pi: f32 = 3.14;
    radius = (area / pi) as f32;
    println!("Promień koła wynosi: {}", radius);
}


