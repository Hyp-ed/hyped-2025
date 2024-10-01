/** @type {import("eslint").Linter.Config} */
module.exports = {
  extends: [
    'eslint:recommended',
    // This isn't working properly. Getting an error that I can't figure out.
    // 'eslint-config-turbo',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/recommended-requiring-type-checking',
    'prettier',
  ],
  parser: '@typescript-eslint/parser',
  plugins: ['only-warn', '@typescript-eslint'],
  ignorePatterns: [
    '**/dist/**/*',
    '**/node_modules/**/*',
    '.eslintrc.js',
    '.eslintrc.cjs',
  ],
  rules: {
    'no-console': 'error',
    // We can tighten up the below rules later. They're not worth the effort at the moment.
    '@typescript-eslint/no-explicit-any': 'off',
    '@typescript-eslint/no-unsafe-member-access': 'off',
    '@typescript-eslint/no-unsafe-return': 'off',
    '@typescript-eslint/no-unsafe-call': 'off',
  },
  root: true,
};
