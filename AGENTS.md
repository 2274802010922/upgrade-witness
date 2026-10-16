# Contributor instructions

- Write new commit titles and bodies in Vietnamese. Conventional Commit prefixes and technical names may remain in English. Do not rewrite old history only to translate it.
- Keep measured results separate from proposed targets. Update the English and Vietnamese README together for user-visible changes.
- Run `npm run typecheck` and `npm test` for implementation changes. Run `npm run demo` when the verification pipeline or fixture changes.
- Preserve the original baseline expectations. Do not update expected outputs just to hide a regression.
- Use original synthetic fixtures and retain all third-party notices. Do not commit `.uw`, credentials, private captures, or `node_modules`.
