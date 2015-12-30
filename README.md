# Warning
This is heavily outdated. This was started before Rust was finalized, and other projects came up. I would like to make a GUI library in Rust one day, but this will need to be reworked significantly, and not much was ever completed.

# Asterias Toolkit
Asterias is a graphical toolkit that is intended to be both easy to use and
easily extended at the SDL2 layer, so it is both high-level and low-level. It is
named after the Asterias Seamount, which is an underwater mountain -- both high
and low.

## Design
Asterias is designed to be modular and extensible.

\#0 Initialize state (once) ->  
\#1 For each UI widget, call render function -> \#2 Present Frame -> \#3 Check for events -> \#4 Asynchronously call event handlers -> (Loop back to \#1)
