import {
  FIRST_BLOCK_PRODUCER_ADDRESS,
  FIRST_RECIPIENT_ADDRESS,
  FIRST_SENDER_ADDRESS,
  LONG_LIVE_SNZ_HASH,
  GENESIS_BLOCK_BLOCK_HASH,
  SNZPOOL_ADDRESS,
} from "../constants";

let test_suite_data = [
  {
    tag: "@tier2",
    url: "/blocks",
    table: {
      heading: "Blocks",
      columns: [
        "Height",
        "State Hash",
        "Slot",
        "Age",
        "Block Producer",
        "Coinbase",
        "User Commands",
        "SNARKs",
        "Coinbase Receiver",
      ],
      canonical_exists: true,
      filter_tests: [
        {
          column: "Height",
          input: 50000,
          assertion: function () {
            cy.assertForEachColumnValue("Blocks", "Height", (text) => {
              let height = parseInt(text);
              expect(height).to.be.lte(50000);
            });
          },
        },
        {
          column: "State Hash",
          input: GENESIS_BLOCK_BLOCK_HASH,
          assertion: function () {
            cy.aliasTableRows("Blocks", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("Blocks", "State Hash", (text) => {
              expect(text).to.equal(GENESIS_BLOCK_BLOCK_HASH);
            });
          },
        },
        {
          column: "Slot",
          input: 90000,
          assertion: function () {
            cy.assertForEachColumnValue("Blocks", "Slot", (text) => {
              let height = parseInt(text);
              expect(height).to.be.lte(90000);
            });
          },
        },
        {
          column: "Block Producer",
          input: FIRST_BLOCK_PRODUCER_ADDRESS,
          assertion: function () {
            cy.assertForEachColumnValue("Blocks", "Block Producer", (text) => {
              expect(text).to.equal(FIRST_BLOCK_PRODUCER_ADDRESS);
            });
          },
        },
      ],
    },
    tests: [],
  },
  {
    tag: "@tier2",
    url: "/commands/user",
    table: {
      heading: "User Commands",
      columns: [
        "Height",
        "Txn Hash",
        "Age",
        "Type",
        "From",
        "To",
        "Nonce",
        "Fee",
        "Amount",
      ],
      canonical_exists: true,
      filter_tests: [
        {
          column: "Height",
          input: 50000,
          assertion: function () {
            cy.assertForEachColumnValue("User Commands", "Height", (text) => {
              let height = parseInt(text);
              expect(height).to.be.lte(50000);
            });
          },
        },
        {
          column: "Txn Hash",
          input: LONG_LIVE_SNZ_HASH,
          assertion: function () {
            cy.aliasTableRows("User Commands", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("User Commands", "Txn Hash", (text) => {
              expect(text).to.contain(LONG_LIVE_SNZ_HASH);
              expect(text).to.contain("LONG LIVE SNZ");
            });
          },
        },
        {
          column: "From",
          input: FIRST_SENDER_ADDRESS,
          assertion: function () {
            cy.assertForEachColumnValue("User Commands", "From", (text) => {
              expect(text).to.equal(FIRST_SENDER_ADDRESS);
            });
          },
        },
        {
          column: "To",
          input: FIRST_RECIPIENT_ADDRESS,
          assertion: function () {
            cy.assertForEachColumnValue("User Commands", "To", (text) => {
              expect(text).to.equal(FIRST_RECIPIENT_ADDRESS);
            });
          },
        },
      ],
    },
    tests: [],
  },
  {
    tag: "@tier2",
    url: `/addresses/accounts/${SNZPOOL_ADDRESS}`,
    table: {
      heading: "User Commands",
      columns: [
        "Height",
        "Txn Hash",
        "Nonce",
        "Age",
        "Type",
        "Direction",
        "Counterparty",
        "Amount/Fee",
      ],
      canonical_exists: true,
      filter_tests: [
        {
          column: "Height",
          input: 50000,
          assertion: function () {
            cy.assertForEachColumnValue("User Commands", "Height", (text) => {
              let height = parseInt(text);
              expect(height).to.be.lte(50000);
            });
          },
        },
        {
          column: "Txn Hash",
          input: LONG_LIVE_SNZ_HASH,
          assertion: function () {
            cy.aliasTableRows("User Commands", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("User Commands", "Txn Hash", (text) => {
              expect(text).to.contain(LONG_LIVE_SNZ_HASH);
              expect(text).to.contain("LONG LIVE SNZ");
            });
          },
        },
        {
          column: "Counterparty",
          input: "B62qkjRK7EREPshjerdoNiZzdkvbUkmg9VH4QB7vrk5KzpPpyEB333y",
          assertion: function () {
            cy.assertForEachColumnValue(
              "User Commands",
              "Counterparty",
              (text) => {
                expect(text).to.equal(
                  "B62qkjRK7EREPshjerdoNiZzdkvbUkmg9VH4QB7vrk5KzpPpyEB333y",
                );
              },
            );
          },
        },
      ],
    },
    tests: [
      () => {
        let expected_fields = ["Balance", "Delegate"];
        cy.testSpotlight(
          "Account Spotlight",
          "B62qoaMj7u1JzuqXaBByQBL5jzqLguK8e7LHVPdY9LcvvLXK7HPsusD",
          expected_fields,
        );
      },
      () => {
        cy.get("#spotlight-meta").should("contain", "SNZPool");
      },
    ],
  },
  {
    tag: "@tier2",
    url: "/addresses/accounts",
    table: {
      heading: "Accounts",
      columns: [
        "Public Key",
        "Username",
        "Balance",
        "Nonce",
        "Delegate",
        "Time Locked",
      ],
      canonical_exists: false,
      filter_tests: [
        {
          column: "Public Key",
          input: SNZPOOL_ADDRESS,
          assertion: function () {
            cy.aliasTableRows("Accounts", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("Accounts", "Public Key", (text) => {
              expect(text).to.equal(SNZPOOL_ADDRESS);
            });
            cy.tableColumnValuesEqual("Accounts", "Username", "SNZPool");
          },
        },
      ],
    },
    tests: [],
  },
  {
    tag: "@tier2",
    url: "/staking-ledgers?epoch=20",
    table: {
      columns: [
        "Key",
        "Username",
        "Stake",
        "Total Stake %",
        "Block Win %",
        "Delegate",
        "Delegators",
      ],
      heading: "Staking Ledger - Epoch 20",
      canonical_exists: false,
      filter_tests: [
        {
          column: "Key",
          input: SNZPOOL_ADDRESS,
          assertion: function () {
            cy.aliasTableRows("Staking Ledger - Epoch 20", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue(
              "Staking Ledger - Epoch 20",
              "Key",
              (text) => {
                expect(text).to.equal(SNZPOOL_ADDRESS);
              },
            );
            cy.tableColumnValuesEqual(
              "Staking Ledger - Epoch 20",
              "Username",
              "SNZPool",
            );
          },
        },
      ],
    },
    tests: [],
  },
  {
    tag: "@tier2",
    url: "/commands/internal",
    table: {
      heading: "Internal Commands",
      columns: ["Height", "State Hash", "Recipient", "Fee", "Type", "Age"],
      canonical_exists: true,
      filter_tests: [
        {
          column: "Height",
          input: 50000,
          assertion: function () {
            cy.assertForEachColumnValue(
              "Internal Commands",
              "Height",
              (text) => {
                let height = parseInt(text);
                expect(height).to.be.lte(50000);
              },
            );
          },
        },
        {
          column: "State Hash",
          input: "3NKP5eG9qvMbJM78thgQMvwnNnjrEkfcAfS31YbNNzeBsa8ADqPR",
          assertion: function () {
            cy.aliasTableRows("Internal Commands", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue(
              "Internal Commands",
              "State Hash",
              (text) => {
                expect(text).to.equal(
                  "3NKP5eG9qvMbJM78thgQMvwnNnjrEkfcAfS31YbNNzeBsa8ADqPR",
                );
              },
            );
          },
        },
        {
          column: "Recipient",
          input: "B62qnucUMHz7Dw2ReNgWhmR5XCvPeQjJWPReuQ8GwPyY4qj1otGBiKr",
          assertion: function () {
            cy.assertForEachColumnValue(
              "Internal Commands",
              "Recipient",
              (text) => {
                expect(text).to.equal(
                  "B62qnucUMHz7Dw2ReNgWhmR5XCvPeQjJWPReuQ8GwPyY4qj1otGBiKr",
                );
              },
            );
          },
        },
      ],
    },
    tests: [],
  },
  {
    tag: "@tier2",
    url: "/snarks",
    table: {
      heading: "SNARKs",
      columns: ["Height", "State Hash", "Age", "Prover", "Fee"],
      canonical_exists: true,
      filter_tests: [
        {
          column: "Height",
          input: 50000,
          assertion: function () {
            cy.assertForEachColumnValue("SNARKs", "Height", (text) => {
              let height = parseInt(text);
              expect(height).to.be.lte(50000);
            });
          },
        },
        {
          column: "State Hash",
          input: "3NKP5eG9qvMbJM78thgQMvwnNnjrEkfcAfS31YbNNzeBsa8ADqPR",
          assertion: function () {
            cy.aliasTableRows("SNARKs", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("SNARKs", "State Hash", (text) => {
              expect(text).to.equal(
                "3NKP5eG9qvMbJM78thgQMvwnNnjrEkfcAfS31YbNNzeBsa8ADqPR",
              );
            });
          },
        },
        {
          column: "Prover",
          input: "B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC",
          assertion: function () {
            cy.assertForEachColumnValue("SNARKs", "Prover", (text) => {
              expect(text).to.equal(
                "B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC",
              );
            });
          },
        },
      ],
    },
    tests: [],
  },
];

test_suite_data.forEach((test_suite_datum) => {
  const {
    tag,
    url,
    table: { heading, filter_tests, canonical_exists, columns },
    tests,
  } = test_suite_datum;

  suite([tag], `table on ${url}`, () => {
    it("has standard functionality", () => {
      cy.visit(url);
      cy.viewport(768, 2000);
      cy.wait(1000);

      cy.tableHasOrderedColumns(heading, columns);

      filter_tests.forEach(({ column, input, assertion }) => {
        cy.get("th").contains(column).parents("th").find("input").as("input");

        cy.get("@input").type(input, { delay: 0 });
        cy.wait(1000);
        assertion();
        cy.wait(1000);
        cy.assertTableRecordsCorrect(heading);
        cy.get("@input").clear();
      });

      if (canonical_exists) {
        canonicalTest(heading);
      }

      tests.forEach((test) => test());
    });
  });
});

const canonicalTest = (section) => {
  cy.get("select#canonical-selection").as("menu");
  cy.url().should("not.include", "canonical");

  // should load canonical by default
  cy.contains("section:has(h1)", section)
    .find("table tr:not(:has(th)) span")
    .as("tableRows");
  cy.get("@tableRows").should("not.have.class", "bg-status-failed");
  cy.get("@tableRows").should("have.class", "bg-status-success");

  cy.wait(500);
  cy.get("@menu").select("Non-Canonical");
  cy.wait(500);
  cy.url().should("include", "canonical=false");
  cy.get("@tableRows").should("not.have.class", "bg-status-success");
  cy.get("@tableRows").should("have.class", "bg-status-failed");

  cy.get("@menu").select("Canonical");
  cy.url().should("include", "canonical=true");
  cy.wait(500);
  cy.contains("section:has(h1)", section)
    .find("table tr:not(:has(th)) span")
    .as("tableRows");
  cy.get("@tableRows").should("not.have.class", "bg-status-failed");
  cy.get("@tableRows").should("have.class", "bg-status-success");
};
