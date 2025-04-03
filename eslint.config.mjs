import pluginCypress from 'eslint-plugin-cypress/flat';
import pluginImport from 'eslint-plugin-import'; // Fix this line

export default [
  pluginCypress.configs.recommended,
  {
    files: ['cypress/**/*.js', 'cypress/**/*.ts'],
    plugins: {
      import: pluginImport,
    },
    rules: {
      'cypress/no-unnecessary-waiting': 'off',
      'no-unused-vars': ['error'],
      'import/no-unused-modules': ['warn'],
      'import/named': 'error',
      'import/no-unresolved': 'error',
    },
  },
];
