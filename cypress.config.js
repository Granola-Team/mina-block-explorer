const { defineConfig } = require("cypress");
const {
  addMatchImageSnapshotPlugin,
} = require("@simonsmith/cypress-image-snapshot/plugin");

module.exports = defineConfig({
  e2e: {
    setupNodeEvents(on, config) {
      addMatchImageSnapshotPlugin(on);
    },
    retries: {
        runMode: 1,
        openMode: 0,
      },
    defaultCommandTimeout: 30000,
    responseTimeout: 30000,
    requestTimeout: 30000
  },
});
