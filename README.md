# Asterias Toolkit
Asterias is a graphical toolkit that is intended to be both easy to use and
easily extended at the SDL2 layer, so it is both high-level and low-level. It is
named after the Asterias Seamount, which is an underwater mountain -- both high
and low.

# Design
Asterias is designed to be modular and extensible.

Initialize state -> For each UI widget, call render function -> Present Frame -
                |                                                              |
                 -  Asynchronously call event handlers  <-  Check for events <-

