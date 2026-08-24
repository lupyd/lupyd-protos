# @lupyd/protos

Protocol Buffers and generated TypeScript definitions for Lupyd.

## Installation

```bash
npm install @lupyd/protos
# or
pnpm add @lupyd/protos
# or
yarn add @lupyd/protos
```

## Usage

### Root Import

```typescript
import { auth, user, post } from '@lupyd/protos';
```

### Subpath Import

```typescript
import { User, FullUser } from '@lupyd/protos/user';
import { Auth, UserTokens } from '@lupyd/protos/auth';
import { FullPost, PostBody } from '@lupyd/protos/post';
```

## Building

```bash
# Build TypeScript bindings
pnpm run build

# Recompile proto files (requires protoc)
pnpm run build-protos
```

## License

MIT
