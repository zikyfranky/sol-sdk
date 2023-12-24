#!/bin/bash

echo "Copying generated program IDL and types to src/generated directory."

cp target/types/skwizz.ts src/generated/Skwizz.ts

printf 'import { Idl } from "@coral-xyz/anchor"\n\nconst SKWIZZ_IDL: Idl = {' > src/generated/idl.ts
tail -n +2 'target/idl/skwizz.json' >> src/generated/idl.ts
printf '\n\nexport default SKWIZZ_IDL;' >> src/generated/idl.ts

npx prettier --write src/generated
npx eslint --fix src/generated