// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --

import { parseFormattedNumber } from "../e2e/helpers";

Cypress.Commands.add("assertAnalyticsSimplValueEquals", (label, value) => {
  cy.get("span")
    .contains(label)
    .parent()
    .parent()
    .siblings()
    .should("have.text", value);
});

Cypress.Commands.add(
  "assertSortOrder",
  (
    tableHeading,
    columnHeading,
    expectDesc,
    columnType = "numeric",
    limit = 5,
  ) => {
    let lastNumber = null;
    cy.assertForEachColumnValue(
      tableHeading,
      columnHeading,
      (text) => {
        switch (columnType) {
          case "numeric":
            let number = parseFormattedNumber(text);
            if (lastNumber != null) {
              if (expectDesc) {
                expect(number).to.be.lte(lastNumber);
              } else {
                expect(number).to.be.gte(lastNumber);
              }
            }
            lastNumber = number;
            break;
          default:
            throw Error(`Unrecognized column type: ${columnType}`);
        }
      },
      limit,
    );
    lastNumber = null;
  },
);

Cypress.Commands.add("assertStandardRowLimits", (tableHeading) => {
  [1000, 500, 250, 100, 50, 25].forEach((l) => {
    cy.assertRowLimitWorks(tableHeading, l);
  });
});

Cypress.Commands.add("assertRowLimitWorks", (tableHeading, limit) => {
  cy.get("select#row-limit")
    .select("" + limit)
    .should("have.value", "" + limit);
  cy.aliasTableRows(tableHeading, "table-rows");
  cy.get("@table-rows").should("have.lengthOf", limit);
});

// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })
Cypress.Commands.add(
  "aliasTableRows",
  (tableHeading, alias, tableHeaderEl = "h1") => {
    cy.get("section")
      .find(tableHeaderEl)
      .contains(tableHeading)
      .parents("section")
      .find("table tr:not(:has(th))")
      .as(alias);
  },
);

Cypress.Commands.add(
  "aliasTransposedTableRows",
  (tableHeading, alias, tableHeaderEl = "h1") => {
    cy.get("section")
      .find(tableHeaderEl)
      .contains(tableHeading)
      .parents("section")
      .find("table tr:has(th)")
      .as(alias);
  },
);

Cypress.Commands.add(
  "aliasTableHeaders",
  (tableHeading, alias, tableHeadingEl = "h1") => {
    cy.get("section")
      .find(tableHeadingEl)
      .contains(tableHeading)
      .parents("section")
      .find("table:first th")
      .as(alias);
  },
);

Cypress.Commands.add("closeAccountDialog", () => {
  cy.get("dialog button#closedialog a").click();
  cy.get("dialog").should("not.exist");
});

Cypress.Commands.add(
  "openAccountDialog",
  (nthRow, columnHeading, tableHeading) => {
    cy.clickLinkInTable(nthRow, columnHeading, tableHeading);
    cy.get("dialog").should("be.visible");
  },
);

Cypress.Commands.add("openMobileMenu", () => {
  cy.get("nav").should("not.be.visible");
  cy.get('label[for="nav-toggle"]').click();
  cy.get("nav").should("be.visible");
});

Cypress.Commands.add(
  "tableHasOrderedColumns",
  (tableHeading, columns, tableHeaderEl = "h1") => {
    cy.aliasTableHeaders(tableHeading, "columns", tableHeaderEl);
    cy.get("@columns").should("have.length", columns.length);
    columns.forEach((col, i) => {
      cy.get("@columns").eq(i).contains(col);
    });
  },
);

Cypress.Commands.add(
  "clickLinkInTransposedTable",
  (columnHeading, tableHeading, tableHeadingEl = "h1") => {
    cy.aliasTransposedTableRows(tableHeading, "table-rows", tableHeadingEl);
    cy.get("@table-rows")
      .contains(columnHeading)
      .siblings("td")
      .first()
      .find("a")
      .click({ force: true });
  },
);

Cypress.Commands.add(
  "assertLoadNextWorks",
  (
    tableHeading,
    column,
    opts = { button_text: "Load Next", expected_button_state: "be.enabled" },
  ) => {
    cy.aliasTableRows(tableHeading, "table-rows");
    cy.aliasTableHeaders(tableHeading, "columns");
    let last_height = null;
    cy.get("@columns")
      .contains(column)
      .invoke("index")
      .then((columnIndex) => {
        cy.get("@table-rows")
          .last()
          .find("td")
          .eq(columnIndex)
          .invoke("text")
          .then((text) => {
            cy.log("Last value: ", text);
            last_height = text;
          });
      });
    cy.intercept("POST", "/graphql").as("graphql");
    cy.contains(opts.button_text).click();
    cy.wait("@graphql").then(() => {
      cy.wait(1000);
      cy.contains(opts.button_text).should(opts.expected_button_state);
      cy.get("@columns")
        .contains(column)
        .invoke("index")
        .then((columnIndex) => {
          cy.get("@table-rows")
            .first()
            .find("td")
            .eq(columnIndex)
            .invoke("text")
            .then((text) => {
              cy.log("This value: ", text);
              expect(last_height).to.not.eq(null);
              expect(text).to.eq(last_height);
            });
        });
    });
  },
);

