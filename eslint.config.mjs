import pluginCypress from 'eslint-plugin-cypress/flat';
export default [
  pluginCypress.configs.recommended,
  {
    files: ['cypress/**/*.js', 'cypress/**/*.ts'], // Target Cypress files
    rules: {
      'cypress/no-unnecessary-waiting': 'off', // Customize rules as needed
    },
  },
];
