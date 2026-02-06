# Contributing to rs-stl

There are several ways you can contribute to rs-stl. We expect every member of
our community to be welcoming, helpful and respectful.

Please file an issue if you have any questions.

## Code contributions

Please get familiar with our [project conventions](CONVENTIONS.md). See
[project design](docs/design.md) to get familiar with architecture of library.
Standard github workflow is used to merge code contributions:

1. Fork the repository.
2. Create a new branch on your repository that is named after your contribution.
3. Commit your contribution to that branch.
4. Push your changes to your repository.
5. Create a pull request from Github.

Make sure your changes do not break by running all the tests.

We may ask to commit additional changes during code review process. Once
everything looks good, we merge the pull request.

### Adding an unimplemented algorithm

As rs-stl is currently in development phase, there are several well known standard
algorithms that are not currently implemented in rs-stl. For any such algorithm,
please also include the appropriate tests and optional code example in doc
if it would be significantally helpful for clients to use that algorithm.

If you are unsure if algorithm is standard enough or in any aspect/design of
algorithm, feel free to open a GitHub issue regarding the same.

### Fixing an algorithm having bug

Although our goal is to provide bug-less library, but still as currently library
is in development phase, bugs are possible. Any such bug fixes are highly
appreciated. Always remember to add corresponding tests for the fix.

### Architecture changes

rs-stl is exploring the domain of safe collections using positions. And this
territory has not been explored completely in general and thus it is possible
to tackle the problem with better design. See
[tagged positions](https://github.com/orgs/hylo-lang/discussions/1801) for such
an example we have been exploring in [hylo](https://github.com/hylo-lang/hylo)
land.

Usually any major design change should be preceeded by a discussion. Currently,
we use github issues for any discussions.


### Interoperability with standard library types

Currently, rs-stl supports few standard library types as collections. However,
we surely want to support more and more standard library types (if possible).
Adding such an implementation would be really helpful.

Orthogonally, if any current standard library type that is conceptually a
collection but can't implemented as collection due to some implementation detail,
we might want to discuss better implementation strategy for the same and might
want to provide better implementation from our library. We might also collaborate
with Rust people to see, if we can make it to standard library.

## Non-Code Contributions

Every contributions don't require code. We use GitHub issues for such contributions.

### Feature Request

There are various areas of feature requests:

1. Adding some trait/protocol to collection hierarchy.
   Please make sure that suggested trait have real use-cases that is discovered
   through already existing algorithms and is not invented. Please read
   [From Mathematics to Generic Programming](https://www.fm2gp.com/) for the
   reasoning.
2. Any specific kind of collection/algorithms.

### Fixing the process document

As rs-stl is quite a research project currently, the process document might not
be well written for new-coming contributors. If you feel so, feel free to
contribute to that.

### Ask Questions

Yes, asking questions are also contributions. It helps us understand
real world requirements and ergonomics of our API. Please don't hesitate to
ask simple questions too, as something that might look simple to you might be
really complex for someone else ;)
