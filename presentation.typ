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

= What are patterns?
== What are Programming Patterns?

#slide(align: (center))[
#let box(hed, col: white, body) = {
  rect(radius: 4pt, fill: col, width: 180pt, height: 150pt, [#hed\ #text(size: 15pt)[#body]])}

#grid(
  columns: (1fr, 1fr, 1fr),
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
  text(size: 11pt)[Patterns = Reusable solutions that make programming easier and more reliable])
]

== Different types of patterns?

#slide(align: center)[
  #image("patterntyps.svg")
]

= Iterator pattern
== What is it and how does it work?
#slide()[
// The iterator pattern delivers an easy and performant way to iterate over a collection of objects
#grid(  
  columns: (1fr, 1fr, 1fr),
  rows: (auto),
  gutter: 5pt,

figure(
  text()[Problem
  #image("solution1.png", width: 100%)],
  caption: [#link("https://refactoring.guru/images/patterns/diagrams/iterator/solution1.png")[Quelle]],)
,
text()[]
,

figure(
  text()[Structure
  #image("structure.png", width: 100%)],
  caption: [#link("https://refactoring.guru/images/patterns/diagrams/iterator/structure.png")[Quelle]],)
,

)
]

== Why use it?

#slide(align: center)[
  #image("wehrToUse.svg", width: 80%)
]

#focus-slide[
  Let us (Lettuce) 🥬 begin 
]