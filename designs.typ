#set page(paper: "us-letter")
#set text(font: "Noto Serif")
#set par(justify: true)
#show heading: smallcaps
#show heading.where(level: 1): it => {pagebreak(weak: true) + it}

= Design Principles

The diagram is composed of multiple station intervals. Each station interval contains trains running at different time.

- How to optimize?
- How to only draw the trains that needed to be drawn? Are there any sort of techniques to reduce the complexity?
- How to prevent $O(n)$ and only use $O(1)$ and $O(log n)$ algorithms?

Burr burr burrs. Ding Dong Ji.

= Reducing Complexity

- Storing indicies
- This works very well!


= Features

Features include exporting heatmaps and diagrams.
