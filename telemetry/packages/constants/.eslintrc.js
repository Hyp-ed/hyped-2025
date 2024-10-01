/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  extends: ['@hyped/eslint-config/basic.js'],
  parserOptions: {
    project: true,
    tsconfigRootDir: __dirname,
  },
};
