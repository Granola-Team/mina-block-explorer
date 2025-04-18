import { GENESIS_ACCOUNT_PK } from "../constants";
describe("staking ledger page", () => {
  [
    {
      origin: `/addresses/accounts/${GENESIS_ACCOUNT_PK}/delegations`,
      dest: "staking-ledgers",
      href: `q-delegate=${GENESIS_ACCOUNT_PK}`,
    },
  ].forEach(({ origin, dest, href }) =>
    it(`is navigated to from ${dest}`, () => {
      cy.visit(origin);
      cy.get("a").contains("See all delegators").click();
      cy.url().should("contain", href);
      // cy.tableColumnValuesEqual("Staking Ledger - Epoch 0", "Delegate", GENESIS_ACCOUNT_PK);
    }),
  );
});
