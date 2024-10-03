/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  extends: ['@hyped/eslint-config/react.js'],
  parserOptions: {
    project: true,
    tsconfigRootDir: __dirname,
  },
  ignorePatterns: ['vite.config.ts', 'app/components/ui/**'],
};
