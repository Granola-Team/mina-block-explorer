import { ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION } from "../constants";

suite(["@tier1"], "Account spotlight", () => {
  let expected_fields = ["Balance", "Delegate"];

  it("displays complete information", () => {
    cy.visit(`/addresses/accounts/${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}`);
    cy.testSpotlight(
      "Account Spotlight",
      ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION,
      expected_fields,
    );
    cy.tableColumnValuesEqual(
      "SNARK Jobs",
      "Prover",
      ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION,
    );
    cy.tableColumnValuesEqual(
      "Block Production",
      "Block Producer",
      ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION,
    );
  });
});
