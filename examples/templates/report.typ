// Simple report template for md2typ
// Usage: md2typ report.md --template examples/templates/report.typ --output report.pdf

#set page(
  paper: "a4", 
  margin: 2cm,
  header: align(right)[_Technical Report_],
  footer: align(center)[#counter(page).display()]
)

#set text(font: "Liberation Serif", size: 11pt)
#set par(justify: true)
#set heading(numbering: "1.1")

// Custom heading styles
#show heading.where(level: 1): it => [
  #v(1em)
  #text(size: 16pt, weight: "bold")[#it.body]
  #v(0.5em)
  #line(length: 100%)
  #v(0.5em)
]