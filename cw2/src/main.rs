fn main() {
/*
 Napisz skrypt, który w łańcuchu triangle zawiera ciąg znaków '#' i '\n' tworzących kształ trójkąta o wysokości
 w zmiennej height. Po wyświetleniu na konsoli powinien zostać wyświetlony poniższy wzór liczba wierzy powinna odpowiadać
 zmiennej height:
 #
 ##
 ###
 ####
 #####
*/
let height = 5;
let mut triangle = " ".to_string();
/*
Wpisz kod zadania w miejscu tego komentarza.
*/

    for _i in 1..height+1 {
            triangle = triangle + "#";
            print!("{}", triangle);
       
        println!("");
    }
}

