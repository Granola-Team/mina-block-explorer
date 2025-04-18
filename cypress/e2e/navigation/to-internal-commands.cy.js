import { GENESIS_ACCOUNT_PK } from "../constants";
describe("internal commands page", () => {
  [
    {
      origin: `/addresses/accounts/${GENESIS_ACCOUNT_PK}/commands/internal`,
      dest: "internal commands",
      href: `/commands/internal?q-recipient=${GENESIS_ACCOUNT_PK}`,
    },
  ].forEach(({ origin, href }) =>
    it(`is navigated to from ${origin}`, () => {
      cy.visit(origin);
      cy.contains("See all internal commands").click();
      cy.url().should("contain", href);
      cy.tableColumnValuesEqual(
        "Internal Commands",
        "Recipient",
        GENESIS_ACCOUNT_PK,
      );
    }),
  );
});