Cypress.Commands.add(
  "clickLinkInTable",
  (nthRow, columnHeading, tableHeading, tableHeadingEl = "h1") => {
    cy.aliasTableHeaders(tableHeading, "columns", tableHeadingEl);
    cy.get("@columns")
      .contains(columnHeading)
      .invoke("index")
      .then((columnIndex) => {
        cy.aliasTableRows(tableHeading, "table-rows", tableHeadingEl);
        cy.get("@table-rows")
          .eq(nthRow)
          .find("td")
          .eq(columnIndex)
          .find("a")
          .click({ force: true });
      });
  },
);

Cypress.Commands.add("tableHasMoreThanNRows", (tableHeading, n) => {
  cy.aliasTableRows(tableHeading, "table-rows");
  cy.get(`@table-rows`).should(($tr) => {
    expect($tr).to.have.length.of.at.least(n);
  });
});

Cypress.Commands.add("testSpotlight", (heading, id, expected_fields) => {
  cy.get("section#spotlight-section h1").contains(heading);
  cy.get("#spotlight-id").contains(id);
  cy.get("section#spotlight-section table").within(() => {
    expected_fields.forEach((field) => {
      cy.get("th").contains(field);
    });
  });
});

Cypress.Commands.add("testSpotlightValue", (key, value) => {
  cy.get("section#spotlight-section table").within(() => {
    cy.get("th").contains(key).parents("tr").find("td").contains(value);
  });
});

Cypress.Commands.add(
  "aliasTableMetadata",
  (heading, alias = "table-metadata") => {
    cy.aliasTableRows(heading, "table-rows");
    cy.get("@table-rows").then(($rows) => {
      cy.wrap($rows).parents("section").find(".metadata").as(alias);
    });
  },
);

Cypress.Commands.add("assertTableMetadataCorrect", (heading, metadata, ith) => {
  cy.aliasTableMetadata(heading, "table-metadata");
  cy.get("@table-metadata")
    .invoke("text")
    .then((text) => {
      text = text.replace(/\+/g, ""); // remove + symbol
      let parsed_metadata = text.split(" of ");
      expect(parseFormattedNumber(parsed_metadata[ith])).to.equal(metadata);
    });
});

Cypress.Commands.add(
  "assertNumberOfTableMetadataDatum",
  (heading, expected_number_of_metadata) => {
    cy.aliasTableMetadata(heading, "table-metadata");
    cy.get("@table-metadata")
      .invoke("text")
      .then((text) => {
        text = text.replace(/\+/g, ""); // remove + symbol
        let parsed_metadata = text.split(" of ");
        expect(parsed_metadata.length).to.equal(expected_number_of_metadata);
      });
  },
);

Cypress.Commands.add("assertTableRecordsCorrect", (heading) => {
  cy.aliasTableMetadata(heading, "table-metadata");
  cy.aliasTableRows(heading, "table-rows");
  cy.get("@table-rows").then(($rows) => {
    cy.get("@table-metadata")
      .invoke("text")
      .then((text) => {
        let [displaying, available, total] = text.split(" of ");
        displaying = displaying.replace(/\+/g, "");
        if (total == null) {
          total = available;
          displaying = parseFormattedNumber(displaying);
          total = parseFormattedNumber(total);
          expect(displaying).to.be.lte(total);
        } else {
          displaying = parseFormattedNumber(displaying);
          available = parseFormattedNumber(available);
          total = parseFormattedNumber(total);
          expect(displaying).to.be.lte(available);
          expect(available).to.be.lte(total);
        }
        expect(displaying).to.eq($rows.length);
      });
  });
});

Cypress.Commands.add(
  "assertForEachColumnValue",
  (heading, column, assertion, limit = 5) => {
    cy.aliasTableHeaders(heading, "columns");
    cy.get("@columns")
      .contains(column)
      .invoke("index")
      .then((columnIndex) => {
        cy.aliasTableRows(heading, "table-rows");
        cy.get("@table-rows").find(".loading-placeholder").should("not.exist");
        cy.get("@table-rows").should("have.length.gte", 1);
        cy.get("@table-rows").each(($row, index) => {
          if (index > limit) return;
          cy.wrap($row)
            .find("td")
            .eq(columnIndex)
            .invoke("text")
            .then(assertion);
        });
      });
  },
);

Cypress.Commands.add(
  "aliasTableColumnValue",
  (columnHeading, column, alias = "table-column-values") => {
    cy.aliasTableHeaders(columnHeading, "columns");
    cy.get("@columns")
      .contains(column)
      .invoke("index")
      .then((columnIndex) => {
        cy.aliasTableRows(columnHeading, "table-rows");
        cy.get("@table-rows").find("td").eq(columnIndex).as(alias);
      });
  },
);

Cypress.Commands.add(
  "tableColumnValuesEqual",
  (columnHeading, column, value) => {
    cy.aliasTableColumnValue(columnHeading, column, "table-column-values");
    cy.get("@table-column-values").should("contain", value);
  },
);
