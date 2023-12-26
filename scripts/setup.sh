#!/bin/bash

echo "Copying generated program IDL and types to src/generated directory."

mkdir -p src/generated

cp target/types/app.ts src/generated/App.ts

printf 'import { Idl } from "@coral-xyz/anchor"\n\nconst APP_IDL: Idl = {' > src/generated/idl.ts
tail -n +2 'target/idl/app.json' >> src/generated/idl.ts
printf '\n\nexport default APP_IDL;' >> src/generated/idl.ts

npx prettier --write src/generated
npx eslint --fix src/generated