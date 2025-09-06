// Basic letter template for md2typ
// Usage: md2typ letter.md --template examples/templates/letter.typ --output letter.pdf

#set page(
  paper: "us-letter",
  margin: (top: 1in, bottom: 1in, left: 1.25in, right: 1.25in)
)

#set text(font: "Times New Roman", size: 11pt)
#set par(leading: 0.65em)

// Letter header
#align(right)[
  #datetime.today().display()
]

#v(2em)