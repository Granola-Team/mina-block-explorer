import { DEFAULT_ACCOUNT_PK } from "../constants";

const SHOWING_LATEST = "Showing 100 of all";
suite(["@CI"], "metadata about the table", () => {
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
            expect(text).to.equal(SHOWING_LATEST);
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
            expect(text).to.equal(SHOWING_LATEST);
          });
      },
    },
    {
      page: "/blocks",
      table_heading: "Blocks",
      column_selector: "input#q-height",
      input: 20,
      assertion: function (target) {
        target.should("have.lengthOf", 20);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            expect(text).to.equal("Showing 20 of 20");
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
            expect(text).to.equal("Showing 28 of 28");
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
            expect(text).to.equal("Showing 35 of 35");
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
            expect(text).to.equal(SHOWING_LATEST);
          });
      },
    },
    {
      page: "/snarks",
      table_heading: "SNARKs",
      column_selector: "input#q-height",
      input: 111,
      assertion: function (target) {
        target.should("have.lengthOf", 64);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            expect(text).to.equal("Showing 64 of 64");
          });
      },
    },
    {
      page: "/staking-ledgers",
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
      page: "/staking-ledgers",
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
    {
      page: "/next-stakes",
      table_heading: "Next Staking Ledger",
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
      page: "/next-stakes",
      table_heading: "Next Staking Ledger",
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
      cy.wait(1000);
      if (input != null) {
        cy.get(column_selector).type(input, { delay: 5 });
      }
      cy.aliasTableRows(table_heading, "tr");
      assertion(cy.get("@tr"));
    }),
  );
});
