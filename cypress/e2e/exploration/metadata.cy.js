import { DEFAULT_ACCOUNT_PK } from "../constants";

const SHOWING_ALL = "Showing all";
const SHOWING_LATEST = "Showing 100 of all";
suite(["@CI"], "metadata about the table", () => {
  let pages = [
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
            expect(text).to.equal(SHOWING_ALL);
          });
      },
    },
    {
      page: "/commands/user",
      table_heading: "User Commands",
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
      page: "/commands/user",
      table_heading: "User Commands",
      column_selector: "input#q-height",
      input: 20,
      assertion: function (target) {
        target.should("have.lengthOf", 28);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            expect(text).to.equal(SHOWING_ALL);
          });
      },
    },
    {
      page: "/commands/internal",
      table_heading: "Internal Commands",
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
      page: "/commands/internal",
      table_heading: "Internal Commands",
      column_selector: "input#q-height",
      input: 20,
      assertion: function (target) {
        target.should("have.lengthOf", 16);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            expect(text).to.equal(SHOWING_ALL);
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
            expect(text).to.equal(SHOWING_ALL);
          });
      },
    },
    {
      page: "/staking-ledgers",
      table_heading: "Current Staking Ledger",
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
      table_heading: "Current Staking Ledger",
      column_selector: "input#q-key",
      input: DEFAULT_ACCOUNT_PK,
      assertion: function (target) {
        target.should("have.lengthOf", 1);
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            expect(text).to.equal(SHOWING_ALL);
          });
      },
    },
    // {
    //   page: "/next-stakes",
    //   table_heading: "Next Staking Ledger",
    //   column_selector: "input#q-key",
    //   input: null,
    //   assertion: function (target) {
    //     target.should("have.lengthOf", 100);
    //     cy.get('.metadata').invoke('text').then(text => {
    //       expect(text).to.equal(SHOWING_LATEST);
    //     })
    //   },
    // },
    // {
    //   page: "/next-stakes",
    //   table_heading: "Next Staking Ledger",
    //   column_selector: "input#q-key",
    //   input: DEFAULT_ACCOUNT_PK,
    //   assertion: function (target) {
    //     target.should("have.lengthOf", 1);
    //     cy.get('.metadata').invoke('text').then(text => {
    //       expect(text).to.equal(SHOWING_ALL);
    //     })
    //   },
    // },
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
