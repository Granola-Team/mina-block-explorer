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
  SNZ_USERNAME,
  SNZPOOL_ADDRESS,
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
      sorting_columns: [
        {
          column: "Max Fee",
          type: "numeric",
          // Sort options order should be the same as the cycle order, starting with the default sort
          sort_options: [null, "MAX_FEE_DESC", "MAX_FEE_ASC"],
        },
        {
          column: "Total Fees",
          type: "numeric",
          // Sort options order should be the same as the cycle order, starting with the default sort
          sort_options: [null, "TOTAL_FEES_DESC", "TOTAL_FEES_ASC"],
        },
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
        "Date",
        "Block Producer",
        "Coinbase",
        "User Commands",
        "SNARKs",
        "Coinbase Receiver",
      ],
      filter_tests: [
        {
          column: "Height",
          input: 2000,
          assertion: function () {
            cy.assertForEachColumnValue("Blocks", "Height", (text) => {
              let height = parseFormattedNumber(text);
              expect(height).to.be.lte(2000);
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
        cy.assertStandardRowLimits("Blocks");
      },
      () => {
        cy.get("select#canonical-selection").as("canonical");
        cy.get("@canonical").select("Canonical");
        cy.intercept("POST", "/graphql").as("graphql");
        cy.wait("@graphql").then(() => {
          cy.aliasTableRows("Blocks", "table-rows");
          cy.get("@table-rows").find(".non-canonical").should("not.exist");
          cy.get("@table-rows").find(".canonical").should("exist");
        });

        cy.get("@canonical").select("Non-Canonical");
        cy.intercept("POST", "/graphql").as("graphql");
        cy.wait("@graphql").then(() => {
          cy.aliasTableRows("Blocks", "table-rows");
          cy.get("@table-rows").find(".non-canonical").should("exist");
          cy.get("@table-rows").find(".canonical").should("not.exist");
        });

        cy.get("@canonical").select("All");
        cy.intercept("POST", "/graphql").as("graphql");
        cy.wait("@graphql").then(() => {
          cy.aliasTableRows("Blocks", "table-rows");
          cy.get("@table-rows").find(".non-canonical").should("exist");
          cy.get("@table-rows").find(".canonical").should("exist");
        });
      },
      () => {
        cy.intercept("POST", "/graphql").as("graphql");
        cy.visit("/blocks?q-height=25");
        cy.wait("@graphql").then(() => {
          cy.wait(1000);
          cy.assertLoadNextWorks("Blocks", "Height", {
            button_text: "Load Next",
            expected_button_state: "be.disabled",
          });
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
        "Date",
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
          input: 2000,
          assertion: function () {
            cy.assertForEachColumnValue("User Commands", "Height", (text) => {
              let height = parseFormattedNumber(text);
              expect(height).to.be.lte(2000);
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
        cy.assertStandardRowLimits("User Commands");
      },
      () => {
        cy.get("select#txn-status").as("txn-applied");
        cy.get("select#canonical-selection").as("canonical");
        ["Failed", "Applied"].forEach((txnApplied) => {
          ["Non-Canonical", "Canonical"].forEach((canonical) => {
            cy.get("@txn-applied")
              .select(txnApplied)
              .should("have.value", txnApplied);
            cy.get("@canonical")
              .select(canonical)
              .should("have.value", canonical);
            cy.intercept("POST", "/graphql").as("graphql");
            cy.wait("@graphql").then(() => {
              cy.assertForEachColumnValue("User Commands", "Status", (text) => {
                expect(text).to.be.eq(txnApplied);
              });
              cy.clickLinkInTable(0, "Txn Hash", "User Commands");
              cy.testSpotlightValue("Status", txnApplied);
              cy.testSpotlightValue(
                "Canonical",
                "" + (canonical === "Canonical"),
              );
              cy.go("back");
            });
          });
        });
      },
      () => {
        cy.intercept("POST", "/graphql").as("graphql");
        cy.visit("/commands/user?q-height=25");
        cy.wait("@graphql").then(() => {
          cy.wait(1000);
          cy.assertLoadNextWorks("User Commands", "Height", {
            button_text: "Load Next",
            expected_button_state: "be.disabled",
          });
        });
      },
    ],
  },
  {
    tag: "@tier2",
    url: "/commands/pending",
    table: {
      heading: "Pending Commands",
      columns: ["Txn Hash", "Type", "From", "To", "Nonce", "Fee", "Amount"],
      filter_tests: [],
    },
    tests: [],
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
        "Date",
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
      columns: ["Height", "State Hash", "Fee", "Type", "Date"],
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
      sorting_columns: [
        {
          column: "Balance",
          type: "numeric",
          // Sort options order should be the same as the cycle order, starting with the default sort
          sort_options: ["BALANCE_DESC", "BALANCE_ASC"],
        },
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
        cy.assertStandardRowLimits("Accounts");
      },
    ],
  },
  {
    tag: "@tier2",
    url: "/staking-ledgers?epoch=1",
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
      sorting_columns: [
        {
          column: "Total Stake %",
          type: "numeric",
          // Sort options order should be the same as the cycle order, starting with the default sort
          sort_options: ["STAKE_DESC", "STAKE_ASC"],
        },
      ],
      heading: "Staking Ledger - Epoch 1",
      filter_tests: [
        {
          column: "Key",
          input: SNZPOOL_ADDRESS,
          assertion: function () {
            cy.aliasTableRows("Staking Ledger - Epoch 1", "table-rows");
            cy.get("@table-rows").should("have.lengthOf", 1);
            cy.assertForEachColumnValue(
              "Staking Ledger - Epoch 1",
              "Key",
              (text) => {
                expect(text).to.equal(SNZPOOL_ADDRESS);
              },
            );
            cy.tableColumnValuesEqual(
              "Staking Ledger - Epoch 1",
              "Username",
              SNZ_USERNAME,
            );
          },
        },
        {
          column: "Stake",
          input: "7,399,987.246422696",
          assertion: function () {
            cy.aliasTableRows("Staking Ledger - Epoch 1", "table-rows");
            cy.assertForEachColumnValue(
              "Staking Ledger - Epoch 1",
              "Stake",
              (text) => {
                expect(parseFloat(text)).to.be.lte(
                  parseFloat("7,399,987.246422696"),
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
      () => {
        cy.assertStandardRowLimits("Staking Ledger - Epoch 1");
      },
    ],
  },
  {
    tag: "@tier2",
    url: "/commands/internal",
    table: {
      heading: "Internal Commands",
      columns: ["Height", "State Hash", "Recipient", "Fee", "Type", "Date"],
      filter_tests: [
        {
          column: "Height",
          input: 2000,
          assertion: function () {
            cy.assertForEachColumnValue(
              "Internal Commands",
              "Height",
              (text) => {
                let height = parseFormattedNumber(text);
                expect(height).to.be.lte(2000);
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
    tests: [
      () => {
        cy.assertStandardRowLimits("Internal Commands");
      },
    ],
  },
  {
    tag: "@tier2",
    url: "/snarks",
    table: {
      heading: "SNARKs",
      columns: ["Height", "State Hash", "Date", "Prover", "Fee"],
      filter_tests: [
        {
          column: "Height",
          input: 2000,
          assertion: function () {
            cy.assertForEachColumnValue("SNARKs", "Height", (text) => {
              let height = parseFormattedNumber(text);
              expect(height).to.be.lte(2000);
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
    tests: [
      () => {
        cy.assertStandardRowLimits("SNARKs");
      },
    ],
  },
];

test_suite_data.forEach((test_suite_datum) => {
  const {
    disabled,
    tag,
    url,
    table: { heading, filter_tests, columns, sorting_columns = [] },
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
          if (columns.filter((c) => c === "Date").length > 0) {
            cy.assertForEachColumnValue(heading, "Date", (dateText) => {
              // Attempt to parse the date string
              const parsedDate = new Date(dateText);

              // Assert that the date is valid
              expect(parsedDate.toString()).not.to.equal("Invalid Date");
            });
          }
          sorting_columns.forEach(({ column, type, sort_options }) => {
            sort_options.forEach((sort, i) => {
              if (sort != null) {
                cy.log("Testing Sort Order: " + sort);
                cy.assertSortOrder(
                  heading,
                  column,
                  sort.includes("DESC"),
                  type,
                );
                // we don't necessarily expect the url to indicate
                // sort direction on the first page load
                if (i !== 0) {
                  cy.url().should("include", `sort-dir=${sort}`);
                }
              }
              cy.get("th").contains(column).click("top");
            });
          });
          filter_tests.forEach(({ column, input, assertion }) => {
            cy.get("th").contains(column).find("input").as("input");
            cy.wait(1000);
            cy.get("@input").type(input, { delay: 0 });
            cy.wait(2000);
            assertion();
            if (heading != "Staking Ledger - Epoch 1") {
              cy.assertTableRecordsCorrect(heading);
            }
            cy.get("@input").clear();
          });

          tests.forEach((test) => test());
        });
      });
    }
  });
});
