import unusedImports from 'eslint-plugin-unused-imports';

export default [
  {
    files: ['cypress/**/*.js'],
    languageOptions: {
      ecmaVersion: 2021,
      sourceType: 'module'
    },
    plugins: {
      'unused-imports': unusedImports
    },
    rules: {
      'unused-imports/no-unused-imports': 'error', // Detects and fixes unused imports
      'no-unused-vars': 'error' // Still flags unused variables (not auto-fixable)
    }
  }
];
