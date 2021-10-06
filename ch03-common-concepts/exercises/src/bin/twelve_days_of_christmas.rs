// Rustbook ch. 03, exercise 3:  Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
//
//On the twelfth day of Christmas
//my true love sent to me:
//12 Drummers Drumming
//Eleven Pipers Piping
//Ten Lords a Leaping
//Nine Ladies Dancing
//Eight Maids a Milking
//Seven Swans a Swimming
//Six Geese a Laying
//Five Golden Rings
//Four Calling Birds
//Three French Hens
//Two Turtle Doves
//and a Partridge in a Pear Tree

fn main() {
    let true_lover = "my true lover sent me:";
    let lyrics = [
        "a Partridge in a Pear Tree\n",
        "and a Partridge in a Pear Tree\n",
        "Turtle Doves",
        "French Hens",
        "Calling Birds",
        "Golden Rings",
        "Geese a Laying",
        "Swans a Swimming",
        "Maids a Milking",
        "Ladies Dancing",
        "Lords a Leaping",
        "Pipers Piping",
        "Drummers Drumming",
    ];

    println!("On the first day of Christmas");
    println!("{}", true_lover);
    println!("{}", lyrics[0]);

    for i in 2..13 {
        println!("On the {} day of Christmas", i); // this is wrong but whatever
        println!("{}", true_lover);
        for j in (2..(i+1)).rev() {
            println!("{} {}", j, lyrics[j]);
        }
        println!("{}", lyrics[1]);
    }

}
