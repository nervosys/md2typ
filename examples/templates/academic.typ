// Simple academic document template for md2typ
// Usage: md2typ document.md --template examples/templates/academic.typ --output paper.pdf

#set page(
  paper: "us-letter", 
  margin: (top: 1in, bottom: 1in, left: 1in, right: 1in)
)

#set text(font: "Times New Roman", size: 12pt, lang: "en")
#set par(justify: true, leading: 0.65em)
#set heading(numbering: "1.")

// Academic paper title page
#align(center)[
  #v(2cm)
  #text(size: 18pt, weight: "bold")[Your Document Title]
  #v(1em)
  #text(size: 12pt)[Generated from Markdown with md2typ]
  #v(3cm)
]

#pagebreak()