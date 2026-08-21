# GraphQL

`graphq-schema.json` is GitHub's public GraphQL schema, obtained by
introspection. It is **not** first-party work and is not covered by this
project's licence; it is reproduced here only as a development convenience.

Regenerate it rather than relying on the committed copy - it goes stale, and at
5.4 MB it dominates this repository's size:

```sh
gh api graphql -f query="$(cat introspection.gql)" > graphq-schema.json
```

Consider dropping it from version control entirely and fetching it in a build
or dev step instead.
