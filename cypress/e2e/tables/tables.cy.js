import {
  FIRST_BLOCK_PRODUCER_ADDRESS,
  FIRST_RECIPIENT_ADDRESS,
  FIRST_SENDER_ADDRESS,
  GENESIS_BLOCK_BLOCK_HASH,
  HUMANIZE_FINANACE_BLOCK_STATE_HASH,
  HUMANIZE_FINANCE_ADDRESS,
  HUMANIZE_FINANCE_TXN_HASH,
  HUMANIZE_FINANCE_USERNAME,
  SLOTS_PER_EPOCH,
} from "../constants";

let test_suite_data = [
  {
    tag: "@tier1",
    url: `/blocks/${HUMANIZE_FINANACE_BLOCK_STATE_HASH}/commands/user`,
    table: {
      heading: "User Commands",
      columns: ["Hash", "Type", "From", "To", "Fee", "Amount"],
      canonical_exists: false,
      filter_tests: [],
    },
    tests: [
      () => {
        cy.aliasTableRows("User Commands", "table-rows");
        cy.get("@table-rows").should("have.lengthOf", 9);
        cy.contains("payout from humanize finance e19");
      },
    ],
  },
  {
    tag: "@tier1",
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
          input: HUMANIZE_FINANCE_TXN_HASH,
          assertion: function () {
            cy.aliasTableRows("User Commands", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("User Commands", "Txn Hash", (text) => {
              expect(text).to.contain(HUMANIZE_FINANCE_TXN_HASH);
              expect(text).to.contain("payout from humanize finance e19");
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
    tag: "@tier1",
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
          input: HUMANIZE_FINANCE_ADDRESS,
          assertion: function () {
            cy.aliasTableRows("Staking Ledger - Epoch 20", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue(
              "Staking Ledger - Epoch 20",
              "Key",
              (text) => {
                expect(text).to.equal(HUMANIZE_FINANCE_ADDRESS);
              },
            );
            cy.tableColumnValuesEqual(
              "Staking Ledger - Epoch 20",
              "Username",
              HUMANIZE_FINANCE_USERNAME,
            );
          },
        },
      ],
    },
    tests: [],
  },
  {
    tag: "@tier1",
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
            cy.get("@table-rows").should("have.length.greaterThan", 1);
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
    tag: "@tier1",
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
            cy.get("@table-rows").should("have.length.greaterThan", 1);
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
  {
    tag: "@tier1",
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
              expect(height).to.be.gt(SLOTS_PER_EPOCH);
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
    tag: "@tier1",
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
          column: "Balance",
          input: 5000,
          assertion: function () {
            cy.aliasTableRows("Accounts", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 100);
            cy.assertForEachColumnValue("Accounts", "Balance", (text) => {
              let numString = text.replace(/,/g, "");
              let balance = parseFloat(numString);
              expect(balance).to.be.lte(5000);
            });
          },
        },
        {
          column: "Public Key",
          input: HUMANIZE_FINANCE_ADDRESS,
          assertion: function () {
            cy.aliasTableRows("Accounts", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("Accounts", "Public Key", (text) => {
              expect(text).to.equal(HUMANIZE_FINANCE_ADDRESS);
            });
            cy.tableColumnValuesEqual(
              "Accounts",
              "Username",
              HUMANIZE_FINANCE_USERNAME,
            );
          },
        },
        {
          column: "Username",
          input: HUMANIZE_FINANCE_USERNAME,
          assertion: function () {
            cy.aliasTableRows("Accounts", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("Accounts", "Username", (text) => {
              expect(text).to.equal(HUMANIZE_FINANCE_USERNAME);
            });
            cy.tableColumnValuesEqual(
              "Accounts",
              "Username",
              HUMANIZE_FINANCE_USERNAME,
            );
          },
        },
      ],
    },
    tests: [],
  },
  {
    tag: "@tier2",
    url: `/addresses/accounts/${HUMANIZE_FINANCE_ADDRESS}`,
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
          input: HUMANIZE_FINANCE_TXN_HASH,
          assertion: function () {
            cy.aliasTableRows("User Commands", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("User Commands", "Txn Hash", (text) => {
              expect(text).to.contain(HUMANIZE_FINANCE_TXN_HASH);
              expect(text).to.contain("payout from humanize finance e19");
            });
          },
        },
        {
          column: "Counterparty",
          input: "B62qqwCPPUFZsHyYZhncvoiWyq4c8FonAL5zvL5qAGReJog6TbAvBev",
          assertion: function () {
            cy.assertForEachColumnValue(
              "User Commands",
              "Counterparty",
              (text) => {
                expect(text).to.equal(
                  "B62qqwCPPUFZsHyYZhncvoiWyq4c8FonAL5zvL5qAGReJog6TbAvBev",
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
          HUMANIZE_FINANCE_ADDRESS,
          expected_fields,
        );
      },
      () => {
        cy.get("#spotlight-meta").should("contain", HUMANIZE_FINANCE_USERNAME);
      },
    ],
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
      cy.intercept("GET", "/summary").as("summaryData");
      cy.wait("@summaryData").then(() => {
        cy.tableHasOrderedColumns(heading, columns);

        filter_tests.forEach(({ column, input, assertion }) => {
          cy.get("th").contains(column).parents("th").find("input").as("input");
          cy.get("@input").type(input, { delay: 0 });
          cy.wait(1000);
          assertion();
          cy.aliasTableRows(heading, "table-rows");
          cy.get(".metadata").should("not.contain", "100 of 0");
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
  cy.url().then((url) => {
    const params = new URLSearchParams(url.split("?")[1]);

    if (params.get("txn-type") != null) {
      expect(cy.url().should("include", "txn-type=Non-Canonical"));
    } else if (params.get("canonical") != null) {
      expect(cy.url().should("include", "canonical=false"));
    } else {
      expect(true).to.equal(false);
    }
  });
  cy.get("@tableRows").should("not.have.class", "bg-status-success");
  cy.get("@tableRows").should("have.class", "bg-status-failed");

  cy.get("@menu").select("Canonical");
  cy.url().then((url) => {
    const params = new URLSearchParams(url.split("?")[1]);

    if (params.get("txn-type") != null) {
      expect(cy.url().should("include", "txn-type=Canonical"));
    } else if (params.get("canonical") != null) {
      expect(cy.url().should("include", "canonical=true"));
    } else {
      expect(true).to.equal(false);
    }
  });
  cy.wait(500);
  cy.contains("section:has(h1)", section)
    .find("table tr:not(:has(th)) span")
    .as("tableRows");
  cy.get("@tableRows").should("not.have.class", "bg-status-failed");
  cy.get("@tableRows").should("have.class", "bg-status-success");
};
