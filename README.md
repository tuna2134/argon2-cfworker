# argon2 for cf-worker
このプロジェクトはcloudflareがargon2をサポートしないことで、不便を強いられました。そのため、wasmを作ることで解決を図るつもりです。

## Example
```ts
// Before you must install package.
//
// ex: pnpm add argon2-cfworker
import { argon2id } from "argon2-cfworker";

const hash_password = argon2id("password", "salt");
console.log(hash_password);
```