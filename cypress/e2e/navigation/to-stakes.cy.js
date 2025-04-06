import { DEFAULT_ACCOUNT_PK } from "../constants";
suite(["@tier2"], "staking ledger page", () => {
  [
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/delegations`,
      dest: "staking-ledgers",
      href: `q-delegate=${DEFAULT_ACCOUNT_PK}`,
    },
  ].forEach(({ origin, dest, href }) =>
    it(`is navigated to from ${dest}`, () => {
      cy.visit(origin);
      cy.get("a").contains("See all delegators").click();
      cy.url().should("contain", href);
      // cy.tableColumnValuesEqual("Staking Ledger - Epoch 0", "Delegate", DEFAULT_ACCOUNT_PK);
    }),
  );
});
