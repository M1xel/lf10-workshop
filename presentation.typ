#import "@preview/touying:0.6.1": *
#import themes.university: *

// Code blocks
#import "@preview/codly:1.3.0": *
#import "@preview/codly-languages:0.1.1": *
#show: codly-init.with()
#codly(languages: codly-languages)

// Define common variables
#let presentation-title = "Iterator Pattern"
#let presentation-subtitle = "LF10 Pattern Workshops"
#let institution-name = "Itch BS14"
#let author-names = "Mika Bomm, Jan-Henrik Ammer"
#let authors-formatted = ([Mika \ Bomm], [Jan-Henrik \ Ammer])

#import "@preview/numbly:0.1.0": numbly

#show: university-theme.with(
  aspect-ratio: "16-9",
  config-info(
    title: presentation-title,
    subtitle: presentation-subtitle,
    author: author-names,
    date: datetime.today(),
    institution: institution-name,
    logo: image("bs14.png", width: 2cm, alt: "bs14 logo"),
  ),
)

#set heading(numbering: numbly("{1}.", default: "1.1"))

#title-slide(
    title: presentation-title,
    subtitle: presentation-subtitle,
    institution: institution-name,
    authors: authors-formatted
)

= What are patterns ?
== What are Programming Patterns?

#slide()[
#set align()
#let box(hed, col: white,  body)= {
  rect(radius: 4pt,fill: col, width: 200pt, height: 180pt, [#hed\ #text(size: 15pt)[#body]])}

   \ \
#grid(

  columns: (1fr , 1fr, 1fr),
  rows: (auto),
  gutter: 5pt,

box("Problem", col: red)[*Repeating coding
    challenges that many
    developers face*
    \
- Hard to maintain code
- Reinventing the wheel],

box("Pattern", col: aqua)[
  *Proven solution template that works*
- Reusable blueprint
- Best practices
- Tested by many devs],


box("Benefits", col: green)[#list(marker: sym.checkmark.heavy,
  [Better code quality],
  [Easier maintenance],
  [Team communication],
  [Faster development],
  [Proven solutions])],
)

#block(
  fill: teal,
  inset: 8pt,
  radius: 10pt,
  text(size: 11pt)[Patters = Reusable solutions that make ro rammin easier and more reliable])

][

]
== difrent typs of paterns ?
#slide()[
  t
]


#focus-slide[
  Let us (Lettuce) 🥬 begin
]

== Prerequisites
From now on this is the Code we are sharing \
with every example we provide
#context {
    set text(size: text.size / (1.2))
    ```rust
    fn main() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    }
    ```
}

== What can we do with the Iterators

=== Basic Iterations

#context {
    set text(size: text.size / (1.2))
    ```rust
    // easy
    println!("{:?}", numbers); //outputs: 1 2 3 4 5 6 7 8 9 10
    // complex
    let output: String = numbers.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", output); //outputs: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    ```
}

=== Transformations
#context {
    set text(size: text.size / (1.2))
    ```rust
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);
    //outputs: [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
    ```
}

=== Filtering
#context {
    set text(size: text.size / (1.2))
    ```rust
    let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).copied().collect();
    println!("{:?}", evens);
    //outputs: [2, 4, 6, 8, 10]
    ```
}

=== Aggregating
#context {
    set text(size: text.size / (1.2))
    ```rust
    let sum = numbers.iter().fold(0, |total, &x| total + x);
    println!("{}", sum);
    //outputs: 55
    ```
}

#matrix-slide[
  left
][
  middle
][
  right
]

#matrix-slide(columns: 1)[
  top
][
  bottom
]

#matrix-slide(columns: (1fr, 2fr, 1fr), ..(lorem(8),) * 9)