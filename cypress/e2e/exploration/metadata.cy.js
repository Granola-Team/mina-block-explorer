import { DEFAULT_ACCOUNT_PK } from "../constants";

const SHOWING_LATEST = "Showing 100 of all";

function extractMetadata(input) {
  let regex = /Showing (\d+) of (\d+)/;
  const match = input.match(regex);
  return match
    ? {
        records: +match[1],
        total_records: +match[2],
      }
    : null;
}

suite(["@CI"], "metadata about the table", () => {
  beforeEach(() => {
    cy.clearLocalStorage();
  });

  let pages = [
    {
      page: `/addresses/accounts`,
      table_heading: "Accounts",
      column_selector: "input#q-public-key",
      input: null,
      assertion: function (target) {
        target.should("have.lengthOf", 100);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            let { records, total_records } = extractMetadata(text);
            expect(records).to.be.lte(total_records);
          });
      },
    },
    {
      page: `/addresses/accounts`,
      table_heading: "Accounts",
      column_selector: "input#q-public-key",
      input: DEFAULT_ACCOUNT_PK,
      assertion: function (target) {
        target.should("have.lengthOf", 1);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            expect(text).to.equal("Showing 1 of 1");
          });
      },
    },
    {
      page: "/blocks",
      table_heading: "Blocks",
      column_selector: "input#q-height",
      input: null,
      assertion: function (target) {
        target.should("have.lengthOf", 100);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            let { records, total_records } = extractMetadata(text);
            expect(records).to.be.lte(total_records);
          });
      },
    },
    {
      page: "/commands/user",
      table_heading: "User Commands",
      column_selector: "input#q-height",
      input: 20,
      assertion: function (target) {
        target.should("have.lengthOf", 28);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            let { records, total_records } = extractMetadata(text);
            expect(records).to.be.lte(total_records);
          });
      },
    },
    {
      page: "/commands/internal",
      table_heading: "Internal Commands",
      column_selector: "input#q-height",
      input: 20,
      assertion: function (target) {
        target.should("have.lengthOf", 35);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            let { records, total_records } = extractMetadata(text);
            expect(records).to.be.lte(total_records);
          });
      },
    },
    {
      page: "/snarks",
      table_heading: "SNARKs",
      column_selector: "input#q-height",
      input: null,
      assertion: function (target) {
        target.should("have.lengthOf", 100);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            let { records, total_records } = extractMetadata(text);
            expect(records).to.be.lte(total_records);
          });
      },
    },
    {
      page: "/staking-ledgers?epoch=1",
      table_heading: "Staking Ledger",
      column_selector: "input#q-key",
      input: null,
      assertion: function (target) {
        target.should("have.lengthOf", 100);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            expect(text).to.equal(SHOWING_LATEST);
          });
      },
    },
    {
      page: "/staking-ledgers?epoch=1",
      table_heading: "Staking Ledger",
      column_selector: "input#q-key",
      input: DEFAULT_ACCOUNT_PK,
      assertion: function (target) {
        target.should("have.lengthOf", 1);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            expect(text).to.equal("Showing 1 of 1");
          });
      },
    },
  ];

  pages.forEach(({ page, column_selector, input, assertion, table_heading }) =>
    it(`is correct on page ${page}`, () => {
      cy.visit(page);
      cy.wait(3000);
      if (input != null) {
        cy.get(column_selector).type(input, { delay: 5 });
      }
      cy.aliasTableRows(table_heading, "tr");
      assertion(cy.get("@tr"));
    }),
  );
});
