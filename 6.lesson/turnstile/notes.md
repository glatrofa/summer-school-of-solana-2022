#

```bash
anchor init <program_name> --no-git
```

Move into the new foldere created and type

```bash
anchor build
```

If this error occurs: TypeError: Module "file:///home/glatro/summer-school-of-solana-2022/6.lesson/turnstile/tsconfig.json" needs an import assertion of type "json", then type

```bash
yarn add ts-mocha
```