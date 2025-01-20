FROM node:18.17.0

# Create app directory
WORKDIR /usr/src/app

# Install pnpm
RUN npm install -g pnpm

# Install dependencies
COPY package.json ./
COPY packages/constants/package.json ./packages/constants/
COPY packages/server/package.json ./packages/server/
COPY packages/types/package.json ./packages/types/
COPY packages/ui/package.json ./packages/ui/
COPY packages/public-app/package.json ./packages/public-app/
COPY packages/fake/package.json ./packages/fake/
COPY packages/tsconfig/package.json ./packages/tsconfig/
COPY packages/e2e-tests/package.json ./packages/e2e-tests/
COPY pnpm-lock.yaml ./
COPY pnpm-workspace.yaml ./
COPY patches ./patches
COPY turbo.json ./
RUN pnpm install --frozen-lockfile
RUN pnpm --filter=e2e-tests exec playwright install --with-deps

# Expose ports
EXPOSE 5173
EXPOSE 3000
EXPOSE 3001
EXPOSE 8086
EXPOSE 9323

# Entrypoint
COPY entry.sh ./

ENTRYPOINT [ "/usr/src/app/entry.sh" ]
