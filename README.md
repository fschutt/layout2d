# layout2d

Preliminary, GridBag-like 2D layout engine (no constraint solving, only direct solving)

May be replaced by a better system in the future. For now, you can construct UIs with it, in order 
to solve Rusts horrible GUI situations.

layout2d is an immediate GUI method, activated whenever there is an event. Caching, etc. help with
keeping the performance down to an acceptable level. Layout is usually done in nanoseconds, rendering
is more of a problem.

Renderer should be seperated from layout later on and support multiple backends. This is not yet
the case.

Note: As I investigated, there are better solutions to making a scalable GUI. There are: Simplex method,
Cassowary and the inner point method (all used to maximize a system of linear equalities or inequalities, 
given an objective function). These methods should be used for constructing a UI properly, however, they
take time to implement.

This a preliminary UI system. Please don't use it yet.
