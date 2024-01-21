const { defineConfig } = require("cypress");
const { addMatchImageSnapshotPlugin } = require("@simonsmith/cypress-image-snapshot/plugin");

module.exports = defineConfig({
  e2e: {
    baseUrl: 'http://localhost:5274',
    setupNodeEvents(on, config) {
      addMatchImageSnapshotPlugin(on)
    },
    retries: {
      runMode: 3
    }
  },
});
