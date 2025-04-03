suite(["@tier2"], "empty table", () => {
  let pages = [
    "/addresses/accounts?q-public-key=B62fake",
    "/blocks?q-state-hash=3Nfake",
    "/commands/user?q-txn-hash=Cpkfake",
    "/commands/internal?q-recipient=B62qfake",
    "/staking-ledgers?q-key=B62qfake",
    "/snarks?q-state-hash=3Nfake",
    "/analytics/staker-leaderboard?epoch=100000",
    "/analytics/snarker-leaderboard?epoch=100000",
  ];
  pages.forEach((page) =>
    it(`on ${page} shows as having zero records`, () => {
      cy.visit(page);
      cy.wait(500);
      cy.contains("No data for this view");
    }),
  );
});
