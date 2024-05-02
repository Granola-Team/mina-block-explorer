import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"], "empty table", () => {
  let pages = [
    "/blocks?q-state-hash=3Nfake",
    "/commands/user-commands?q-txn-hash=Cpkfake",
    "/commands/internal-commands?q-recipient=B62qfake",
    "/staking-ledgers?q-key=B62qfake",
    "/next-stakes?q-key=B62qfake",
    "/snarks?q-state-hash=3Nfake",
    `/addresses/accounts/${DEFAULT_ACCOUNT_PK}?q-height=-1`,
  ];

  pages.forEach((page) =>
    it(`on ${page} shows as having zero records`, () => {
      cy.visit(page);
      cy.contains("Showing 0 to 0 of 0 records");
    }),
  );
});
