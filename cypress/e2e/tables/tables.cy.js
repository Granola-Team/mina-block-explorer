import {
  FIRST_BLOCK_PRODUCER_ADDRESS,
  FIRST_RECIPIENT_ADDRESS,
  FIRST_SENDER_ADDRESS,
  GENESIS_BLOCK_BLOCK_HASH,
  BLOCK_STATE_HASH_MIXED_USER_COMMANDS,
  ROMEK_ADDRESS,
  ROMEK_MINA_NAMING_SERVICE_TXN_HASH,
  ROMEK_USERNAME,
  SLOTS_PER_EPOCH,
  MINA_NAMING_SERVICE_ADDRESS,
  ROMEK_BLOCK_STATE_HASH,
  VETAL_BLOCK_STATE_HASH,
  ROMEK_NAMING_MEMO,
} from "../constants";
import { parseFormattedNumber } from "../helpers";

let test_suite_data = [
  {
    tag: "@tier2",
    url: `/analytics/snarker-leaderboard?epoch=0`,
    table: {
      heading: "Snarker Leaderboard",
      columns: [
        "Username",
        "Public Key",
        "Total Fees",
        "Min Fee",
        "Max Fee",
        "Snarks Sold",
      ],
      filter_tests: [],
    },
    tests: [],
  },
  {
    tag: "@tier2",
    url: `/analytics/staker-leaderboard?epoch=0`,
    table: {
      heading: "Staker Leaderboard",
      columns: [
        "Username",
        "Public Key",
        "Canonical Blocks Produced",
        "Supercharged Blocks Produced",
        "Slots Produced",
        "Orphan Rate",
      ],
      filter_tests: [],
    },
    tests: [],
  },
  {
    tag: "@tier2",
    url: `/analytics/snarks`,
    table: {
      heading: "SNARK Fees Overview",
      columns: ["Metric", "All SNARKs", "SNARKs with non-zero fees"],
      filter_tests: [],
    },
    tests: [],
  },
  {
    tag: "@tier2",
    url: `/blocks/${BLOCK_STATE_HASH_MIXED_USER_COMMANDS}/commands/user`,
    exclusive: true,
    table: {
      heading: "User Commands",
      columns: [
        "Hash",
        "Type",
        "Status",
        "From",
        "To",
        "Nonce",
        "Fee",
        "Amount",
      ],
      filter_tests: [],
    },
    tests: [
      () => {
        cy.aliasTableRows("User Commands", "table-rows");
        cy.get("@table-rows").should("have.lengthOf", 6);
        cy.get("@table-rows").eq(0).should("contain.text", "Failed");
        cy.get("@table-rows").eq(5).should("contain.text", "Applied");
      },
    ],
  },
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
      filter_tests: [
        {
          column: "Height",
          input: 50000,
          assertion: function () {
            cy.assertForEachColumnValue("Blocks", "Height", (text) => {
              let height = parseFormattedNumber(text);
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
              let height = parseFormattedNumber(text);
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
    tests: [
      () => {
        [25, 50, 100, 250].forEach((l) => {
          cy.assertRowLimitWorks("Blocks", l);
        });
      },
    ],
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
        "Status",
        "From",
        "To",
        "Nonce",
        "Fee",
        "Amount",
      ],
      filter_tests: [
        {
          column: "Height",
          input: 50000,
          assertion: function () {
            cy.assertForEachColumnValue("User Commands", "Height", (text) => {
              let height = parseFormattedNumber(text);
              expect(height).to.be.lte(50000);
            });
          },
        },
        {
          column: "Txn Hash",
          input: ROMEK_MINA_NAMING_SERVICE_TXN_HASH,
          assertion: function () {
            cy.aliasTableRows("User Commands", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("User Commands", "Txn Hash", (text) => {
              expect(text).to.contain(ROMEK_MINA_NAMING_SERVICE_TXN_HASH);
              expect(text).to.contain(ROMEK_NAMING_MEMO);
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
    tests: [
      () => {
        [25, 50, 100, 250].forEach((l) => {
          cy.assertRowLimitWorks("User Commands", l);
        });
      },
    ],
  },
  {
    tag: "@tier2",
    url: `/addresses/accounts/${ROMEK_ADDRESS}`,
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
      filter_tests: [
        {
          column: "Height",
          input: 2500,
          assertion: function () {
            cy.assertForEachColumnValue("User Commands", "Height", (text) => {
              let height = parseFormattedNumber(text);
              expect(height).to.be.lte(2500);
            });
          },
        },
        {
          column: "Txn Hash",
          input: ROMEK_MINA_NAMING_SERVICE_TXN_HASH,
          assertion: function () {
            cy.aliasTableRows("User Commands", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("User Commands", "Txn Hash", (text) => {
              expect(text).to.contain(ROMEK_MINA_NAMING_SERVICE_TXN_HASH);
              expect(text).to.contain(ROMEK_NAMING_MEMO);
            });
          },
        },
        {
          column: "Counterparty",
          input: MINA_NAMING_SERVICE_ADDRESS,
          assertion: function () {
            cy.assertForEachColumnValue(
              "User Commands",
              "Counterparty",
              (text) => {
                expect(text).to.equal(MINA_NAMING_SERVICE_ADDRESS);
              },
            );
          },
        },
      ],
    },
    tests: [
      () => {
        let expected_fields = [
          "Balance",
          "Delegate",
          "Nonce",
          "Updated Block #",
        ];
        cy.testSpotlight("Account Spotlight", ROMEK_ADDRESS, expected_fields);
      },
      () => {
        cy.get("#spotlight-meta").should("contain", ROMEK_USERNAME);
        cy.testSpotlightValue(
          "Balance",
          "Includes 1 MINA account creation fee",
        );
      },
    ],
  },
  {
    tag: "@tier2",
    url: `/addresses/accounts/B62qiVr4Wy6yKhxNV49Npnpr2XF5AhsFejFWWQpWKARQpTYsb9snNZY/commands/internal`,
    table: {
      heading: "Internal Commands",
      columns: ["Height", "State Hash", "Fee", "Type", "Age"],
      filter_tests: [
        {
          column: "Height",
          input: 5200,
          assertion: function () {
            cy.wait(1000);
            cy.assertForEachColumnValue(
              "Internal Commands",
              "Height",
              (text) => {
                let height = parseFormattedNumber(text);
                expect(height).to.be.lte(5200);
              },
            );
          },
        },
        {
          column: "State Hash",
          input: "3NKq6mHhx31ikA9Gax1JcRuzTMp3tMudKfcwt3VxMDnvAeMYZGPA",
          assertion: function () {
            cy.aliasTableRows("Internal Commands", "table-rows");
            cy.assertForEachColumnValue(
              "Internal Commands",
              "State Hash",
              (text) => {
                expect(text).to.contain(
                  "3NKq6mHhx31ikA9Gax1JcRuzTMp3tMudKfcwt3VxMDnvAeMYZGPA",
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
    url: `/addresses/accounts/${MINA_NAMING_SERVICE_ADDRESS}/delegations`,
    table: {
      heading: "Delegations",
      columns: [
        "Public Key",
        "Username",
        "Delegated Balance",
        "% of Delegation",
      ],
      filter_tests: [],
    },
    tests: [],
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
      filter_tests: [
        {
          column: "Balance",
          input: 5000,
          assertion: function () {
            cy.aliasTableRows("Accounts", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 25);
            cy.assertForEachColumnValue("Accounts", "Balance", (text) => {
              let balance = parseFormattedNumber(text);
              expect(balance).to.be.lte(5000);
            });
          },
        },
        {
          column: "Public Key",
          input: ROMEK_ADDRESS,
          assertion: function () {
            cy.aliasTableRows("Accounts", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("Accounts", "Public Key", (text) => {
              expect(text).to.equal(ROMEK_ADDRESS);
            });
            cy.tableColumnValuesEqual("Accounts", "Username", ROMEK_USERNAME);
          },
        },
        {
          column: "Username",
          input: ROMEK_USERNAME,
          assertion: function () {
            cy.aliasTableRows("Accounts", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue("Accounts", "Username", (text) => {
              expect(text).to.equal(ROMEK_USERNAME);
            });
            cy.tableColumnValuesEqual("Accounts", "Username", ROMEK_USERNAME);
          },
        },
        {
          column: "Delegate",
          input: ROMEK_ADDRESS,
          assertion: function () {
            cy.aliasTableRows("Accounts", "table-rows");
            cy.assertForEachColumnValue("Accounts", "Delegate", (text) => {
              expect(text).to.equal(ROMEK_ADDRESS);
            });
          },
        },
      ],
    },
    tests: [
      () => {
        [25, 50, 100, 250].forEach((l) => {
          cy.assertRowLimitWorks("Accounts", l);
        });
      },
    ],
  },
  {
    disabled: true,
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
      filter_tests: [
        {
          column: "Key",
          input: ROMEK_ADDRESS,
          assertion: function () {
            cy.aliasTableRows("Staking Ledger - Epoch 20", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue(
              "Staking Ledger - Epoch 20",
              "Key",
              (text) => {
                expect(text).to.equal(ROMEK_ADDRESS);
              },
            );
            cy.tableColumnValuesEqual(
              "Staking Ledger - Epoch 20",
              "Username",
              ROMEK_USERNAME,
            );
          },
        },
        {
          column: "Stake",
          input: "37,123,109.762455837",
          assertion: function () {
            cy.aliasTableRows("Staking Ledger - Epoch 20", "table-rows");
            cy.assertForEachColumnValue(
              "Staking Ledger - Epoch 20",
              "Stake",
              (text) => {
                expect(parseFloat(text)).to.be.lte(
                  parseFloat("37,123,109.762455837"),
                );
              },
            );
          },
        },
      ],
    },
    tests: [
      () => {
        cy.get(".metadata")
          .invoke("text")
          .then((text) => {
            expect(text.split(" of ").length).to.equal(3);
          });
      },
    ],
  },
  {
    tag: "@tier2",
    url: "/commands/internal",
    table: {
      heading: "Internal Commands",
      columns: ["Height", "State Hash", "Recipient", "Fee", "Type", "Age"],
      filter_tests: [
        {
          column: "Height",
          input: 50000,
          assertion: function () {
            cy.assertForEachColumnValue(
              "Internal Commands",
              "Height",
              (text) => {
                let height = parseFormattedNumber(text);
                expect(height).to.be.lte(50000);
              },
            );
          },
        },
        {
          column: "State Hash",
          input: ROMEK_BLOCK_STATE_HASH,
          assertion: function () {
            cy.aliasTableRows("Internal Commands", "table-rows");
            cy.get("@table-rows").should("have.length.greaterThan", 1);
            cy.assertForEachColumnValue(
              "Internal Commands",
              "State Hash",
              (text) => {
                expect(text).to.equal(ROMEK_BLOCK_STATE_HASH);
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
      filter_tests: [
        {
          column: "Height",
          input: 50000,
          assertion: function () {
            cy.assertForEachColumnValue("SNARKs", "Height", (text) => {
              let height = parseFormattedNumber(text);
              expect(height).to.be.lte(50000);
            });
          },
        },
        {
          column: "State Hash",
          input: VETAL_BLOCK_STATE_HASH,
          assertion: function () {
            cy.aliasTableRows("SNARKs", "table-rows");
            cy.get("@table-rows").should("have.length.greaterThan", 1);
            cy.assertForEachColumnValue("SNARKs", "State Hash", (text) => {
              expect(text).to.equal(VETAL_BLOCK_STATE_HASH);
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
    disabled,
    tag,
    url,
    table: { heading, filter_tests, columns },
    tests,
  } = test_suite_datum;

  suite([tag], `table on ${url}`, () => {
    if (disabled) {
      xit("has standard functionality", () => {});
    } else {
      it("has standard functionality", () => {
        cy.visit(url);
        cy.viewport(768, 2000);
        cy.intercept("GET", "/summary").as("summaryData");
        cy.wait("@summaryData").then(() => {
          cy.tableHasOrderedColumns(heading, columns);
          filter_tests.forEach(({ column, input, assertion }) => {
            cy.get("th").contains(column).find("input").as("input");
            cy.wait(1000);
            cy.get("@input").type(input, { delay: 0 });
            cy.wait(1000);
            assertion();
            cy.assertTableRecordsCorrect(heading);
            cy.get("@input").clear();
          });

          tests.forEach((test) => test());
        });
      });
    }
  });
});
