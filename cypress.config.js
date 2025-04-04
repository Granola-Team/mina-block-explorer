const { defineConfig } = require("cypress");
const {
  addMatchImageSnapshotPlugin,
} = require("@simonsmith/cypress-image-snapshot/plugin");

module.exports = defineConfig({
  e2e: {
    setupNodeEvents(on, config) {
      addMatchImageSnapshotPlugin(on);
    },
    retries: 1,
    defaultCommandTimeout: 30000,
    responseTimeout: 30000,
    require: 30000
  },
});
